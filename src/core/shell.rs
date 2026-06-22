// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use anyhow::Result;
use std::process::{Command, Output};

/// Executor de comandos do sistema
pub struct ShellExecutor;

impl ShellExecutor {
    /// Executa um comando shell e retorna a saída
    pub fn execute(command: &str, current_dir: &std::path::Path) -> Result<CommandOutput> {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", command])
                .current_dir(current_dir)
                .output()?
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(command)
                .current_dir(current_dir)
                .output()?
        };
        
        Ok(CommandOutput::from_output(output))
    }
}

/// Resultado da execução de um comando
#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
}

impl CommandOutput {
    fn from_output(output: Output) -> Self {
        Self {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            success: output.status.success(),
        }
    }
    
    /// Retorna a saída combinada (stdout + stderr)
    pub fn combined_output(&self) -> String {
        if self.stderr.is_empty() {
            self.stdout.clone()
        } else if self.stdout.is_empty() {
            self.stderr.clone()
        } else {
            format!("{}\n{}", self.stdout, self.stderr)
        }
    }
}
