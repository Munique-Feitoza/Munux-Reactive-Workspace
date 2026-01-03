// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use serde::{Deserialize, Serialize};

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
}

impl Quest {
    /// Verifica se a quest está completa
    pub fn is_complete(&self) -> bool {
        match &self.objective {
            QuestObjective::ExecuteCommand { count, current, .. } => current >= count,
            QuestObjective::CreateFile { done, .. } => *done,
            QuestObjective::CreateDirectory { done, .. } => *done,
            QuestObjective::NavigateTo { done, .. } => *done,
            QuestObjective::ReadFile { done, .. } => *done,
            QuestObjective::DeleteFile { done, .. } => *done,
            QuestObjective::ReachLevel { level } => false, // Verificado externamente
            QuestObjective::ExecuteAnyCommands { count, current } => current >= count,
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
                    return !was_complete && self.is_complete();
                }
            }
            QuestObjective::ExecuteAnyCommands { current, .. } => {
                *current += 1;
            }
        }
        
        if self.is_complete() && !self.completed {
            self.completed = true;
            return true;
        }
        
        false
    }
    
    /// Retorna o progresso em texto
    pub fn get_progress_text(&self) -> String {
        match &self.objective {
            QuestObjective::ExecuteCommand { command, count, current } => {
                format!("Execute '{}' {}/{} vezes", command, current, count)
            }
            QuestObjective::CreateFile { name, done } => {
                if *done {
                    format!("✓ Arquivo '{}' criado", name)
                } else {
                    format!("Crie um arquivo chamado '{}'", name)
                }
            }
            QuestObjective::CreateDirectory { name, done } => {
                if *done {
                    format!("✓ Diretório '{}' criado", name)
                } else {
                    format!("Crie um diretório chamado '{}'", name)
                }
            }
            QuestObjective::NavigateTo { path, done } => {
                if *done {
                    format!("✓ Navegou para '{}'", path)
                } else {
                    format!("Navegue para '{}'", path)
                }
            }
            QuestObjective::ReadFile { name, done } => {
                if *done {
                    format!("✓ Arquivo '{}' lido", name)
                } else {
                    format!("Leia o arquivo '{}'", name)
                }
            }
            QuestObjective::DeleteFile { name, done } => {
                if *done {
                    format!("✓ Arquivo '{}' deletado", name)
                } else {
                    format!("Delete o arquivo '{}'", name)
                }
            }
            QuestObjective::ReachLevel { level } => {
                format!("Alcance o nível {}", level)
            }
            QuestObjective::ExecuteAnyCommands { count, current } => {
                format!("Execute comandos {}/{}", current, count)
            }
        }
    }
}

/// Gera quests baseadas no nível do jogador
pub fn generate_quests_for_level(level: u32) -> Vec<Quest> {
    let mut quests = Vec::new();
    
    // Quests para iniciantes (nível 1-3)
    if level <= 3 {
        quests.push(Quest {
            id: "tutorial_ls".to_string(),
            title: "Primeiros Passos".to_string(),
            description: "Liste os arquivos do diretório atual".to_string(),
            objective: QuestObjective::ExecuteCommand {
                command: "ls".to_string(),
                count: 1,
                current: 0,
            },
            xp_reward: 20,
            completed: false,
        });
        
        quests.push(Quest {
            id: "tutorial_create_file".to_string(),
            title: "Criador de Mundos".to_string(),
            description: "Crie seu primeiro arquivo".to_string(),
            objective: QuestObjective::CreateFile {
                name: "hello.txt".to_string(),
                done: false,
            },
            xp_reward: 30,
            completed: false,
        });
    }
    
    // Quests intermediárias (nível 4-9)
    if level >= 4 && level <= 9 {
        quests.push(Quest {
            id: "intermediate_mkdir".to_string(),
            title: "Arquiteto Digital".to_string(),
            description: "Crie um diretório para organizar seus arquivos".to_string(),
            objective: QuestObjective::CreateDirectory {
                name: "projeto".to_string(),
                done: false,
            },
            xp_reward: 40,
            completed: false,
        });
        
        quests.push(Quest {
            id: "intermediate_commands".to_string(),
            title: "Praticante".to_string(),
            description: "Execute 20 comandos diferentes".to_string(),
            objective: QuestObjective::ExecuteAnyCommands {
                count: 20,
                current: 0,
            },
            xp_reward: 100,
            completed: false,
        });
    }
    
    // Quests avançadas (nível 10+)
    if level >= 10 {
        quests.push(Quest {
            id: "advanced_level".to_string(),
            title: "Ascensão".to_string(),
            description: "Alcance o nível 15".to_string(),
            objective: QuestObjective::ReachLevel { level: 15 },
            xp_reward: 200,
            completed: false,
        });
    }
    
    quests
}
