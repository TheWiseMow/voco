# TheWiseMow/Voco

Fork of [cjpais/Handy](https://github.com/cjpais/Handy) with AI-powered voice commands for developers.

## What's Different

This fork adds:
- 🎯 **Voice Commands** - Speak commands like "build", "test", "commit"
- 🔍 **Project Auto-Detection** - Automatically detects Node.js, Rust, .NET, Go, Python projects
- ⚡ **Smart Execution** - Runs the correct command for your project type

## Voice Commands

### Build & Run
- "build" → Runs `npm run build` / `cargo build` / `dotnet build` / etc.
- "test" → Runs tests for your project
- "run" / "start" → Runs dev server

### Git Commands
- "commit [message]" → Git commit with your message
- "push" → Git push
- "pull" → Git pull
- "branch" → List branches
- "status" → Git status

### File Commands
- "new file [name]" → Create new file
- "open [file]" → Open in VS Code
- "search [query]" → Search in code

## Setup

```bash
# Clone
git clone https://github.com/TheWiseMow/Voco.git
cd Voco

# Install deps
bun install

# Download models
mkdir -p src-tauri/resources/models
curl -o src-tauri/resources/models/silero_vad_v4.onnx https://blob.Voco.computer/silero_vad_v4.onnx

# Run dev
bun run tauri dev
```

## Status

🚧 **Under Development** - Not ready for production yet.

### To Do
- [ ] Push code to GitHub (needs repo creation)
- [ ] Add AI command interpretation
- [ ] Add settings UI for voice commands
- [ ] Test on Windows

---

*Original README at README.md.original*
