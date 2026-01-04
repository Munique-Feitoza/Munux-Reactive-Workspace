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
    /// Verifica se a quest está completa
    pub fn is_complete(&self) -> bool {
        match &self.objective {
            QuestObjective::ExecuteCommand { count, current, .. } => current >= count,
            QuestObjective::CreateFile { done, .. } => *done,
            QuestObjective::CreateDirectory { done, .. } => *done,
            QuestObjective::NavigateTo { done, .. } => *done,
            QuestObjective::ReadFile { done, .. } => *done,
            QuestObjective::DeleteFile { done, .. } => *done,
            QuestObjective::ReachLevel { level: _ } => false, // Verificado externamente
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
                    return !was_complete && self.is_complete();
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
                if command.starts_with("pacman") || command.starts_with("apt") 
                   || command.starts_with("dnf") || command.starts_with("zypper")
                   || command.starts_with("flatpak") {
                    *done = true;
                }
            }
            QuestObjective::UseGrep { current, .. } => {
                if command.starts_with("grep") || command.starts_with("find") {
                    *current += 1;
                }
            }
            QuestObjective::UsePipe { done } => {
                if command.contains("|") {
                    *done = true;
                }
            }
            QuestObjective::CreateSymlink { done } => {
                if command.starts_with("ln -s") {
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
                if (command.starts_with("nano") || command.starts_with("vim") || command.starts_with("vi"))
                   && (command.contains(".sh") || command.contains(".bash")) {
                    *done = true;
                }
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
            QuestObjective::UseGit { done } => {
                if *done {
                    format!("✓ Configurou Git")
                } else {
                    format!("Configure Git")
                }
            }
            QuestObjective::UseSSH { done } => {
                if *done {
                    format!("✓ Usou SSH")
                } else {
                    format!("Use SSH para conectar")
                }
            }
            QuestObjective::UsePackageManager { done } => {
                if *done {
                    format!("✓ Instalou um pacote")
                } else {
                    format!("Instale um pacote")
                }
            }
            QuestObjective::UseGrep { count, current } => {
                format!("Use grep/find {}/{} vezes", current, count)
            }
            QuestObjective::UsePipe { done } => {
                if *done {
                    format!("✓ Usou pipes")
                } else {
                    format!("Use pipes (|)")
                }
            }
            QuestObjective::CreateSymlink { done } => {
                if *done {
                    format!("✓ Criou link simbólico")
                } else {
                    format!("Crie um link simbólico")
                }
            }
            QuestObjective::UseTextEditor { editor, done } => {
                if *done {
                    format!("✓ Usou {}", editor)
                } else {
                    format!("Use o editor {}", editor)
                }
            }
            QuestObjective::UseSystemctl { done } => {
                if *done {
                    format!("✓ Usou systemctl")
                } else {
                    format!("Use systemctl")
                }
            }
            QuestObjective::WriteScript { done } => {
                if *done {
                    format!("✓ Escreveu um script")
                } else {
                    format!("Escreva um shell script")
                }
            }
        }
    }
}

