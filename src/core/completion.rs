// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

//! Auto-complete (Tab) para comandos e caminhos.
//!
//! Ao pressionar Tab, a última "palavra" do buffer de input é completada:
//! - se for a primeira palavra, completa a partir dos comandos especiais do
//!   Munux + executáveis encontrados no `$PATH`;
//! - caso contrário, completa caminhos de arquivos/diretórios relativos ao
//!   diretório de trabalho atual.

use std::path::Path;

/// Comandos internos/especiais do Munux que sempre aparecem na completação.
const BUILTIN_COMMANDS: &[&str] = &[
    "help", "stats", "quests", "missions", "achievements", "xp", "clear", "exit",
    "alias", "unalias", "tip", "tutorial", "benchmark",
    "cd", "ls", "ll", "la", "pwd", "cat", "less", "grep", "find", "git", "ssh",
    "mkdir", "touch", "cp", "mv", "rm", "rmdir", "top", "htop", "ps", "df", "du",
    "ping", "curl", "wget", "tar", "zip", "unzip", "sudo", "systemctl", "cowsay",
    "fortune", "matrix",
];

/// Resultado de uma tentativa de completação.
pub struct Completion {
    /// Novo conteúdo do buffer de input (igual ao original se nada mudou).
    pub new_input: String,
    /// Candidatos quando há ambiguidade (vazio quando há 0 ou 1 candidato).
    pub suggestions: Vec<String>,
}

impl Completion {
    fn unchanged(input: &str) -> Self {
        Completion { new_input: input.to_string(), suggestions: Vec::new() }
    }
}

/// Tenta completar o `input` considerando o diretório de trabalho `cwd`.
pub fn complete(input: &str, cwd: &Path) -> Completion {
    // Não dá pra completar dentro de espaços à direita de forma útil; trata
    // "git " como "completar argumento vazio" (lista o diretório atual).
    let trailing_space = input.ends_with(' ');
    let words: Vec<&str> = input.split_whitespace().collect();

    let (prefix_so_far, partial) = if trailing_space || words.is_empty() {
        (input.to_string(), "")
    } else {
        let partial = *words.last().unwrap();
        let cut = input.len() - partial.len();
        (input[..cut].to_string(), partial)
    };

    let is_first_word = words.len() <= 1 && !trailing_space;

    let candidates = if is_first_word {
        command_candidates(partial)
    } else {
        path_candidates(partial, cwd)
    };

    match candidates.len() {
        0 => Completion::unchanged(input),
        1 => Completion {
            new_input: format!("{}{}", prefix_so_far, candidates[0]),
            suggestions: Vec::new(),
        },
        _ => {
            let common = longest_common_prefix(&candidates);
            // Só estende o buffer se o prefixo comum for maior que o digitado.
            let new_input = if common.len() > partial.len() {
                format!("{}{}", prefix_so_far, common)
            } else {
                input.to_string()
            };
            Completion { new_input, suggestions: candidates }
        }
    }
}

/// Comandos (builtins + executáveis do `$PATH`) que começam com `partial`.
fn command_candidates(partial: &str) -> Vec<String> {
    let mut set: Vec<String> = Vec::new();
    let mut push = |name: &str| {
        if name.starts_with(partial) && !set.iter().any(|n| n == name) {
            set.push(name.to_string());
        }
    };

    for c in BUILTIN_COMMANDS {
        push(c);
    }

    if let Ok(path_var) = std::env::var("PATH") {
        for dir in path_var.split(':').filter(|d| !d.is_empty()) {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    if let Ok(name) = entry.file_name().into_string() {
                        if name.starts_with(partial) {
                            push(&name);
                        }
                    }
                }
            }
        }
    }

    set.sort();
    set
}

/// Caminhos relativos a `cwd` que começam com `partial`. Diretórios recebem `/`.
fn path_candidates(partial: &str, cwd: &Path) -> Vec<String> {
    // Separa o diretório já digitado do fragmento final do nome.
    let (dir_part, name_part) = match partial.rfind('/') {
        Some(idx) => (&partial[..=idx], &partial[idx + 1..]),
        None => ("", partial),
    };

    let search_dir = if dir_part.is_empty() {
        cwd.to_path_buf()
    } else if dir_part.starts_with('/') {
        Path::new(dir_part).to_path_buf()
    } else {
        cwd.join(dir_part)
    };

    let mut out: Vec<String> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&search_dir) {
        for entry in entries.flatten() {
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();
            // Esconde dotfiles a menos que o usuário tenha digitado o "."
            if name.starts_with('.') && !name_part.starts_with('.') {
                continue;
            }
            if name.starts_with(name_part) {
                let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
                let suffix = if is_dir { "/" } else { "" };
                out.push(format!("{}{}{}", dir_part, name, suffix));
            }
        }
    }

    out.sort();
    out
}

/// Maior prefixo comum a todas as strings.
fn longest_common_prefix(items: &[String]) -> String {
    if items.is_empty() {
        return String::new();
    }
    let first = &items[0];
    let mut len = first.len();
    for item in &items[1..] {
        len = first
            .bytes()
            .zip(item.bytes())
            .take(len)
            .take_while(|(a, b)| a == b)
            .count();
        if len == 0 {
            break;
        }
    }
    // `len` está em bytes; garante que cai num limite de caractere.
    while !first.is_char_boundary(len) {
        len -= 1;
    }
    first[..len].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lcp_basic() {
        let v = vec!["acha".to_string(), "ache".to_string(), "achievements".to_string()];
        assert_eq!(longest_common_prefix(&v), "ach");
    }

    #[test]
    fn completes_unique_builtin() {
        // "achie" só casa com "achievements"
        let c = complete("achie", Path::new("/"));
        assert_eq!(c.new_input, "achievements");
        assert!(c.suggestions.is_empty());
    }

    #[test]
    fn ambiguous_extends_to_common_prefix() {
        // "ac" casa com "achievements" e nada mais nos builtins? "ac"... only achievements.
        // Use "h" -> help, htop -> common prefix "h" (no extension), suggestions listed.
        let c = complete("h", Path::new("/"));
        assert!(c.suggestions.len() >= 2);
    }

    #[test]
    fn no_match_keeps_input() {
        let c = complete("zzzznotacommand", Path::new("/"));
        assert_eq!(c.new_input, "zzzznotacommand");
    }
}
