GITHUB ACTIONS - HOW TO GET YOUR .EXE WITHOUT INSTALLING ANYTHING

1. Create a free GitHub account at github.com
2. Create a new repo (public = free unlimited Windows builds)
3. Upload ALL files from this ZIP to that repo
4. In the repo, create folder: .github/workflows/
5. Copy GITHUB-WORKFLOW-build-windows.yml into that folder and rename it to: build-windows.yml
   (GitHub does not allow uploading a .github folder from a ZIP easily, so this file is provided at top level)
6. Push / commit - GitHub will build automatically
7. Go to Actions tab > click the run > Artifacts > download ClippedbyShowdown-Windows
8. Inside: ClippedbyShowdown.exe + .msi + -setup.exe installer
