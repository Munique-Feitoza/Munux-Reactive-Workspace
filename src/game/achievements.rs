// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use crate::game::state::{GameState, Achievement};
use chrono::Utc;

/// Verifica e desbloqueia conquistas baseado na ação do usuário
pub struct AchievementChecker;

impl AchievementChecker {
    /// Verifica conquistas após executar um comando
    pub fn check_command(game_state: &mut GameState, command: &str, success: bool) -> Option<Achievement> {
        let cmd_parts: Vec<&str> = command.split_whitespace().collect();
        if cmd_parts.is_empty() {
            return None;
        }
        
        let base_cmd = cmd_parts[0];
        
        // Conquista: Primeiro Comando
        if game_state.total_commands == 1 && success {
            return Self::unlock_if_new(
                game_state,
                "first_command",
                "Primeira Jornada",
                "Executou seu primeiro comando no terminal",
                10,
            );
        }
        
        // Conquista: Primeiro ls
        if base_cmd == "ls" && success && !game_state.has_achievement("first_ls") {
            return Self::unlock_if_new(
                game_state,
                "first_ls",
                "Explorador Iniciante",
                "Listou arquivos pela primeira vez",
                15,
            );
        }
        
        // Conquista: Primeiro cd
        if base_cmd == "cd" && success && !game_state.has_achievement("first_cd") {
            return Self::unlock_if_new(
                game_state,
                "first_cd",
                "Viajante",
                "Navegou entre diretórios pela primeira vez",
                15,
            );
        }
        
        // Conquista: Primeiro arquivo criado
        if base_cmd == "touch" && success && !game_state.has_achievement("first_file") {
            return Self::unlock_if_new(
                game_state,
                "first_file",
                "Criador",
                "Criou seu primeiro arquivo",
                20,
            );
        }
        
        // Conquista: Primeira pasta criada
        if base_cmd == "mkdir" && success && !game_state.has_achievement("first_dir") {
            return Self::unlock_if_new(
                game_state,
                "first_dir",
                "Arquiteto",
                "Criou seu primeiro diretório",
                20,
            );
        }
        
        // Conquista: Primeiro cat
        if base_cmd == "cat" && success && !game_state.has_achievement("first_cat") {
            return Self::unlock_if_new(
                game_state,
                "first_cat",
                "Leitor",
                "Leu o conteúdo de um arquivo",
                15,
            );
        }
        
        // Conquista: Primeiro rm
        if base_cmd == "rm" && success && !game_state.has_achievement("first_rm") {
            return Self::unlock_if_new(
                game_state,
                "first_rm",
                "Destruidor",
                "Removeu seu primeiro arquivo (com grandes poderes...)",
                25,
            );
        }
        
        // Conquista: Primeiro sudo
        if base_cmd == "sudo" && success && !game_state.has_achievement("first_sudo") {
            return Self::unlock_if_new(
                game_state,
                "first_sudo",
                "Root Master",
                "Usou poderes de super usuário",
                50,
            );
        }
        
        // Conquista: Primeiro pacman (Arch/Manjaro)
        if (base_cmd == "pacman" || base_cmd == "yay" || base_cmd == "paru") 
            && success && !game_state.has_achievement("first_pacman") {
            return Self::unlock_if_new(
                game_state,
                "first_pacman",
                "Arch User",
                "Usou o pacman pela primeira vez - BTW, I use Arch!",
                40,
            );
        }
        
        // Conquista: Primeiro apt (Debian/Ubuntu)
        if (base_cmd == "apt" || base_cmd == "apt-get") 
            && success && !game_state.has_achievement("first_apt") {
            return Self::unlock_if_new(
                game_state,
                "first_apt",
                "Debian Disciple",
                "Usou apt pela primeira vez - The universal operating system!",
                40,
            );
        }
        
        // Conquista: Primeiro git
        if base_cmd == "git" && success && !game_state.has_achievement("first_git") {
            return Self::unlock_if_new(
                game_state,
                "first_git",
                "Version Control Master",
                "Começou a usar controle de versão",
                35,
            );
        }
        
        // Conquista: Primeiro SSH
        if base_cmd == "ssh" && !game_state.has_achievement("first_ssh") {
            return Self::unlock_if_new(
                game_state,
                "first_ssh",
                "Remote Access",
                "Conectou-se remotamente via SSH",
                45,
            );
        }
        
        // Conquista: Primeiro systemctl
        if base_cmd == "systemctl" && success && !game_state.has_achievement("first_systemctl") {
            return Self::unlock_if_new(
                game_state,
                "first_systemctl",
                "Systemd Warrior",
                "Gerenciou serviços do sistema",
                40,
            );
        }
        
        // Conquistas por quantidade de comandos
        if game_state.total_commands == 10 && !game_state.has_achievement("commands_10") {
            return Self::unlock_if_new(
                game_state,
                "commands_10",
                "Praticante",
                "Executou 10 comandos",
                30,
            );
        }
        
        if game_state.total_commands == 50 && !game_state.has_achievement("commands_50") {
            return Self::unlock_if_new(
                game_state,
                "commands_50",
                "Veterano",
                "Executou 50 comandos",
                100,
            );
        }
        
        if game_state.total_commands == 100 && !game_state.has_achievement("commands_100") {
            return Self::unlock_if_new(
                game_state,
                "commands_100",
                "Centurião",
                "Executou 100 comandos!",
                200,
            );
        }
        
        if game_state.total_commands == 500 && !game_state.has_achievement("commands_500") {
            return Self::unlock_if_new(
                game_state,
                "commands_500",
                "Terminal Master",
                "Executou 500 comandos! Você é imparável!",
                500,
            );
        }
        
        None
    }
    
