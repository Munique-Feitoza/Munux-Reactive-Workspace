// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use serde::{Deserialize, Serialize};
use crate::i18n::I18n;

/// Quest/Missão para o usuário completar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quest {
    pub id: String,
    pub title: String,
    pub description: String,
    pub objective: QuestObjective,
    pub xp_reward: u32,
    pub completed: bool,
}

/// Objetivo da quest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestObjective {
    ExecuteCommand { command: String, count: u32, current: u32 },
    CreateFile { name: String, done: bool },
    CreateDirectory { name: String, done: bool },
    NavigateTo { path: String, done: bool },
    ReadFile { name: String, done: bool },
    DeleteFile { name: String, done: bool },
    ReachLevel { level: u32 },
    ExecuteAnyCommands { count: u32, current: u32 },
    UseGit { done: bool },
    UseSSH { done: bool },
    UsePackageManager { done: bool },
    UseGrep { count: u32, current: u32 },
    UsePipe { done: bool },
    CreateSymlink { done: bool },
    UseTextEditor { editor: String, done: bool },
    UseSystemctl { done: bool },
    WriteScript { done: bool },
}

impl Quest {
    /// Retorna o progresso em texto
    pub fn get_progress_text(&self, i18n: &I18n) -> String {
        use fluent::{FluentArgs, FluentValue};
        let mut args = FluentArgs::new();

        match &self.objective {
            QuestObjective::ExecuteCommand { command, count, current } => {
                args.set("command", FluentValue::from(command.as_str()));
                args.set("current", FluentValue::from(*current));
                args.set("count", FluentValue::from(*count));
                i18n.t("quest-progress-run", Some(&args))
            }
            QuestObjective::CreateFile { name, done } => {
                args.set("name", FluentValue::from(name.as_str()));
                args.set("item", FluentValue::from("file"));
                if *done {
                    i18n.t("quest-progress-created", Some(&args))
                } else {
                    i18n.t("quest-progress-create", Some(&args))
                }
            }
            QuestObjective::CreateDirectory { name, done } => {
                args.set("name", FluentValue::from(name.as_str()));
                args.set("item", FluentValue::from("folder"));
                if *done {
                    i18n.t("quest-progress-created", Some(&args))
                } else {
                    i18n.t("quest-progress-create", Some(&args))
                }
            }
            QuestObjective::NavigateTo { path, done } => {
                args.set("path", FluentValue::from(path.as_str()));
                if *done {
                    i18n.t("quest-progress-navigated", Some(&args))
                } else {
                    i18n.t("quest-progress-navigate", Some(&args))
                }
            }
            QuestObjective::ReadFile { name, done } => {
                args.set("name", FluentValue::from(name.as_str()));
                if *done {
                    i18n.t("quest-progress-read", Some(&args))
                } else {
                    i18n.t("quest-progress-read-action", Some(&args))
                }
            }
            QuestObjective::DeleteFile { name, done } => {
                args.set("name", FluentValue::from(name.as_str()));
                if *done {
                    i18n.t("quest-progress-deleted", Some(&args))
                } else {
                    i18n.t("quest-progress-delete-action", Some(&args))
                }
            }
            QuestObjective::ReachLevel { level } => {
                args.set("level", FluentValue::from(*level));
                i18n.t("quest-progress-reach-level", Some(&args))
            }
            QuestObjective::ExecuteAnyCommands { count, current } => {
                args.set("current", FluentValue::from(*current));
                args.set("count", FluentValue::from(*count));
                i18n.t("quest-progress-any-command", Some(&args))
            }
            QuestObjective::UseGit { done } => {
                if *done { i18n.tc("quest-progress-git-done") }
                else { i18n.tc("quest-progress-git-todo") }
            }
            QuestObjective::UseSSH { done } => {
                if *done { i18n.tc("quest-progress-ssh-done") }
                else { i18n.tc("quest-progress-ssh-todo") }
            }
            QuestObjective::UsePackageManager { done } => {
                if *done { i18n.tc("quest-progress-pkg-done") }
                else { i18n.tc("quest-progress-pkg-todo") }
            }
            QuestObjective::UseGrep { count, current } => {
                args.set("current", FluentValue::from(*current));
                args.set("count", FluentValue::from(*count));
                i18n.t("quest-progress-grep", Some(&args))
            }
            QuestObjective::UsePipe { done } => {
                if *done { i18n.tc("quest-progress-pipe-done") }
                else { i18n.tc("quest-progress-pipe-todo") }
            }
            QuestObjective::CreateSymlink { done } => {
                if *done { i18n.tc("quest-progress-symlink-done") }
                else { i18n.tc("quest-progress-symlink-todo") }
            }
            QuestObjective::UseTextEditor { editor, done } => {
                args.set("editor", FluentValue::from(editor.as_str()));
                if *done { i18n.t("quest-progress-editor-done", Some(&args)) }
                else { i18n.t("quest-progress-editor-todo", Some(&args)) }
            }
            QuestObjective::UseSystemctl { done } => {
                if *done { i18n.tc("quest-progress-systemctl-done") }
                else { i18n.tc("quest-progress-systemctl-todo") }
            }
            QuestObjective::WriteScript { done } => {
                if *done { i18n.tc("quest-progress-script-done") }
                else { i18n.tc("quest-progress-script-todo") }
            }
        }
    }

