// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use std::path::PathBuf;

/// Analisador de comandos em tempo real
pub struct CommandParser;

#[derive(Debug, Clone, PartialEq)]
pub enum CommandType {
    Navigation,        // cd, ls, pwd
    FileOperation,     // mkdir, touch, rm, mv, cp
    FileViewing,       // cat, less, head, tail, nano, vim
    SystemMonitoring,  // top, htop, ps, free
    Search,            // grep, find
    Dangerous,         // rm -rf, sudo, dd
    VersionControl,    // git
    PackageManager,    // pacman, yay, apt, dnf, zypper
    NetworkTools,      // ping, curl, wget, ssh
    Compression,       // tar, zip, unzip
    TextProcessing,    // sed, awk, grep
    SystemAdmin,       // systemctl, service, journalctl
    MunuxSpecial,      // stats, quests, achievements
    EasterEgg,         // sl, cowsay, fortune, matrix
    Unknown,
}

impl CommandParser {
    /// Analisa o comando e retorna o tipo, consultando a fonte única
    /// (`core::commands`). Só `rm` tem lógica especial: vira `Dangerous`
    /// quando acompanhado de flags destrutivas.
    pub fn classify_command(input: &str) -> CommandType {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return CommandType::Unknown;
        }

        // Extrai o primeiro token (comando base)
        let first_word = trimmed.split_whitespace().next().unwrap_or("");

        if first_word == "rm" {
            let destructive = trimmed.contains("-rf")
                || trimmed.contains("-fr")
                || trimmed.contains("-r")
                || trimmed.contains("-f");
            return if destructive {
                CommandType::Dangerous
            } else {
                CommandType::FileOperation
            };
        }

