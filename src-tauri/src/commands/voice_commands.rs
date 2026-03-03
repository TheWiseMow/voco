use serde::{Deserialize, Serialize};
use std::process::Command;
use std::path::PathBuf;
use tauri::command;
use log::{info, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceCommand {
    pub intent: CommandIntent,
    pub raw_text: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CommandIntent {
    Build,
    Test,
    Run,
    Dev,
    Commit,
    Push,
    Pull,
    Branch,
    Status,
    NewFile,
    OpenFile,
    Search,
    Explain,
    Errors,
    Help,
    Unknown,
}

impl CommandIntent {
    pub fn as_str(&self) -> &'static str {
        match self {
            CommandIntent::Build => "build",
            CommandIntent::Test => "test",
            CommandIntent::Run => "run",
            CommandIntent::Dev => "dev",
            CommandIntent::Commit => "commit",
            CommandIntent::Push => "push",
            CommandIntent::Pull => "pull",
            CommandIntent::Branch => "branch",
            CommandIntent::Status => "status",
            CommandIntent::NewFile => "new_file",
            CommandIntent::OpenFile => "open_file",
            CommandIntent::Search => "search",
            CommandIntent::Explain => "explain",
            CommandIntent::Errors => "errors",
            CommandIntent::Help => "help",
            CommandIntent::Unknown => "unknown",
        }
    }
}

impl VoiceCommand {
    pub fn parse(text: &str) -> Self {
        let text_lower = text.to_lowercase().trim().to_string();
        let words: Vec<&str> = text_lower.split_whitespace().collect();
        
        let (intent, args) = match words.get(0) {
            // Build commands
            Some(&"build") => (CommandIntent::Build, words[1..].to_vec()),
            Some(&"compile") => (CommandIntent::Build, words[1..].to_vec()),
            Some(&"rebuild") => (CommandIntent::Build, vec!["--force"]),
            
            // Test commands
            Some(&"test") | Some(&"tests") | Some(&"testing") => (CommandIntent::Test, words[1..].to_vec()),
            
            // Run commands
            Some(&"run") | Some(&"execute") => (CommandIntent::Run, words[1..].to_vec()),
            Some(&"start") | Some(&"serve") | Some(&"dev") => (CommandIntent::Dev, words[1..].to_vec()),
            
            // Git commands
            Some(&"commit") | Some(&"save") => (CommandIntent::Commit, words[1..].to_vec()),
            Some(&"push") | Some(&"upload") => (CommandIntent::Push, words[1..].to_vec()),
            Some(&"pull") | Some(&"download") | Some(&"sync") => (CommandIntent::Pull, words[1..].to_vec()),
            Some(&"branch") | Some(&"branches") => (CommandIntent::Branch, words[1..].to_vec()),
            Some(&"status") | Some(&"stat") => (CommandIntent::Status, vec![]),
            Some(&"git") => {
                match words.get(1) {
                    Some(&"commit") => (CommandIntent::Commit, words[2..].to_vec()),
                    Some(&"push") => (CommandIntent::Push, words[2..].to_vec()),
                    Some(&"pull") => (CommandIntent::Pull, words[2..].to_vec()),
                    Some(&"branch") => (CommandIntent::Branch, words[2..].to_vec()),
                    Some(&"status") | Some(&"stat") => (CommandIntent::Status, vec![]),
                    _ => (CommandIntent::Unknown, words[1..].to_vec()),
                }
            }
            
            // File commands
            Some(&"new") | Some(&"create") | Some(&"make") => {
                if words.get(1) == Some(&"file") || words.get(1) == Some(&"folder") || words.get(1) == Some(&"dir") {
                    (CommandIntent::NewFile, words[2..].to_vec())
                } else {
                    (CommandIntent::NewFile, words[1..].to_vec())
                }
            }
            Some(&"open") | Some(&"edit") => (CommandIntent::OpenFile, words[1..].to_vec()),
            
            // Search commands
            Some(&"search") | Some(&"find") | Some(&"grep") | Some(&"look") => (CommandIntent::Search, words[1..].to_vec()),
            
            // Code understanding
            Some(&"explain") | Some(&"describe") | Some(&"what") => (CommandIntent::Explain, words[1..].to_vec()),
            Some(&"errors") | Some(&"error") | Some(&"bugs") | Some(&"issues") => (CommandIntent::Errors, words[1..].to_vec()),
            
            // Help
            Some(&"help") | Some(&"commands") | Some(&"what") | Some(&"list") => (CommandIntent::Help, vec![]),
            
            _ => (CommandIntent::Unknown, words.to_vec()),
        };

        VoiceCommand {
            intent,
            raw_text: text.to_string(),
            args: args.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn is_command(&self) -> bool {
        self.intent != CommandIntent::Unknown
    }

    /// Check if this looks like a command vs regular speech
    pub fn is_likely_command(&self) -> bool {
        // Short phrases with action words are likely commands
        let action_words = ["build", "test", "run", "commit", "push", "pull", "branch", 
                          "new", "open", "search", "find", "help", "start", "stop"];
        let first_word = self.raw_text.to_lowercase().split_whitespace().next();
        first_word.map(|w| action_words.contains(&w)).unwrap_or(false)
    }
}

pub struct CommandExecutor {
    pub working_directory: Option<String>,
}

impl CommandExecutor {
    pub fn new(working_directory: Option<String>) -> Self {
        Self { working_directory }
    }

    pub fn execute(&self, command: &VoiceCommand, project_commands: &ProjectCommands) -> CommandResult {
        info!("Executing command: {:?} with args: {:?}", command.intent, command.args);
        
        match command.intent {
            CommandIntent::Build => {
                let mut cmd = project_commands.build_command.clone();
                if command.args.contains(&"--force".to_string()) || command.args.contains(&"-f".to_string()) {
                    cmd.push_str(" --force");
                }
                self.run_shell(&cmd)
            }
            CommandIntent::Test => {
                match &project_commands.test_command {
                    Some(test_cmd) => {
                        let mut cmd = test_cmd.clone();
                        if command.args.contains(&"--watch".to_string()) || command.args.contains(&"-w".to_string()) {
                            cmd.push_str(" --watch");
                        }
                        self.run_shell(&cmd)
                    }
                    None => CommandResult::error("No test command configured for this project"),
                }
            }
            CommandIntent::Run | CommandIntent::Dev => {
                match &project_commands.run_command {
                    Some(run_cmd) => self.run_shell(run_cmd),
                    None => CommandResult::error("No run command configured for this project. Try 'build' instead."),
                }
            }
            CommandIntent::Commit => {
                let msg = if command.args.is_empty() {
                    "Voice commit".to_string()
                } else {
                    command.args.join(" ")
                };
                self.run_shell(&format!("git add -A && git commit -m \"{}\"", msg))
            }
            CommandIntent::Push => {
                let mut cmd = String::from("git push");
                if command.args.contains(&"--force".to_string()) || command.args.contains(&"-f".to_string()) {
                    cmd.push_str(" --force");
                }
                if command.args.contains(&"--force-with-lease".to_string()) {
                    cmd.push_str(" --force-with-lease");
                }
                self.run_shell(&cmd)
            }
            CommandIntent::Pull => {
                self.run_shell("git pull")
            }
            CommandIntent::Branch => {
                if command.args.is_empty() {
                    self.run_shell("git branch -a")
                } else if command.args.contains(&"-d".to_string()) || command.args.contains(&"--delete".to_string()) {
                    // Delete branch
                    let branch_name = command.args.iter()
                        .skip_while(|a| *a == "-d" || *a == "--delete")
                        .next()
                        .unwrap_or("");
                    if !branch_name.is_empty() {
                        self.run_shell(&format!("git branch -d {}", branch_name))
                    } else {
                        CommandResult::error("Please specify branch name to delete")
                    }
                } else if command.args.contains(&"-m".to_string()) || command.args.contains(&"--move".to_string()) {
                    // Rename branch
                    let parts: Vec<&str> = command.args.iter()
                        .skip_while(|a| *a == "-m" || *a == "--move")
                        .collect();
                    if parts.len() >= 2 {
                        self.run_shell(&format!("git branch -m {} {}", parts[0], parts[1]))
                    } else {
                        CommandResult::error("Please specify current and new branch name")
                    }
                } else {
                    // Create/switch to branch
                    let branch_name = command.args.join(" ");
                    self.run_shell(&format!("git checkout -b {} 2>/dev/null || git checkout {}", branch_name, branch_name))
                }
            }
            CommandIntent::Status => {
                self.run_shell("git status --short")
            }
            CommandIntent::NewFile => {
                if command.args.is_empty() {
                    CommandResult::error("Please specify a filename. Example: 'new file src/index.ts'")
                } else {
                    let filename = command.args.join(" ");
                    self.run_shell(&format!("touch \"{}\"", filename))
                }
            }
            CommandIntent::OpenFile => {
                if command.args.is_empty() {
                    CommandResult::error("Please specify a filename. Example: 'open src/App.tsx'")
                } else {
                    let filename = command.args.join(" ");
                    // Try VS Code, then VS Code Insiders, then system default
                    self.run_shell(&format!(
                        "code \"{}\" 2>/dev/null || code-insiders \"{}\" 2>/dev/null || nano \"{}\" 2>/dev/null || echo \"File: {}\"",
                        filename, filename, filename, filename
                    ))
                }
            }
            CommandIntent::Search => {
                if command.args.is_empty() {
                    CommandResult::error("Please specify what to search for. Example: 'search TODO'")
                } else {
                    let query = command.args.join(" ");
                    // Search in common code files
                    self.run_shell(&format!(
                        "grep -r \"{}\" . --include=\"*.rs\" --include=\"*.ts\" --include=\"*.tsx\" --include=\"*.js\" --include=\"*.jsx\" --include=\"*.py\" --include=\"*.go\" --include=\"*.java\" -l -n 2>/dev/null | head -30",
                        query
                    ))
                }
            }
            CommandIntent::Explain => {
                // This would need AI - return helpful message
                let target = if command.args.is_empty() {
                    String::from("the current file")
                } else {
                    command.args.join(" ")
                };
                CommandResult::success(&format!(
                    "To explain code, use Claude Code:\n  claude --print << 'EOF'\nExplain this code: {}\nEOF",
                    target
                ))
            }
            CommandIntent::Errors => {
                // Check for common error patterns in logs and code
                self.run_shell("grep -r \"ERROR\\|error:\\|Error\\|failed\\|Failed\\|exception\\|Exception\" . --include=\"*.log\" -n 2>/dev/null | head -20; grep -r \"TODO\\|FIXME\\|XXX\\|HACK\" . --include=\"*.rs\" --include=\"*.ts\" --include=\"*.tsx\" --include=\"*.js\" -n 2>/dev/null | head -10")
            }
            CommandIntent::Help => {
                CommandResult::success(Self::help_text())
            }
            CommandIntent::Unknown => {
                if command.args.is_empty() {
                    CommandResult::error("Unknown command. Say 'help' for available commands.")
                } else {
                    // Try as shell command
                    self.run_shell(&command.args.join(" "))
                }
            }
        }
    }

    fn run_shell(&self, cmd: &str) -> CommandResult {
        info!("Running: {}", cmd);
        
        let output = if let Some(ref dir) = self.working_directory {
            Command::new("sh")
                .args(["-c", cmd])
                .current_dir(dir)
                .output()
        } else {
            Command::new("sh")
                .args(["-c", cmd])
                .output()
        };

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                
                if output.status.success() {
                    if stdout.is_empty() {
                        CommandResult::success("Command completed successfully (no output)")
                    } else {
                        CommandResult::success(&stdout)
                    }
                } else {
                    if !stderr.is_empty() {
                        CommandResult::error(&stderr)
                    } else if !stdout.is_empty() {
                        CommandResult::error(&stdout)
                    } else {
                        CommandResult::error("Command failed with no output")
                    }
                }
            }
            Err(e) => {
                error!("Command failed: {}", e);
                CommandResult::error(&format!("Failed to execute: {}", e))
            }
        }
    }

    fn help_text() -> String {
        r#"🎤 Voice Commands Available:

📦 Build & Run:
• "build" - Build your project
• "test" - Run tests  
• "run" / "start" - Run dev server

🔧 Git:
• "commit [msg]" - Commit changes
• "push" - Push to remote
• "pull" - Pull from remote
• "branch" - List branches
• "branch [name]" - Create/switch branch
• "status" - Show git status

📁 Files:
• "new file [name]" - Create file
• "open [file]" - Open in editor
• "search [query]" - Search code

🔍 Code:
• "errors" - Find errors/TODOs
• "explain [file]" - Explain code (AI)

💡 Tips:
• Works with: Node.js, Rust, .NET, Go, Python, Java, Ruby, PHP
• Say "help" anytime for this list
"#.to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectCommands {
    pub build_command: String,
    pub test_command: Option<String>,
    pub run_command: Option<String>,
}

impl Default for ProjectCommands {
    fn default() -> Self {
        Self {
            build_command: "echo 'No project detected. Navigate to a project folder first.'".to_string(),
            test_command: None,
            run_command: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub success: bool,
    pub message: String,
}

impl CommandResult {
    pub fn success(msg: &str) -> Self {
        Self {
            success: true,
            message: msg.to_string(),
        }
    }

    pub fn error(msg: &str) -> Self {
        Self {
            success: false,
            message: msg.to_string(),
        }
    }
}

// ============ Tauri Commands ============

#[command]
pub fn parse_voice_command(text: &str) -> VoiceCommand {
    VoiceCommand::parse(text)
}

#[command]
pub fn detect_project_type(path: &str) -> Option<ProjectCommands> {
    let path = PathBuf::from(path);
    if let Some(project) = crate::managers::project::detect_project(path) {
        Some(ProjectCommands {
            build_command: project.build_command,
            test_command: project.test_command,
            run_command: project.run_command,
        })
    } else {
        None
    }
}

#[command]
pub fn execute_voice_command(text: &str, working_dir: Option<String>) -> CommandResult {
    let command = VoiceCommand::parse(text);
    
    let project_commands = if let Some(ref dir) = working_dir {
        let path = PathBuf::from(dir);
        if let Some(project) = crate::managers::project::detect_project(path) {
            ProjectCommands {
                build_command: project.build_command,
                test_command: project.test_command,
                run_command: project.run_command,
            }
        } else {
            ProjectCommands::default()
        }
    } else {
        ProjectCommands::default()
    };
    
    let executor = CommandExecutor::new(working_dir);
    executor.execute(&command, &project_commands)
}

#[command]
pub fn get_command_help() -> String {
    CommandExecutor::help_text()
}

#[command]
pub fn get_supported_intents() -> Vec<String> {
    vec![
        "build".to_string(),
        "test".to_string(),
        "run".to_string(),
        "start".to_string(),
        "dev".to_string(),
        "commit".to_string(),
        "push".to_string(),
        "pull".to_string(),
        "branch".to_string(),
        "status".to_string(),
        "new file".to_string(),
        "open".to_string(),
        "search".to_string(),
        "find".to_string(),
        "explain".to_string(),
        "errors".to_string(),
        "help".to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_build_command() {
        let cmd = VoiceCommand::parse("build");
        assert_eq!(cmd.intent, CommandIntent::Build);
        assert!(cmd.is_command());
    }

    #[test]
    fn test_parse_rebuild() {
        let cmd = VoiceCommand::parse("rebuild");
        assert_eq!(cmd.intent, CommandIntent::Build);
        assert!(cmd.args.contains(&"--force".to_string()));
    }

    #[test]
    fn test_parse_git_commit() {
        let cmd = VoiceCommand::parse("commit fix bug in auth");
        assert_eq!(cmd.intent, CommandIntent::Commit);
        assert_eq!(cmd.args, vec!["fix", "bug", "in", "auth"]);
    }

    #[test]
    fn test_parse_git_push() {
        let cmd = VoiceCommand::parse("push --force");
        assert_eq!(cmd.intent, CommandIntent::Push);
        assert!(cmd.args.contains(&"--force".to_string()));
    }

    #[test]
    fn test_parse_test() {
        let cmd = VoiceCommand::parse("test --watch");
        assert_eq!(cmd.intent, CommandIntent::Test);
    }

    #[test]
    fn test_parse_run() {
        let cmd = VoiceCommand::parse("run server");
        assert_eq!(cmd.intent, CommandIntent::Run);
        assert_eq!(cmd.args, vec!["server"]);
    }

    #[test]
    fn test_parse_new_file() {
        let cmd = VoiceCommand::parse("new file src/components/Button.tsx");
        assert_eq!(cmd.intent, CommandIntent::NewFile);
        assert_eq!(cmd.args, vec!["src/components/Button.tsx"]);
    }

    #[test]
    fn test_parse_search() {
        let cmd = VoiceCommand::parse("search TODO");
        assert_eq!(cmd.intent, CommandIntent::Search);
        assert_eq!(cmd.args, vec!["TODO"]);
    }

    #[test]
    fn test_parse_help() {
        let cmd = VoiceCommand::parse("help");
        assert_eq!(cmd.intent, CommandIntent::Help);
    }

    #[test]
    fn test_parse_unknown() {
        let cmd = VoiceCommand::parse("hello world what is up");
        assert_eq!(cmd.intent, CommandIntent::Unknown);
        assert!(!cmd.is_command());
    }

    #[test]
    fn test_is_likely_command() {
        assert!(VoiceCommand::parse("build").is_likely_command());
        assert!(VoiceCommand::parse("commit fix").is_likely_command());
        assert!(VoiceCommand::parse("test").is_likely_command());
        assert!(!VoiceCommand::parse("hello world").is_likely_command());
    }

    #[test]
    fn test_intent_to_string() {
        assert_eq!(CommandIntent::Build.as_str(), "build");
        assert_eq!(CommandIntent::Commit.as_str(), "commit");
        assert_eq!(CommandIntent::Help.as_str(), "help");
    }
}