    /// Verifica conquistas baseadas em streak
    pub fn check_streak(game_state: &mut GameState) -> Option<Achievement> {
        if game_state.command_streak == 5 && !game_state.has_achievement("streak_5") {
            return Self::unlock_if_new(
                game_state,
                "streak_5",
                "Em Ritmo!",
                "5 comandos corretos seguidos",
                40,
            );
        }
        
        if game_state.command_streak == 10 && !game_state.has_achievement("streak_10") {
            return Self::unlock_if_new(
                game_state,
                "streak_10",
                "Imparável!",
                "10 comandos corretos seguidos",
                80,
            );
        }
        
        if game_state.command_streak == 25 && !game_state.has_achievement("streak_25") {
            return Self::unlock_if_new(
                game_state,
                "streak_25",
                "Perfeição!",
                "25 comandos corretos seguidos - você é um mestre!",
                200,
            );
        }
        
        None
    }
    
    /// Verifica conquistas de nível
    pub fn check_level(game_state: &mut GameState) -> Option<Achievement> {
        let level = game_state.level;
        
        if level == 5 && !game_state.has_achievement("level_5") {
            return Self::unlock_if_new(
                game_state,
                "level_5",
                "Terminal User",
                "Alcançou o nível 5 - Modo de segurança desativado!",
                100,
            );
        }
        
        if level == 10 && !game_state.has_achievement("level_10") {
            return Self::unlock_if_new(
                game_state,
                "level_10",
                "Script Kiddie",
                "Alcançou o nível 10 - Tema Hacker desbloqueado!",
                150,
            );
        }
        
        if level == 20 && !game_state.has_achievement("level_20") {
            return Self::unlock_if_new(
                game_state,
                "level_20",
                "Sysadmin",
                "Alcançou o nível 20 - Tema Cyberpunk desbloqueado!",
                250,
            );
        }
        
        if level == 30 && !game_state.has_achievement("level_30") {
            return Self::unlock_if_new(
                game_state,
                "level_30",
                "Elite Hacker",
                "Alcançou o nível 30 - Você é parte da elite!",
                500,
            );
        }
        
        if level == 50 && !game_state.has_achievement("level_50") {
            return Self::unlock_if_new(
                game_state,
                "level_50",
                "LEGEND",
                "Alcançou o nível 50 - VOCÊ É UMA LENDA!",
                1000,
            );
        }
        
        None
    }
    
    /// Easter eggs
    pub fn check_easter_egg(game_state: &mut GameState, command: &str) -> Option<Achievement> {
        if command == "sudo rm -rf /" && !game_state.has_achievement("easter_egg_nuke") {
            return Self::unlock_if_new(
                game_state,
                "easter_egg_nuke",
                "Bomba Nuclear",
                "Tentou deletar o universo (mas foi bloqueado)",
                666,
            );
        }
        
        if command.contains("cowsay") && !game_state.has_achievement("easter_egg_cow") {
            return Self::unlock_if_new(
                game_state,
                "easter_egg_cow",
                "Moo!",
                "A vaca disse algo interessante",
                30,
            );
        }
        
        if command == "sl" && !game_state.has_achievement("easter_egg_train") {
            return Self::unlock_if_new(
                game_state,
                "easter_egg_train",
                "Choo Choo!",
                "Errou 'ls' e achou o trem!",
                25,
            );
        }
        
        None
    }
    
    /// Helper para desbloquear conquista
    fn unlock_if_new(
        game_state: &mut GameState,
        id: &str,
        name: &str,
        description: &str,
        xp_reward: u32,
    ) -> Option<Achievement> {
        if game_state.has_achievement(id) {
            return None;
        }
        
        let achievement = Achievement {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            unlocked_at: Utc::now(),
            xp_reward,
        };
        
        game_state.achievements.push(achievement.clone());
        // NÃO adiciona XP aqui - será feito no app.rs para mostrar notificação
        
        Some(achievement)
    }
}