/// Gera quests baseadas no nível do jogador
pub fn generate_quests_for_level(level: u32) -> Vec<Quest> {
    let mut quests = Vec::new();
    
    // 🌱 BEGINNER (Níveis 1-9) - Comandos Básicos
    if level >= 1 && level <= 9 {
        quests.push(Quest {
            id: "beginner_ls".to_string(),
            title: "Primeiros Passos".to_string(),
            description: "Execute seu primeiro ls".to_string(),
            objective: QuestObjective::ExecuteCommand {
                command: "ls".to_string(),
                count: 1,
                current: 0,
            },
            xp_reward: 20,
            completed: false,
        });
        
        quests.push(Quest {
            id: "beginner_create_file".to_string(),
            title: "Criador".to_string(),
            description: "Crie um arquivo com touch".to_string(),
            objective: QuestObjective::CreateFile {
                name: "hello.txt".to_string(),
                done: false,
            },
            xp_reward: 30,
            completed: false,
        });
        
        quests.push(Quest {
            id: "beginner_navigate".to_string(),
            title: "Navegador".to_string(),
            description: "Navegue para /home".to_string(),
            objective: QuestObjective::NavigateTo {
                path: "/home".to_string(),
                done: false,
            },
            xp_reward: 25,
            completed: false,
        });
    }
    
    // 💻 TERMINAL (Níveis 10-19) - Manipulação de Arquivos
    if level >= 10 && level <= 19 {
        quests.push(Quest {
            id: "terminal_grep".to_string(),
            title: "Buscador".to_string(),
            description: "Use grep para encontrar texto".to_string(),
            objective: QuestObjective::UseGrep {
                count: 3,
                current: 0,
            },
            xp_reward: 50,
            completed: false,
        });
        
        quests.push(Quest {
            id: "terminal_package".to_string(),
            title: "Gerente de Pacotes".to_string(),
            description: "Instale um pacote".to_string(),
            objective: QuestObjective::UsePackageManager {
                done: false,
            },
            xp_reward: 60,
            completed: false,
        });
        
        quests.push(Quest {
            id: "terminal_read".to_string(),
            title: "Leitor".to_string(),
            description: "Leia um arquivo com cat".to_string(),
            objective: QuestObjective::ReadFile {
                name: "README".to_string(),
                done: false,
            },
            xp_reward: 40,
            completed: false,
        });
    }
    
    // 🔓 HACKER (Níveis 20-29) - Git & SSH
    if level >= 20 && level <= 29 {
        quests.push(Quest {
            id: "hacker_git".to_string(),
            title: "Versionador".to_string(),
            description: "Configure Git".to_string(),
            objective: QuestObjective::UseGit {
                done: false,
            },
            xp_reward: 80,
            completed: false,
        });
        
        quests.push(Quest {
            id: "hacker_ssh".to_string(),
            title: "Conectador Remoto".to_string(),
            description: "Use SSH para conectar".to_string(),
            objective: QuestObjective::UseSSH {
                done: false,
            },
            xp_reward: 90,
            completed: false,
        });
        
        quests.push(Quest {
            id: "hacker_symlink".to_string(),
            title: "Mestre dos Links".to_string(),
            description: "Crie um link simbólico".to_string(),
            objective: QuestObjective::CreateSymlink {
                done: false,
            },
            xp_reward: 70,
            completed: false,
        });
        
        quests.push(Quest {
            id: "hacker_editor".to_string(),
            title: "Editor de Texto".to_string(),
            description: "Use nano ou vim".to_string(),
            objective: QuestObjective::UseTextEditor {
                editor: "nano".to_string(),
                done: false,
            },
            xp_reward: 75,
            completed: false,
        });
    }
    
    // 🌃 CYBERPUNK (Níveis 30-39) - Administração de Sistema
    if level >= 30 && level <= 39 {
        quests.push(Quest {
            id: "cyberpunk_systemctl".to_string(),
            title: "Administrador de Serviços".to_string(),
            description: "Use systemctl".to_string(),
            objective: QuestObjective::UseSystemctl {
                done: false,
            },
            xp_reward: 120,
            completed: false,
        });
        
        quests.push(Quest {
            id: "cyberpunk_pipes".to_string(),
            title: "Mestre dos Pipes".to_string(),
            description: "Use pipes (|) em comandos".to_string(),
            objective: QuestObjective::UsePipe {
                done: false,
            },
            xp_reward: 100,
            completed: false,
        });
        
        quests.push(Quest {
            id: "cyberpunk_script".to_string(),
            title: "Scriptador".to_string(),
            description: "Escreva um shell script".to_string(),
            objective: QuestObjective::WriteScript {
                done: false,
            },
            xp_reward: 150,
            completed: false,
        });
    }
    
    // 👑 ELITE (Níveis 40-49) - Tarefas Avançadas
    if level >= 40 && level <= 49 {
        quests.push(Quest {
            id: "elite_master".to_string(),
            title: "Mestre Terminal".to_string(),
            description: "Execute 500 comandos".to_string(),
            objective: QuestObjective::ExecuteAnyCommands {
                count: 500,
                current: 0,
            },
            xp_reward: 500,
            completed: false,
        });
        
        quests.push(Quest {
            id: "elite_ascension".to_string(),
            title: "Ascensão".to_string(),
            description: "Alcance o nível 50".to_string(),
            objective: QuestObjective::ReachLevel {
                level: 50,
            },
            xp_reward: 1000,
            completed: false,
        });
    }
    
    // ⭐ LEGEND (Nível 50+) - Sem mais quests, você é o mestre!
    if level >= 50 {
        quests.push(Quest {
            id: "legend_status".to_string(),
            title: "Lenda do Terminal".to_string(),
            description: "Você dominou todos os comandos!".to_string(),
            objective: QuestObjective::ExecuteAnyCommands {
                count: 0,
                current: 0,
            },
            xp_reward: 0,
            completed: true,
        });
    }
    
    quests
}