    /// Verifica se a quest está completa
    pub fn is_complete(&self) -> bool {
        match &self.objective {
            QuestObjective::ExecuteCommand { count, current, .. } => current >= count,
            QuestObjective::CreateFile { done, .. } => *done,
            QuestObjective::CreateDirectory { done, .. } => *done,
            QuestObjective::NavigateTo { done, .. } => *done,
            QuestObjective::ReadFile { done, .. } => *done,
            QuestObjective::DeleteFile { done, .. } => *done,
            QuestObjective::ReachLevel { .. } => false, // Verificado externamente
            QuestObjective::ExecuteAnyCommands { count, current } => current >= count,
            QuestObjective::UseGit { done } => *done,
            QuestObjective::UseSSH { done } => *done,
            QuestObjective::UsePackageManager { done } => *done,
            QuestObjective::UseGrep { count, current } => current >= count,
            QuestObjective::UsePipe { done } => *done,
            QuestObjective::CreateSymlink { done } => *done,
            QuestObjective::UseTextEditor { done, .. } => *done,
            QuestObjective::UseSystemctl { done } => *done,
            QuestObjective::WriteScript { done } => *done,
        }
    }

    /// Atualiza o progresso da quest
    pub fn update_progress(&mut self, command: &str, current_level: u32) -> bool {
        let was_complete = self.is_complete();
        
        match &mut self.objective {
            QuestObjective::ExecuteCommand { command: cmd, current, .. } => {
                if command.starts_with(cmd.as_str()) {
                    *current += 1;
                }
            }
            QuestObjective::CreateFile { name, done } => {
                if command.starts_with("touch") && command.contains(name.as_str()) {
                    *done = true;
                }
            }
            QuestObjective::CreateDirectory { name, done } => {
                if command.starts_with("mkdir") && command.contains(name.as_str()) {
                    *done = true;
                }
            }
            QuestObjective::NavigateTo { path, done } => {
                if command.starts_with("cd") && command.contains(path.as_str()) {
                    *done = true;
                }
            }
            QuestObjective::ReadFile { name, done } => {
                if command.starts_with("cat") && command.contains(name.as_str()) {
                    *done = true;
                }
            }
            QuestObjective::DeleteFile { name, done } => {
                if command.starts_with("rm") && command.contains(name.as_str()) {
                    *done = true;
                }
            }
            QuestObjective::ReachLevel { level } => {
                if current_level >= *level {
                    self.completed = true;
                    return true;
                }
            }
            QuestObjective::ExecuteAnyCommands { current, .. } => {
                *current += 1;
            }
            QuestObjective::UseGit { done } => {
                if command.starts_with("git") {
                    *done = true;
                }
            }
            QuestObjective::UseSSH { done } => {
                if command.starts_with("ssh") {
                    *done = true;
                }
            }
            QuestObjective::UsePackageManager { done } => {
                if command.starts_with("pacman") || command.starts_with("apt") || command.starts_with("yay") {
                    *done = true;
                }
            }
            QuestObjective::UseGrep { current, .. } => {
                if command.starts_with("grep") {
                    *current += 1;
                }
            }
            QuestObjective::UsePipe { done } => {
                if command.contains('|') {
                    *done = true;
                }
            }
            QuestObjective::CreateSymlink { done } => {
                if command.starts_with("ln") && command.contains("-s") {
                    *done = true;
                }
            }
            QuestObjective::UseTextEditor { editor, done } => {
                if command.starts_with(editor.as_str()) {
                    *done = true;
                }
            }
            QuestObjective::UseSystemctl { done } => {
                if command.starts_with("systemctl") {
                    *done = true;
                }
            }
            QuestObjective::WriteScript { done } => {
                if command.ends_with(".sh") && (command.starts_with("./") || command.starts_with("bash")) {
                    *done = true;
                }
            }
        }
        
        if self.is_complete() && !was_complete {
            self.completed = true;
            return true;
        }
        
        false
    }
}

