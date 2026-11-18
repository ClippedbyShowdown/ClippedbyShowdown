# ClippedbyShowdown 1.0.1 FULL

## Real FFmpeg render
Export now takes your imported .mp4s + timeline (trim start/end, speed, caption, crop, quality) and renders real mp4 via ffmpeg: trim -> scale -> crop -> drawtext caption -> atempo -> concat.
Install ffmpeg on Windows: choco install ffmpeg
If no source files, Export saves edit JSON (still real file).

## Real Import preview
Import Clip in .exe uses Tauri dialog.open + convertFileSrc -> <video> preview shows YOUR file, not demo. In browser preview, uses file input + Blob URL.

## Build
Push to GitHub -> Actions Build Windows -> download ClippedbyShowdown-Windows-Setup
