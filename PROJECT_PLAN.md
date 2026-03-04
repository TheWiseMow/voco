# TheWiseMow/Voco - Voice Commands Feature

## Overview
Fork of cjpais/Voco with AI-powered voice commands for hands-free development.

## Features to Add

### 1. Voice Commands
- Detect when user speaks a command vs regular transcription
- AI interprets intent from transcribed text
- Execute actions based on commands

### 2. Command Categories
- **Build commands**: "build", "test", "run", "start"
- **Git commands**: "commit", "push", "pull", "branch"
- **File commands**: "new file", "open file", "search"
- **System commands**: "explain code", "what errors"

### 3. Project Detection
Auto-detect project type:
- package.json → npm/yarn/pnpm
- Cargo.toml → cargo
- *.csproj → dotnet
- go.mod → go
- Makefile → make
- pom.xml/gradle → mvn/gradle

### 4. AI Integration
- Use existing LLM client infrastructure
- Add command interpretation prompt
- Support local (Ollama) or cloud APIs

## Architecture

### New Files
- `src-tauri/src/commands/voice_commands.rs` - Command parsing & execution
- `src-tauri/src/managers/project.rs` - Project detection
- `src-tauri/src/command_registry.rs` - Command definitions

### Modified Files
- `src-tauri/src/settings.rs` - Add voice command settings
- `src-tauri/src/lib.rs` - Register new managers
- `src/components/settings/` - Add voice commands settings UI

## Progress
- [x] Clone repo
- [ ] Create GitHub repo (token issues - needs manual)
- [x] Add project detection (src-tauri/src/managers/project.rs)
- [x] Add voice command parsing (src-tauri/src/commands/voice_commands.rs)
- [x] Add command execution integration (lib.rs)
- [x] Add comprehensive tests (12+ tests)
- [ ] Test on Windows
- [ ] Add AI command interpretation (optional)
- [ ] Add settings UI

## TaskRunner (Second Project)
- [x] Created Rust CLI project at /data/.openclaw/workspace/taskrunner/
- [x] Project detection module
- [x] Command parsing & execution
- [ ] Add hotkey support (would need Tauri or platform-specific code)
- [ ] Add voice input integration

## GitHub
- Repo: TheWiseMow/Voco
- Need: Don to create repo or fix token permissions
