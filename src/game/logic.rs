// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

/// Calcula a recompensa de XP baseada no comando executado
use crate::core::parser::CommandType;

/// Calcula a recompensa de XP baseada no comando executado
pub fn calculate_xp_reward(command: &str, cmd_type: &CommandType, success: bool) -> u32 {
    if !success {
        // Comandos que falharam dão 0 XP
        return 0;
    }
    
    // XP base por tipo de comando
    let base_xp = match cmd_type {
        CommandType::Navigation => 5,        // cd, ls, pwd
        CommandType::FileOperation => {
            if command.starts_with("mkdir") || command.starts_with("touch") {
                10 // Criação
            } else {
                8  // Remoção/Movimentação
            }
        },
        CommandType::FileViewing => 7,       // cat, less, etc
        CommandType::SystemMonitoring => 10, // top, htop
        CommandType::Search => 15,           // grep, find
        CommandType::Dangerous => 30,        // sudo, etc (alto risco = alta recompensa se der certo)
        CommandType::VersionControl => 25,   // git
        CommandType::PackageManager => 20,   // pacman, apt
        CommandType::NetworkTools => 15,     // ping, curl
        CommandType::Compression => 15,      // tar, zip
        CommandType::TextProcessing => 12,   // sed, awk
        CommandType::SystemAdmin => 20,      // systemctl
        CommandType::MunuxSpecial => 5,      // stats, etc
        CommandType::EasterEgg => 50,        // Easter eggs dão bastante XP!
        CommandType::Unknown => {
             if command.contains("|") {
                20 // Pipes
            } else if command.contains("&&") || command.contains("||") {
                15 // Encadeamento
            } else {
                10 // Genérico
            }
        }
    };
    
    // Bônus por complexidade extra (pipes, redirects)
    let complexity_bonus = if command.contains("|") { 5 } else { 0 } + 
                          if command.contains("&&") { 5 } else { 0 } +
                          if command.len() > 20 { 2 } else { 0 }; // Comandos longos
    
    base_xp + complexity_bonus
}


