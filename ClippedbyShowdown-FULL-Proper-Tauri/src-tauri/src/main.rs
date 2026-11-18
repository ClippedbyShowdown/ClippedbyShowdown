#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
struct ClipInput { path: String, start: f64, end: f64 }
#[derive(Serialize, Deserialize)]
struct ExportRequest { title: String, category: String, clips: Vec<String>, clip_files: Vec<ClipInput>, music: String, caption: String, quality: String, crop: String, speed: f64 }

#[tauri::command]
async fn save_to_downloads(filename: String, content: String) -> Result<String, String> {
    let dl = dirs::download_dir().or_else(|| std::env::var("USERPROFILE").ok().map(|p| PathBuf::from(p).join("Downloads"))).ok_or("No Downloads folder")?;
    let folder = dl.join("ClippedbyShowdown");
    std::fs::create_dir_all(&folder).map_err(|e| e.to_string())?;
    let path = folder.join(filename);
    std::fs::write(&path, content).map_err(|e| e.to_string())?;
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
async fn pick_video_file() -> Result<Option<String>, String> {
    // Tauri dialog is handled in frontend via @tauri-apps/api dialog, this is fallback
    Ok(None)
}

#[tauri::command]
async fn export_video(req: ExportRequest) -> Result<String, String> {
    let dl = dirs::download_dir().ok_or("No Downloads")?;
    let folder = dl.join("ClippedbyShowdown");
    std::fs::create_dir_all(&folder).map_err(|e| e.to_string())?;
    let safe: String = req.title.chars().map(|c| if c.is_alphanumeric() { c } else { '_' }).collect();
    let json_path = folder.join(format!("{}-edit.json", safe));
    let txt = serde_json::to_string_pretty(&req).map_err(|e| e.to_string())?;
    std::fs::write(&json_path, &txt).map_err(|e| e.to_string())?;

    // If no real clip files, fall back to edit JSON (still real file)
    if req.clip_files.is_empty() {
        return Ok(format!("{} (no source video files - add real clips via Import to render mp4, edit JSON saved)", json_path.to_string_lossy()));
    }

    // REAL FFmpeg render: trim each input, scale to quality, concat, draw caption, set speed
    let (w,h) = match req.quality.as_str() { "4K" => (3840,2160), "1080p" => (1920,1080), _ => (1280,720) };
    let (cw,ch) = if req.crop.contains("9:16") { (720,1280) } else if req.crop.contains("1:1") { (720,720) } else { (w,h) };
    let mp4 = folder.join(format!("{}.mp4", safe));
    let concat_list = folder.join(format!("{}-concat.txt", safe));
    let mut concat_content = String::new();
    let mut temp_files: Vec<PathBuf> = vec![];

    for (i, clip) in req.clip_files.iter().enumerate() {
        let tmp = folder.join(format!("{}-part{}.mp4", safe, i));
        let dur = (clip.end - clip.start).max(0.5);
        // Trim + scale + crop + speed + caption drawtext
        let vf = format!("trim=start={}:duration={},setpts={}*PTS,scale={}:{},crop={}:{},drawtext=text='{}':fontsize=48:fontcolor=white:borderw=2:bordercolor=black:x=(w-text_w)/2:y=h-80", clip.start, dur, 1.0/req.speed.max(0.25), w, h, cw, ch, req.caption.replace('\'',""));
        let out = Command::new("ffmpeg").args(["-y","-ss", &clip.start.to_string(), "-i", &clip.path, "-t", &dur.to_string(), "-vf", &vf, "-af", &format!("atempo={}", req.speed.max(0.25)), "-c:v","libx264","-preset","veryfast","-c:a","aac", tmp.to_str().unwrap()]).output();
        if let Ok(o) = out { if !o.status.success() { return Err(format!("FFmpeg trim failed for clip {}: {}", i, String::from_utf8_lossy(&o.stderr))); } } else { return Err("FFmpeg not found - install ffmpeg (choco install ffmpeg) for mp4 render, edit JSON saved".into()); }
        temp_files.push(tmp.clone());
        concat_content.push_str(&format!("file '{}'\n", tmp.to_string_lossy().replace('\'', "'\\''")));
    }
    std::fs::write(&concat_list, concat_content).map_err(|e| e.to_string())?;
    let out2 = Command::new("ffmpeg").args(["-y","-f","concat","-safe","0","-i", concat_list.to_str().unwrap(), "-c","copy", mp4.to_str().unwrap()]).output();
    // Cleanup temp
    for f in temp_files { let _ = std::fs::remove_file(f); }
    let _ = std::fs::remove_file(concat_list);
    match out2 { Ok(o) if o.status.success() => Ok(mp4.to_string_lossy().to_string()), Ok(o) => Err(format!("FFmpeg concat failed: {}", String::from_utf8_lossy(&o.stderr))), Err(e) => Err(e.to_string()) }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_to_downloads, export_video, pick_video_file])
        .run(tauri::generate_context!()).expect("error while running tauri application");
}