        crate::core::commands::command_type(first_word).unwrap_or(CommandType::Unknown)
    }
    
    /// Detecta a linguagem do arquivo pela extensão (delega à fonte única).
    pub fn detect_language(filename: &str) -> String {
        crate::core::filetype::classify(filename).language.to_string()
    }
    
    /// Verifica se o comando requer permissões administrativas
    pub fn requires_sudo(input: &str) -> bool {
        let trimmed = input.trim();
        trimmed.starts_with("sudo") || 
        trimmed.starts_with("apt") ||
        trimmed.starts_with("systemctl") ||
        trimmed.starts_with("service")
    }
    
    /// Verifica se o comando é permitido no modo seguro.
    ///
    /// Analisa **todos** os segmentos separados por `;`, `&&`, `||`, `|`, `&`
    /// (não apenas o primeiro token), fechando o bypass `echo ok; rm -rf /`.
    /// Substituição de comando (`$(...)` / crase) é bloqueada por poder esconder
    /// comandos perigosos dentro de um comando aparentemente seguro.
    pub fn is_safe_command(input: &str, safe_mode: bool) -> bool {
        if !safe_mode {
            return true; // Modo livre, tudo é permitido
        }

        if input.contains("$(") || input.contains('`') {
            return false;
        }

        Self::split_segments(input).into_iter().all(|segment| {
            let segment = segment.trim();
            if segment.is_empty() {
                return true;
            }
            !matches!(Self::classify_command(segment), CommandType::Dangerous)
                && !Self::requires_sudo(segment)
        })
    }

    /// Divide uma linha de comando em segmentos executáveis independentes,
    /// separados por `;`, `&&`, `||`, `|`, `&` ou nova linha.
    ///
    /// Conservador por design (uso exclusivo na validação do modo seguro): não
    /// interpreta aspas, então no máximo divide demais — nunca de menos. Os
    /// separadores são ASCII, garantindo fatiamento em fronteiras UTF-8 válidas.
    fn split_segments(input: &str) -> Vec<&str> {
        let bytes = input.as_bytes();
        let mut segments = Vec::new();
        let mut start = 0;
        let mut i = 0;
        while i < bytes.len() {
            match bytes[i] {
                b';' | b'\n' | b'|' | b'&' => {
                    segments.push(&input[start..i]);
                    // Operadores duplos (`&&`, `||`) contam como um separador só.
                    if (bytes[i] == b'&' || bytes[i] == b'|')
                        && i + 1 < bytes.len()
                        && bytes[i + 1] == bytes[i]
                    {
                        i += 1;
                    }
                    i += 1;
                    start = i;
                }
                _ => i += 1,
            }
        }
        segments.push(&input[start..]);
        segments
    }
    
    /// Busca todos os arquivos que correspondem ao nome parcial.
    /// `pub(crate)` para a camada `app` montar o painel de preview/sugestões.
    pub(crate) fn find_matching_files(dir: &PathBuf, partial_name: &str) -> Vec<PathBuf> {
        let mut matches = Vec::new();
        
        // Se o arquivo existe exatamente, retorna só ele
        let exact_path = dir.join(partial_name);
        if exact_path.exists() && exact_path.is_file() {
            return vec![exact_path];
        }
        
        // Busca arquivos que começam com o nome parcial OU contêm o texto.
        //
        // O filtro segue a ordem barato -> caro: primeiro o nome (em memória),
        // só depois o tipo do arquivo. E o tipo vem de `entry.file_type()`, que
        // no Linux usa o `d_type` do `readdir` — `entry.path().is_file()`
        // montava um `PathBuf` e disparava um `stat` **por entrada**, e esta
        // função roda a cada tecla digitada num `cat ...`.
        let lowercase_partial = partial_name.to_lowercase();
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let file_name = entry.file_name();
                let Some(name) = file_name.to_str() else { continue };

                // Corresponde se:
                // 1. Começa com o texto digitado (tex -> texto.txt)
                // 2. Contém o texto digitado (tex -> latex.txt)
                // 3. É muito similar (diferença de 1-2 caracteres)
                let lowercase_name = name.to_lowercase();
                let matched = lowercase_name.contains(&lowercase_partial)
                    || Self::is_similar(&lowercase_name, &lowercase_partial);

                if matched && entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
                    matches.push(entry.path());
                }
            }
        }
        
        // Ordena por relevância: primeiro os que começam com, depois os que contêm
        matches.sort_by(|a, b| {
            let name_a = a.file_name().and_then(|n| n.to_str()).unwrap_or("");
            let name_b = b.file_name().and_then(|n| n.to_str()).unwrap_or("");
            let partial_lower = partial_name.to_lowercase();
            
            let starts_a = name_a.to_lowercase().starts_with(&partial_lower);
            let starts_b = name_b.to_lowercase().starts_with(&partial_lower);
            
            match (starts_a, starts_b) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => name_a.cmp(name_b),
            }
        });
        
        matches
    }
    
    /// Verifica se dois textos são similares (diferença máxima de 2 caracteres)
    fn is_similar(s1: &str, s2: &str) -> bool {
        // Ignora se a diferença de tamanho for muito grande
        let len_diff = (s1.len() as i32 - s2.len() as i32).abs();
        if len_diff > 3 {
            return false;
        }
        
        // Conta diferenças de caracteres
        let mut differences = 0;
        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();
        let min_len = chars1.len().min(chars2.len());
        
        for i in 0..min_len {
            if chars1[i] != chars2[i] {
                differences += 1;
            }
        }
        
        differences += len_diff as usize;
        
        // Similar se tem no máximo 2 diferenças
        differences <= 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_classify_navigation() {
        assert_eq!(CommandParser::classify_command("cd /home"), CommandType::Navigation);
        assert_eq!(CommandParser::classify_command("ls -la"), CommandType::Navigation);
    }
    
    #[test]
    fn test_classify_dangerous() {
        assert_eq!(CommandParser::classify_command("rm -rf /"), CommandType::Dangerous);
        assert_eq!(CommandParser::classify_command("sudo apt update"), CommandType::Dangerous);
    }
    
    #[test]
    fn test_classify_file_viewing() {
        assert_eq!(CommandParser::classify_command("cat test.txt"), CommandType::FileViewing);
    }

    #[test]
    fn test_safe_mode_blocks_separator_bypass() {
        // Bypass histórico: comando perigoso após um separador passava batido.
        assert!(!CommandParser::is_safe_command("echo ok; rm -rf /tmp/x", true));
        assert!(!CommandParser::is_safe_command("x=1 && rm -rf algo", true));
        assert!(!CommandParser::is_safe_command("echo a || dd if=/dev/zero of=/dev/sda", true));
        assert!(!CommandParser::is_safe_command("ls | sudo tee /etc/x", true));
        // Substituição de comando esconde o perigoso.
        assert!(!CommandParser::is_safe_command("echo $(rm -rf /)", true));
        assert!(!CommandParser::is_safe_command("echo `rm -rf /`", true));
        // Comandos legítimos continuam permitidos.
        assert!(CommandParser::is_safe_command("ls -la", true));
        assert!(CommandParser::is_safe_command("echo ok && ls", true));
        // Modo livre permite tudo.
        assert!(CommandParser::is_safe_command("rm -rf /tmp/x", false));
    }
}
