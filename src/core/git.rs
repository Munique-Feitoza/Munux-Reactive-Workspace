// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use std::path::Path;
use std::process::Command;

/// Informações sobre o status do Git no diretório atual
#[derive(Debug, Clone)]
pub struct GitStatus {
    pub branch: String,
    pub repo_name: String,
    pub ahead: usize,
    pub behind: usize,
    pub staged: usize,
    pub modified: usize,
    pub untracked: usize,
}

pub struct GitManager;

impl GitManager {
    /// Detecta se o diretório faz parte de um repositório Git e extrai informações
    pub fn get_status(path: &Path) -> Option<GitStatus> {
        // Verifica se é um repositório git
        if !Command::new("git")
            .arg("-C")
            .arg(path)
            .arg("rev-parse")
            .arg("--is-inside-work-tree")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            return None;
        }

        // Obtém o nome da branch
        let branch = Command::new("git")
            .arg("-C")
            .arg(path)
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg("HEAD")
            .output()
            .ok()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        // Obtém status detalhado via porcelain
        let mut staged = 0;
        let mut modified = 0;
        let mut untracked = 0;
        
        if let Ok(output) = Command::new("git")
            .arg("-C")
            .arg(path)
            .arg("status")
            .arg("--porcelain")
            .output()
        {
            let status_text = String::from_utf8_lossy(&output.stdout);
            for line in status_text.lines() {
                if line.len() < 2 { continue; }
                
                let x = line.chars().nth(0).unwrap_or(' ');
                let y = line.chars().nth(1).unwrap_or(' ');
                
                // Untracked: ??
                if x == '?' && y == '?' {
                    untracked += 1;
                    continue;
                }
                
                // Staged (X)
                if x != ' ' {
                    staged += 1;
                }
                
                // Modified in worktree (Y)
                if y != ' ' {
                    modified += 1;
                }
            }
        }

        // Obtém o nome do repositório (último componente do path da raiz do repo)
        let repo_root_path = Command::new("git")
            .arg("-C")
            .arg(path)
            .arg("rev-parse")
            .arg("--show-toplevel")
            .output()
            .ok()
            .and_then(|o| String::from_utf8_lossy(&o.stdout).trim().parse::<std::path::PathBuf>().ok());

        let repo_name = repo_root_path
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
            .unwrap_or_else(|| "repo".to_string());

        // Verifica ahead/behind
        let mut ahead = 0;
        let mut behind = 0;
        
        // Primeiro verifica se existe um upstream configurado
        let has_upstream = Command::new("git")
            .arg("-C")
            .arg(path)
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg("--symbolic-full-name")
            .arg("@{u}")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        if has_upstream {
            if let Ok(output) = Command::new("git")
                .arg("-C")
                .arg(path)
                .arg("rev-list")
                .arg("--left-right")
                .arg("--count")
                .arg("HEAD...@{u}")
                .output()
            {
                let counts = String::from_utf8_lossy(&output.stdout);
                let parts: Vec<&str> = counts.split_whitespace().collect();
                if parts.len() == 2 {
                    ahead = parts[0].parse().unwrap_or(0);
                    behind = parts[1].parse().unwrap_or(0);
                }
            }
        }

        Some(GitStatus {
            branch,
            repo_name,
            ahead,
            behind,
            staged,
            modified,
            untracked,
        })
    }
}
