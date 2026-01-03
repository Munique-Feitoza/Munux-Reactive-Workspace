// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use crate::app::RightPanelMode;
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
    /// Analisa o comando e retorna o tipo
    pub fn classify_command(input: &str) -> CommandType {
        let trimmed = input.trim();
        
        if trimmed.is_empty() {
            return CommandType::Unknown;
        }
        
        // Extrai o primeiro token (comando base)
        let first_word = trimmed.split_whitespace().next().unwrap_or("");
        
        match first_word {
            // Navegação
            "cd" | "ls" | "pwd" | "dirs" | "pushd" | "popd" => CommandType::Navigation,
            
            // Operações de arquivo
            "mkdir" | "touch" | "cp" | "mv" | "rmdir" => CommandType::FileOperation,
            
            // Visualização de arquivos
            "cat" | "less" | "more" | "head" | "tail" | "nano" | "vim" | "vi" | "emacs" => {
                CommandType::FileViewing
            }
            
            // Monitoramento
            "top" | "htop" | "ps" | "free" | "df" | "du" | "vmstat" | "iostat" => {
                CommandType::SystemMonitoring
            }
            
            // Busca
            "grep" | "find" | "locate" | "which" | "whereis" => CommandType::Search,
            
            // Git
            "git" => CommandType::VersionControl,
            
            // Gerenciadores de Pacotes
            "pacman" | "yay" | "paru" | "pamac" => CommandType::PackageManager,  // Arch/Manjaro
            "apt" | "apt-get" | "aptitude" | "dpkg" => CommandType::PackageManager,  // Debian/Ubuntu
            "dnf" | "yum" => CommandType::PackageManager,  // Fedora/RHEL
            "zypper" => CommandType::PackageManager,  // openSUSE
            "snap" | "flatpak" | "appimage" => CommandType::PackageManager,  // Universal
            
            // Ferramentas de Rede
            "ping" | "curl" | "wget" | "ssh" | "scp" | "rsync" | "netstat" | "ip" | "ifconfig" => {
                CommandType::NetworkTools
            }
            
            // Compressão/Arquivamento
            "tar" | "zip" | "unzip" | "gzip" | "gunzip" | "bzip2" | "7z" | "rar" | "unrar" => {
                CommandType::Compression
            }
            
            // Processamento de Texto
            "sed" | "awk" | "cut" | "sort" | "uniq" | "wc" | "tr" | "diff" | "patch" => {
                CommandType::TextProcessing
            }
            
            // Administração do Sistema
            "systemctl" | "service" | "journalctl" | "dmesg" | "uname" | "hostname" | "reboot" | "shutdown" => {
                CommandType::SystemAdmin
            }
            
            // Comandos perigosos
            "rm" => {
                // Verifica se tem flags destrutivas
                if trimmed.contains("-rf") 
                    || trimmed.contains("-fr") 
                    || trimmed.contains("-r")
                    || trimmed.contains("-f") {
                    CommandType::Dangerous
                } else {
                    CommandType::FileOperation
                }
            }
            "sudo" | "dd" | "mkfs" | "fdisk" | "parted" | "chmod" | "chown" => {
                CommandType::Dangerous
            }
            
            // Comandos especiais do Munux
            "stats" | "quests" | "missions" | "achievements" | "xp" | "help" => {
                CommandType::MunuxSpecial
            }
            
            // Easter eggs
            "sl" | "cowsay" | "fortune" | "matrix" | "hack" | "konami" => {
                CommandType::EasterEgg
            }
            
            _ => CommandType::Unknown,
        }
    }
    
    /// Converte o tipo de comando para o modo do painel direito
    pub fn command_to_panel_mode(input: &str, current_dir: &PathBuf) -> RightPanelMode {
        let cmd_type = Self::classify_command(input);
        let trimmed = input.trim();
        
        match cmd_type {
            CommandType::Dangerous => {
                // Mensagens específicas por tipo de comando perigoso
                let warning = if trimmed.contains("rm") && (trimmed.contains("-rf") || trimmed.contains("-fr")) {
                    if trimmed.contains("/") && (trimmed.contains("/*") || trimmed.ends_with("/")) {
                        "REMOÇÃO RECURSIVA EM DIRETÓRIO RAIZ!"
                    } else {
                        "Remoção recursiva e forçada de arquivos"
                    }
                } else if trimmed.contains("rm") {
                    "Remoção de arquivo(s) - operação irreversível"
                } else if trimmed.starts_with("sudo") {
                    "Execução com privilégios de superusuário"
                } else if trimmed.contains("dd") {
                    "Cópia de baixo nível - pode sobrescrever dados"
                } else if trimmed.contains("mkfs") || trimmed.contains("fdisk") || trimmed.contains("parted") {
                    "Modificação de partições/sistema de arquivos"
                } else if trimmed.contains("chmod") || trimmed.contains("chown") {
                    "Modificação de permissões/propriedade de arquivos"
                } else if trimmed.contains("reboot") || trimmed.contains("shutdown") || trimmed.contains("poweroff") {
                    "Desligamento/reinicialização do sistema"
                } else {
                    "Comando potencialmente destrutivo detectado"
                };
                
                RightPanelMode::DangerZone {
                    warning: warning.to_string(),
                    command: trimmed.to_string(),
                }
            }
            
            CommandType::FileViewing => {
                // Extrai o nome do arquivo (parcial ou completo)
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 2 {
                    let filename = parts[1];
                    
                    // Busca arquivos que correspondem
                    let matches = Self::find_matching_files(current_dir, filename);
                    
                    if matches.len() == 1 {
                        // Se encontrou exatamente 1, mostra preview
                        RightPanelMode::FilePreview {
                            path: matches[0].clone(),
                            content: String::new(),
                            language: Self::detect_language(filename),
                        }
                    } else if matches.len() > 1 {
                        // Se encontrou vários, mostra lista de sugestões
                        let suggestions = matches.iter()
                            .filter_map(|p| p.file_name()?.to_str().map(|s| s.to_string()))
                            .collect::<Vec<_>>()
                            .join("\n  → ");
                        
                        RightPanelMode::FilePreview {
                            path: current_dir.join(filename),
                            content: format!("💡 Arquivos encontrados:\n\n  → {}", suggestions),
                            language: "text".to_string(),
                        }
                    } else {
                        // Nenhum encontrado, mostra erro
                        RightPanelMode::FilePreview {
                            path: current_dir.join(filename),
                            content: String::new(),
                            language: Self::detect_language(filename),
                        }
                    }
                } else {
                    RightPanelMode::FileTree {
                        path: current_dir.clone(),
                    }
                }
            }
            
            CommandType::SystemMonitoring => {
                RightPanelMode::ResourceMonitor {
                    cpu_usage: 0.0,
                    memory_used: 0,
                    memory_total: 0,
                    process_count: 0,
                }
            }
            
            _ => RightPanelMode::FileTree {
                path: current_dir.clone(),
            },
        }
    }
    
    /// Detecta a linguagem do arquivo pela extensão
    pub fn detect_language(filename: &str) -> String {
        if filename.ends_with(".rs") {
            "rust".to_string()
        } else if filename.ends_with(".py") {
            "python".to_string()
        } else if filename.ends_with(".js") || filename.ends_with(".ts") {
            "javascript".to_string()
        } else if filename.ends_with(".sh") {
            "bash".to_string()
        } else if filename.ends_with(".toml") {
            "toml".to_string()
        } else if filename.ends_with(".json") {
            "json".to_string()
        } else if filename.ends_with(".md") {
            "markdown".to_string()
        } else {
            "text".to_string()
        }
    }
    
    /// Verifica se o comando requer permissões administrativas
    pub fn requires_sudo(input: &str) -> bool {
        let trimmed = input.trim();
        trimmed.starts_with("sudo") || 
        trimmed.starts_with("apt") ||
        trimmed.starts_with("systemctl") ||
        trimmed.starts_with("service")
    }
    
    /// Verifica se o comando é permitido no modo seguro
    pub fn is_safe_command(input: &str, safe_mode: bool) -> bool {
        if !safe_mode {
            return true; // Modo livre, tudo é permitido
        }
        
        let cmd_type = Self::classify_command(input);
        match cmd_type {
            CommandType::Dangerous => false,
            _ => !Self::requires_sudo(input),
        }
    }
    
    /// Busca todos os arquivos que correspondem ao nome parcial
    fn find_matching_files(dir: &PathBuf, partial_name: &str) -> Vec<PathBuf> {
        let mut matches = Vec::new();
        
        // Se o arquivo existe exatamente, retorna só ele
        let exact_path = dir.join(partial_name);
        if exact_path.exists() && exact_path.is_file() {
            return vec![exact_path];
        }
        
        // Busca arquivos que começam com o nome parcial OU contêm o texto
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if entry.path().is_file() {
                        // Corresponde se:
                        // 1. Começa com o texto digitado (tex -> texto.txt)
                        // 2. Contém o texto digitado (tex -> latex.txt)
                        // 3. É muito similar (diferença de 1-2 caracteres)
                        let lowercase_name = name.to_lowercase();
                        let lowercase_partial = partial_name.to_lowercase();
                        
                        if lowercase_name.starts_with(&lowercase_partial) ||
                           lowercase_name.contains(&lowercase_partial) ||
                           Self::is_similar(&lowercase_name, &lowercase_partial) {
                            matches.push(entry.path());
                        }
                    }
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
}
