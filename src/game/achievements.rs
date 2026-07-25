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
            if let Some(a) = Self::unlock_if_new(game_state, "first_command", i18n, 10) {
                return Some(a);
            }
        }

        // Primeiro uso de cada comando (lista de aliases -> id -> xp). `unlock_if_new`
        // já evita duplicar; usamos `if let` para não cortar os checks por
        // quantidade quando a conquista de comando já estava desbloqueada.
        if success {
            const FIRST_USE: &[(&[&str], &str, u32)] = &[
                (&["ls"], "first_ls", 15),
                (&["cd"], "first_cd", 15),
                (&["touch"], "first_file", 20),
                (&["mkdir"], "first_dir", 20),
                (&["cat"], "first_cat", 15),
                (&["rm"], "first_rm", 25),
                (&["sudo"], "first_sudo", 50),
                (&["pacman", "yay", "paru"], "first_pacman", 40),
                (&["apt", "apt-get"], "first_apt", 40),
                (&["git"], "first_git", 35),
                (&["systemctl"], "first_systemctl", 40),
            ];
            for (cmds, id, xp) in FIRST_USE {
                if cmds.contains(&base_cmd) {
                    if let Some(a) = Self::unlock_if_new(game_state, id, i18n, *xp) {
                        return Some(a);
                    }
                    break;
                }
            }
        }

        // SSH (independe de sucesso)
        if base_cmd == "ssh" {
            if let Some(a) = Self::unlock_if_new(game_state, "first_ssh", i18n, 45) {
                return Some(a);
            }
        }

        // Conquistas por quantidade (cada marco ocorre uma única vez)
        let by_count = match game_state.total_commands {
            10 => Some(("commands_10", 30)),
            50 => Some(("commands_50", 100)),
            100 => Some(("commands_100", 200)),
            500 => Some(("commands_500", 500)),
            _ => None,
        };
        if let Some((id, xp)) = by_count {
            if let Some(a) = Self::unlock_if_new(game_state, id, i18n, xp) {
                return Some(a);
            }
        }

        // Pipe Master
        if command.contains('|') {
            return Self::unlock_if_new(game_state, "pipe_master", i18n, 30);
        }

        None
    }
    
    /// Verifica conquistas baseadas em streak. `unlock_if_new` já barra duplicatas,
    /// então a guarda `has_achievement` foi removida.
    pub fn check_streak(game_state: &mut GameState, i18n: &I18n) -> Option<Achievement> {
        let (id, xp) = match game_state.command_streak {
            5 => ("streak_5", 40),
            10 => ("streak_10", 80),
            25 => ("streak_25", 200),
            _ => return None,
        };
        Self::unlock_if_new(game_state, id, i18n, xp)
    }

    /// Verifica conquistas de nível.
    pub fn check_level(game_state: &mut GameState, i18n: &I18n) -> Option<Achievement> {
        let (id, xp) = match game_state.level {
            5 => ("level_5", 100),
            10 => ("level_10", 150),
            20 => ("level_20", 250),
            30 => ("level_30", 500),
            50 => ("level_50", 1000),
            _ => return None,
        };
        Self::unlock_if_new(game_state, id, i18n, xp)
    }
    
    /// Conquistas de easter egg: a do próprio easter egg descoberto e, ao atingir
    /// o limiar, a meta-conquista "caçador de easter eggs".
    pub fn check_easter_egg(game_state: &mut GameState, command: &str, i18n: &I18n) -> Option<Achievement> {
        use crate::game::easter_eggs::{EasterEggs, HUNTER_THRESHOLD};

        let egg = EasterEggs::classify(command)?;

        if let Some((id, xp)) = EasterEggs::achievement(egg) {
            if let Some(achievement) = Self::unlock_if_new(game_state, id, i18n, xp) {
                return Some(achievement);
            }
        }

        // Meta-conquista: já encontrou easter eggs suficientes.
        let found = game_state
            .achievements
            .iter()
            .filter(|a| a.id.starts_with("easter_egg_") && a.id != "easter_egg_hunter")
            .count();
        if found >= HUNTER_THRESHOLD {
            return Self::unlock_if_new(game_state, "easter_egg_hunter", i18n, 250);
        }

        None
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
        
        game_state.push_achievement(achievement.clone());
        Some(achievement)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::state::GameState;
    use crate::i18n::{I18n, Language};

    fn setup() -> (GameState, I18n) {
        let i18n = I18n::new(Language::PtBr);
        let gs = GameState::new(&i18n);
        (gs, i18n)
    }

    #[test]
    fn first_command_then_first_ls_unlock_in_order() {
        let (mut gs, i18n) = setup();

        gs.total_commands = 1;
        assert!(AchievementChecker::check_command(&mut gs, "ls", true, &i18n).is_some());
        assert!(gs.has_achievement("first_command"));

        // Próximo `ls` (total != 1) desbloqueia a conquista específica do comando.
        gs.total_commands = 2;
        assert!(AchievementChecker::check_command(&mut gs, "ls -la", true, &i18n).is_some());
        assert!(gs.has_achievement("first_ls"));
    }

    #[test]
    fn achievements_never_unlock_twice() {
        let (mut gs, i18n) = setup();
        gs.total_commands = 5;
        assert!(AchievementChecker::check_command(&mut gs, "git status", true, &i18n).is_some());
        let count = gs.achievements.len();
        // Repetir o mesmo comando não cria conquista duplicada.
        assert!(AchievementChecker::check_command(&mut gs, "git log", true, &i18n).is_none());
        assert_eq!(gs.achievements.len(), count);
    }

    #[test]
    fn count_milestone_is_awarded_even_when_command_achievement_exists() {
        let (mut gs, i18n) = setup();
        // Pré-desbloqueia first_ls para forçar o fall-through até o marco de contagem.
        gs.total_commands = 1;
        let _ = AchievementChecker::check_command(&mut gs, "ls", true, &i18n); // first_command
        gs.total_commands = 2;
        let _ = AchievementChecker::check_command(&mut gs, "ls", true, &i18n); // first_ls
        // 10º comando sendo `ls` (já tem first_ls) ainda deve conceder commands_10.
        gs.total_commands = 10;
        assert!(AchievementChecker::check_command(&mut gs, "ls", true, &i18n).is_some());
        assert!(gs.has_achievement("commands_10"));
    }
}