/// Gera as quests iniciais ou novas quests baseadas no nível
pub fn generate_quests_for_level(level: u32, i18n: &I18n) -> Vec<Quest> {
    let mut quests = Vec::new();
    
    match level {
        1..=4 => {
            quests.push(Quest {
                id: "intro_ls".to_string(),
                title: i18n.tc("quest-explorer-title"),
                description: i18n.tc("quest-explorer-desc"),
                objective: QuestObjective::ExecuteCommand {
                    command: "ls".to_string(),
                    count: 1,
                    current: 0,
                },
                xp_reward: 20,
                completed: false,
            });
            
            quests.push(Quest {
                id: "intro_pwd".to_string(),
                title: i18n.tc("quest-location-title"),
                description: i18n.tc("quest-location-desc"),
                objective: QuestObjective::ExecuteCommand {
                    command: "pwd".to_string(),
                    count: 1,
                    current: 0,
                },
                xp_reward: 15,
                completed: false,
            });

            quests.push(Quest {
                id: "intro_mkdir".to_string(),
                title: i18n.tc("quest-architect-title"),
                description: i18n.tc("quest-architect-desc"),
                objective: QuestObjective::CreateDirectory {
                    name: "munux".to_string(),
                    done: false,
                },
                xp_reward: 30,
                completed: false,
            });
        }
        5..=9 => {
            quests.push(Quest {
                id: "apprentice_cat".to_string(),
                title: i18n.tc("quest-reader-title"),
                description: i18n.tc("quest-reader-desc"),
                objective: QuestObjective::ReadFile {
                    name: "README".to_string(),
                    done: false,
                },
                xp_reward: 40,
                completed: false,
            });

            quests.push(Quest {
                id: "apprentice_rm".to_string(),
                title: i18n.tc("quest-cleaner-title"),
                description: i18n.tc("quest-cleaner-desc"),
                objective: QuestObjective::DeleteFile {
                    name: "tmp".to_string(),
                    done: false,
                },
                xp_reward: 35,
                completed: false,
            });
        }
        _ => {
            quests.push(Quest {
                id: "xp_grind".to_string(),
                title: i18n.tc("quest-focus-title"),
                description: i18n.tc("quest-focus-desc"),
                objective: QuestObjective::ExecuteAnyCommands {
                    count: 10,
                    current: 0,
                },
                xp_reward: 50,
                completed: false,
            });
        }
    }

    quests
}

#[cfg(test)]
mod tests {
    use super::*;

    fn quest(objective: QuestObjective) -> Quest {
        Quest {
            id: "t".to_string(),
            title: "t".to_string(),
            description: "t".to_string(),
            objective,
            xp_reward: 10,
            completed: false,
        }
    }

    #[test]
    fn execute_command_completes_on_matching_prefix() {
        let mut q = quest(QuestObjective::ExecuteCommand {
            command: "ls".to_string(),
            count: 1,
            current: 0,
        });
        assert!(q.update_progress("ls -la", 1));
        assert!(q.completed);
    }

    #[test]
    fn execute_command_ignores_non_matching() {
        let mut q = quest(QuestObjective::ExecuteCommand {
            command: "ls".to_string(),
            count: 1,
            current: 0,
        });
        assert!(!q.update_progress("cd /tmp", 1));
        assert!(!q.completed);
    }

    #[test]
    fn execute_any_commands_counts_up_to_target() {
        let mut q = quest(QuestObjective::ExecuteAnyCommands { count: 3, current: 0 });
        assert!(!q.update_progress("ls", 1));
        assert!(!q.update_progress("pwd", 1));
        assert!(q.update_progress("cd x", 1)); // 3º comando completa
        assert!(q.completed);
    }

    #[test]
    fn reach_level_completes_at_threshold() {
        let mut q = quest(QuestObjective::ReachLevel { level: 5 });
        assert!(!q.update_progress("seja o que for", 4));
        assert!(q.update_progress("seja o que for", 5));
        assert!(q.completed);
    }

    #[test]
    fn pipe_and_git_are_detected() {
        let mut p = quest(QuestObjective::UsePipe { done: false });
        assert!(p.update_progress("ps aux | grep init", 1));

        let mut g = quest(QuestObjective::UseGit { done: false });
        assert!(g.update_progress("git status", 1));
    }

    #[test]
    fn completion_triggers_only_once() {
        let mut q = quest(QuestObjective::UseGit { done: false });
        assert!(q.update_progress("git status", 1)); // transição -> true
        assert!(!q.update_progress("git log", 1)); // já completa -> false
    }
}
