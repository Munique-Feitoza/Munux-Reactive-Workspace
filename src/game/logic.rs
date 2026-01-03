// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

/// Calcula a recompensa de XP baseada no comando executado
pub fn calculate_xp_reward(command: &str, success: bool) -> u32 {
    if !success {
        // Comandos que falharam dão 0 XP
        return 0;
    }
    
    let base_xp = if command.trim().is_empty() {
        0
    } else if command.starts_with("cd") {
        5 // Navegação básica
    } else if command.starts_with("ls") || command.starts_with("pwd") {
        3 // Listagem
    } else if command.starts_with("mkdir") || command.starts_with("touch") {
        10 // Criação de arquivos
    } else if command.starts_with("rm") || command.starts_with("rmdir") {
        8 // Remoção (perigoso mas necessário)
    } else if command.starts_with("cat") 
           || command.starts_with("less") 
           || command.starts_with("head") 
           || command.starts_with("tail") {
        7 // Leitura de arquivos
    } else if command.starts_with("grep") || command.starts_with("find") {
        15 // Busca avançada
    } else if command.starts_with("nano") 
           || command.starts_with("vim") 
           || command.starts_with("vi") {
        20 // Edição de texto
    } else if command.starts_with("git") {
        25 // Controle de versão
    } else if command.starts_with("sudo") {
        30 // Comandos administrativos (alto risco)
    } else if command.contains("|") {
        20 // Uso de pipes (conhecimento intermediário)
    } else if command.contains("&&") || command.contains("||") {
        15 // Encadeamento de comandos
    } else {
        10 // Comando padrão
    };
    
    base_xp
}

/// Verifica se o comando merece uma conquista especial
pub fn check_achievements(command: &str, total_commands: u32) -> Option<(&'static str, &'static str, &'static str, u32)> {
    // Retorna: (id, nome, descrição, xp_bonus)
    
    // Primeira vez usando git
    if command.starts_with("git") && total_commands < 50 {
        return Some((
            "first_git",
            "Versionado",
            "Executou seu primeiro comando Git",
            50
        ));
    }
    
    // Primeiro uso de pipe
    if command.contains("|") {
        return Some((
            "pipe_master",
            "Encanador",
            "Dominou o uso de pipes no terminal",
            30
        ));
    }
    
    // Primeiro uso de sudo
    if command.starts_with("sudo") {
        return Some((
            "sudo_warrior",
            "Administrador",
            "Ganhou poderes de root",
            100
        ));
    }
    
    // Marcos de comandos totais
    match total_commands {
        10 => Some((
            "novice",
            "Novato",
            "Executou 10 comandos",
            25
        )),
        50 => Some((
            "explorer",
            "Explorador",
            "Executou 50 comandos",
            50
        )),
        100 => Some((
            "veteran",
            "Veterano",
            "Executou 100 comandos",
            100
        )),
        500 => Some((
            "master",
            "Mestre",
            "Executou 500 comandos",
            250
        )),
        _ => None
    }
}
