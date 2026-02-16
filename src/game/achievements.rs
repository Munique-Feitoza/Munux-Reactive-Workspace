// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use crate::game::state::{GameState, Achievement};
use crate::i18n::I18n;
use chrono::Utc;

/// Verifica e desbloqueia conquistas baseado na ação do usuário
pub struct AchievementChecker;

impl AchievementChecker {
    /// Verifica conquistas após executar um comando
    pub fn check_command(game_state: &mut GameState, command: &str, success: bool, i18n: &I18n) -> Option<Achievement> {
        let cmd_parts: Vec<&str> = command.split_whitespace().collect();
        if cmd_parts.is_empty() {
            return None;
        }
        
        let base_cmd = cmd_parts[0];
        
        // Conquista: Primeiro Comando
        if game_state.total_commands == 1 && success {
            return Self::unlock_if_new(game_state, "first_command", i18n, 10);
        }

        // Conquistas por comando específico
        if success {
            match base_cmd {
                "ls" if !game_state.has_achievement("first_ls") => {
                    return Self::unlock_if_new(game_state, "first_ls", i18n, 15);
                },
                "cd" if !game_state.has_achievement("first_cd") => {
                    return Self::unlock_if_new(game_state, "first_cd", i18n, 15);
                },
                "touch" if !game_state.has_achievement("first_file") => {
                    return Self::unlock_if_new(game_state, "first_file", i18n, 20);
                },
                "mkdir" if !game_state.has_achievement("first_dir") => {
                    return Self::unlock_if_new(game_state, "first_dir", i18n, 20);
                },
                "cat" if !game_state.has_achievement("first_cat") => {
                    return Self::unlock_if_new(game_state, "first_cat", i18n, 15);
                },
                "rm" if !game_state.has_achievement("first_rm") => {
                    return Self::unlock_if_new(game_state, "first_rm", i18n, 25);
                },
                "sudo" if !game_state.has_achievement("first_sudo") => {
                    return Self::unlock_if_new(game_state, "first_sudo", i18n, 50);
                },
                "pacman" | "yay" | "paru" if !game_state.has_achievement("first_pacman") => {
                    return Self::unlock_if_new(game_state, "first_pacman", i18n, 40);
                },
                "apt" | "apt-get" if !game_state.has_achievement("first_apt") => {
                    return Self::unlock_if_new(game_state, "first_apt", i18n, 40);
                },
                "git" if !game_state.has_achievement("first_git") => {
                    return Self::unlock_if_new(game_state, "first_git", i18n, 35);
                },
                "systemctl" if !game_state.has_achievement("first_systemctl") => {
                    return Self::unlock_if_new(game_state, "first_systemctl", i18n, 40);
                },
                _ => {}
            }
        }
        
        // SSH (check independente de sucesso)
        if base_cmd == "ssh" && !game_state.has_achievement("first_ssh") {
            return Self::unlock_if_new(game_state, "first_ssh", i18n, 45);
        }
        
        // Conquistas por quantidade
        match game_state.total_commands {
            10 if !game_state.has_achievement("commands_10") => {
                return Self::unlock_if_new(game_state, "commands_10", i18n, 30);
            },
            50 if !game_state.has_achievement("commands_50") => {
                return Self::unlock_if_new(game_state, "commands_50", i18n, 100);
            },
            100 if !game_state.has_achievement("commands_100") => {
                return Self::unlock_if_new(game_state, "commands_100", i18n, 200);
            },
            500 if !game_state.has_achievement("commands_500") => {
                return Self::unlock_if_new(game_state, "commands_500", i18n, 500);
            },
            _ => {}
        }
        
        // Conquista: Pipe Master
        if command.contains('|') && !game_state.has_achievement("pipe_master") {
             return Self::unlock_if_new(game_state, "pipe_master", i18n, 30);
        }
        
        None
    }
    
    /// Verifica conquistas baseadas em streak
    pub fn check_streak(game_state: &mut GameState, i18n: &I18n) -> Option<Achievement> {
        match game_state.command_streak {
            5 if !game_state.has_achievement("streak_5") => {
                Self::unlock_if_new(game_state, "streak_5", i18n, 40)
            },
            10 if !game_state.has_achievement("streak_10") => {
                Self::unlock_if_new(game_state, "streak_10", i18n, 80)
            },
            25 if !game_state.has_achievement("streak_25") => {
                Self::unlock_if_new(game_state, "streak_25", i18n, 200)
            },
            _ => None
        }
    }
    
    /// Verifica conquistas de nível
    pub fn check_level(game_state: &mut GameState, i18n: &I18n) -> Option<Achievement> {
        match game_state.level {
            5 if !game_state.has_achievement("level_5") => {
                Self::unlock_if_new(game_state, "level_5", i18n, 100)
            },
            10 if !game_state.has_achievement("level_10") => {
                Self::unlock_if_new(game_state, "level_10", i18n, 150)
            },
            20 if !game_state.has_achievement("level_20") => {
                Self::unlock_if_new(game_state, "level_20", i18n, 250)
            },
            30 if !game_state.has_achievement("level_30") => {
                Self::unlock_if_new(game_state, "level_30", i18n, 500)
            },
            50 if !game_state.has_achievement("level_50") => {
                Self::unlock_if_new(game_state, "level_50", i18n, 1000)
            },
            _ => None
        }
    }
    
    /// Easter eggs
    pub fn check_easter_egg(game_state: &mut GameState, command: &str, i18n: &I18n) -> Option<Achievement> {
        match command {
            "sudo rm -rf /" if !game_state.has_achievement("easter_egg_nuke") => {
                Self::unlock_if_new(game_state, "easter_egg_nuke", i18n, 666)
            },
            "sl" if !game_state.has_achievement("easter_egg_train") => {
                Self::unlock_if_new(game_state, "easter_egg_train", i18n, 25)
            },
            c if c.contains("cowsay") && !game_state.has_achievement("easter_egg_cow") => {
                Self::unlock_if_new(game_state, "easter_egg_cow", i18n, 30)
            },
            _ => None
        }
    }
    
    /// Helper para desbloquear conquista
    fn unlock_if_new(
        game_state: &mut GameState,
        id: &str,
        i18n: &I18n,
        xp_reward: u32,
    ) -> Option<Achievement> {
        if game_state.has_achievement(id) {
            return None;
        }
        
        let (name, description) = i18n.achievement_info(id);
        
        let achievement = Achievement {
            id: id.to_string(),
            name,
            description,
            unlocked_at: Utc::now(),
            xp_reward,
        };
        
        game_state.achievements.push(achievement.clone());
        Some(achievement)
    }
}
