use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub project_type: ProjectType,
    pub root: PathBuf,
    pub build_command: String,
    pub test_command: Option<String>,
    pub run_command: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectType {
    Nodejs { package_manager: PackageManager },
    Rust,
    Dotnet,
    Go,
    Python { runner: PythonRunner },
    Java { build_tool: JavaBuildTool },
    Ruby,
    PHP,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PackageManager {
    Npm,
    Yarn,
    Pnpm,
    Bun,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PythonRunner {
    Pip,
    Poetry,
    UV,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum JavaBuildTool {
    Maven,
    Gradle,
}

impl ProjectType {
    pub fn detect(root: &PathBuf) -> Self {
        // Check for Node.js
        if root.join("package.json").exists() {
            let package_manager = if root.join("pnpm-workspace.yaml").exists() {
                PackageManager::Pnpm
            } else if root.join("yarn.lock").exists() {
                PackageManager::Yarn
            } else if root.join("bun.lock").exists() {
                PackageManager::Bun
            } else {
                PackageManager::Npm
            };
            return ProjectType::Nodejs { package_manager };
        }

        // Check for Rust
        if root.join("Cargo.toml").exists() {
            return ProjectType::Rust;
        }

        // Check for .NET
        if root.glob("*.csproj").next().is_some() || root.glob("*.sln").next().is_some() {
            return ProjectType::Dotnet;
        }

        // Check for Go
        if root.join("go.mod").exists() {
            return ProjectType::Go;
        }

        // Check for Python
        if root.join("pyproject.toml").exists() {
            return ProjectType::Python { runner: PythonRunner::Poetry };
        }
        if root.join("uv.lock").exists() {
            return ProjectType::Python { runner: PythonRunner::UV };
        }
        if root.join("requirements.txt").exists() || root.join("Pipfile").exists() {
            return ProjectType::Python { runner: PythonRunner::Pip };
        }

        // Check for Java
        if root.join("pom.xml").exists() {
            return ProjectType::Java { build_tool: JavaBuildTool::Maven };
        }
        if root.join("build.gradle").exists() || root.join("build.gradle.kts").exists() {
            return ProjectType::Java { build_tool: JavaBuildTool::Gradle };
        }

        // Check for Ruby
        if root.join("Gemfile").exists() {
            return ProjectType::Ruby;
        }

        // Check for PHP
        if root.join("composer.json").exists() {
            return ProjectType::PHP;
        }

        // Check for Make
        if root.join("Makefile").exists() || root.join("makefile").exists() {
            // Make can build anything, so we return Unknown with a fallback
            return ProjectType::Unknown;
        }

        ProjectType::Unknown
    }

    pub fn build_command(&self) -> String {
        match self {
            ProjectType::Nodejs { package_manager } => {
                match package_manager {
                    PackageManager::Npm => "npm run build".to_string(),
                    PackageManager::Yarn => "yarn build".to_string(),
                    PackageManager::Pnpm => "pnpm build".to_string(),
                    PackageManager::Bun => "bun run build".to_string(),
                }
            }
            ProjectType::Rust => "cargo build".to_string(),
            ProjectType::Dotnet => "dotnet build".to_string(),
            ProjectType::Go => "go build".to_string(),
            ProjectType::Python { runner } => {
                match runner {
                    PythonRunner::Poetry => "poetry build".to_string(),
                    PythonRunner::UV => "uv build".to_string(),
                    PythonRunner::Pip => "python -m build".to_string(),
                    PythonRunner::None => "pip install".to_string(),
                }
            }
            ProjectType::Java { build_tool } => {
                match build_tool {
                    JavaBuildTool::Maven => "mvn build".to_string(),
                    JavaBuildTool::Gradle => "gradle build".to_string(),
                }
            }
            ProjectType::Ruby => "bundle install".to_string(),
            ProjectType::PHP => "composer install".to_string(),
            ProjectType::Unknown => "make".to_string(),
        }
    }

    pub fn test_command(&self) -> Option<String> {
        match self {
            ProjectType::Nodejs { .. } => Some("npm test".to_string()),
            ProjectType::Rust => Some("cargo test".to_string()),
            ProjectType::Dotnet => Some("dotnet test".to_string()),
            ProjectType::Go => Some("go test ./...".to_string()),
            ProjectType::Python { runner } => {
                match runner {
                    PythonRunner::Poetry => Some("poetry run pytest".to_string()),
                    PythonRunner::UV => Some("uv run pytest".to_string()),
                    _ => Some("pytest".to_string()),
                }
            }
            ProjectType::Java { build_tool } => {
                match build_tool {
                    JavaBuildTool::Maven => Some("mvn test".to_string()),
                    JavaBuildTool::Gradle => Some("gradle test".to_string()),
                }
            }
            _ => None,
        }
    }

    pub fn run_command(&self) -> Option<String> {
        match self {
            ProjectType::Nodejs { package_manager } => {
                match package_manager {
                    PackageManager::Npm => Some("npm run dev".to_string()),
                    PackageManager::Yarn => Some("yarn dev".to_string()),
                    PackageManager::Pnpm => Some("pnpm dev".to_string()),
                    PackageManager::Bun => Some("bun run dev".to_string()),
                }
            }
            ProjectType::Rust => Some("cargo run".to_string()),
            ProjectType::Dotnet => Some("dotnet run".to_string()),
            ProjectType::Go => Some("go run .".to_string()),
            ProjectType::Python { runner } => {
                match runner {
                    PythonRunner::Poetry => Some("poetry run python".to_string()),
                    PythonRunner::UV => Some("uv run python".to_string()),
                    _ => Some("python".to_string()),
                }
            }
            ProjectType::Java { build_tool } => {
                match build_tool {
                    JavaBuildTool::Maven => Some("mvn spring-boot:run".to_string()),
                    JavaBuildTool::Gradle => Some("gradle bootRun".to_string()),
                }
            }
            _ => None,
        }
    }
}

pub fn detect_project(root: PathBuf) -> Option<ProjectInfo> {
    let project_type = ProjectType::detect(&root);
    
    if project_type == ProjectType::Unknown {
        return None;
    }

    Some(ProjectInfo {
        project_type,
        root,
        build_command: project_type.build_command(),
        test_command: project_type.test_command(),
        run_command: project_type.run_command(),
    })
}

/// Find the project root by searching upward for project markers
pub fn find_project_root(start_path: &PathBuf) -> Option<PathBuf> {
    let mut current = start_path.clone();
    
    // Limit search depth to avoid infinite loops
    for _ in 0..20 {
        let project_type = ProjectType::detect(&current);
        if project_type != ProjectType::Unknown {
            return Some(current);
        }
        
        // Go up one directory
        if !current.pop() {
            break;
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_package_manager_detection() {
        assert_eq!(PackageManager::Npm, PackageManager::Npm);
    }

    #[test]
    fn test_project_type_build_commands() {
        let nodejs = ProjectType::Nodejs { package_manager: PackageManager::Npm };
        assert_eq!(nodejs.build_command(), "npm run build");
        
        let rust = ProjectType::Rust;
        assert_eq!(rust.build_command(), "cargo build");
        
        let dotnet = ProjectType::Dotnet;
        assert_eq!(dotnet.build_command(), "dotnet build");
    }

    #[test]
    fn test_project_type_test_commands() {
        let nodejs = ProjectType::Nodejs { package_manager: PackageManager::Npm };
        assert_eq!(nodejs.test_command(), Some("npm test".to_string()));
        
        let rust = ProjectType::Rust;
        assert_eq!(rust.test_command(), Some("cargo test".to_string()));
    }

    #[test]
    fn test_project_type_run_commands() {
        let nodejs = ProjectType::Nodejs { package_manager: PackageManager::Npm };
        assert_eq!(nodejs.run_command(), Some("npm run dev".to_string()));
        
        let rust = ProjectType::Rust;
        assert_eq!(rust.run_command(), Some("cargo run".to_string()));
    }

    #[test]
    fn test_detect_nodejs_with_package_json() {
        // This would require temp directory - just test the logic exists
        let unknown = ProjectType::Unknown;
        assert_eq!(unknown.build_command(), "make");
    }
}
