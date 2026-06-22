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
    /// Roda `git -C <path> <args...>` e devolve o stdout (sem espaços nas pontas)
    /// quando o comando tem sucesso. Fonte única — antes o boilerplate
    /// `Command::new("git").arg("-C")...` se repetia 6 vezes.
    fn git(path: &Path, args: &[&str]) -> Option<String> {
        let output = Command::new("git").arg("-C").arg(path).args(args).output().ok()?;
        if output.status.success() {
            Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            None
        }
    }

    /// Detecta se o diretório faz parte de um repositório Git e extrai informações
    pub fn get_status(path: &Path) -> Option<GitStatus> {
        // Verifica se é um repositório git
        Self::git(path, &["rev-parse", "--is-inside-work-tree"])?;

        // Obtém o nome da branch
        let branch = Self::git(path, &["rev-parse", "--abbrev-ref", "HEAD"])
            .unwrap_or_else(|| "unknown".to_string());

        // Obtém status detalhado via porcelain
        let mut staged = 0;
        let mut modified = 0;
        let mut untracked = 0;

        if let Some(status_text) = Self::git(path, &["status", "--porcelain"]) {
            for line in status_text.lines() {
                if line.len() < 2 { continue; }

                let x = line.chars().next().unwrap_or(' ');
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
        let repo_name = Self::git(path, &["rev-parse", "--show-toplevel"])
            .map(std::path::PathBuf::from)
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
            .unwrap_or_else(|| "repo".to_string());

        // Verifica ahead/behind (só se houver upstream configurado)
        let mut ahead = 0;
        let mut behind = 0;

        let has_upstream =
            Self::git(path, &["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"])
                .is_some();

        if has_upstream {
            if let Some(counts) =
                Self::git(path, &["rev-list", "--left-right", "--count", "HEAD...@{u}"])
            {
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
