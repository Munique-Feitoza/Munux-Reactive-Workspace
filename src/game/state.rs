// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Estado de gamificação do usuário
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    /// Experiência (XP) atual
    pub xp: u32,
    
    /// Nível atual do usuário
    pub level: u32,
    
    /// XP necessário para o próximo nível
    pub xp_to_next_level: u32,
    
    /// Integridade do sistema (0-100)
    pub integrity: u8,
    
    /// Modo de segurança (bloqueia comandos perigosos em níveis baixos)
    pub safe_mode: bool,
    
    /// Conquistas desbloqueadas
    pub achievements: Vec<Achievement>,
    
    /// Timestamp da criação da conta
    pub created_at: DateTime<Utc>,
    
    /// Timestamp da última sessão
    pub last_session: DateTime<Utc>,
    
    /// Total de comandos executados
    pub total_commands: u32,
    
    /// Streak de dias consecutivos
    pub daily_streak: u32,
    
    /// Streak de comandos corretos seguidos
    pub command_streak: u32,
    
    /// Última conquista desbloqueada (para mostrar notificação)
    pub last_achievement: Option<Achievement>,
    
    /// Quests ativas
    pub active_quests: Vec<crate::game::quests::Quest>,
    
    /// Comandos executados com sucesso
    pub successful_commands: u32,
    
    /// Comandos que falharam
    pub failed_commands: u32,
}

/// Conquista desbloqueada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub unlocked_at: DateTime<Utc>,
    pub xp_reward: u32,
}

impl GameState {
    /// Cria um novo estado de jogo
    pub fn new(i18n: &crate::i18n::I18n) -> Self {
        Self {
            xp: 0,
            level: 1,
            xp_to_next_level: 100,
            integrity: 100,
            safe_mode: true,
            achievements: Vec::new(),
            created_at: Utc::now(),
            last_session: Utc::now(),
            total_commands: 0,
            daily_streak: 1,
            command_streak: 0,
            last_achievement: None,
            active_quests: crate::game::quests::generate_quests_for_level(1, i18n),
            successful_commands: 0,
            failed_commands: 0,
        }
    }
    
    /// Adiciona XP e verifica level up
    pub fn add_xp(&mut self, amount: u32) -> bool {
        self.xp += amount;
        
        // Verifica se subiu de nível
        if self.xp >= self.xp_to_next_level {
            self.level_up();
            return true;
        }
        
        false
    }
    
    /// Sobe de nível
    fn level_up(&mut self) {
        self.level += 1;
        self.xp = self.xp.saturating_sub(self.xp_to_next_level);
        
        // Fórmula progressiva: cada nível precisa de 20% a mais de XP
        self.xp_to_next_level = (self.xp_to_next_level as f32 * 1.2) as u32;
        
        // Desbloqueia comandos perigosos no nível 5
        if self.level >= 5 {
            self.safe_mode = false;
        }
        
        // Recupera integridade ao subir de nível
        self.integrity = (self.integrity + 20).min(100);
    }
    
    /// Reduz a integridade do sistema (erros, comandos perigosos)
    pub fn damage_integrity(&mut self, amount: u8) {
        self.integrity = self.integrity.saturating_sub(amount);
    }
    
    /// Restaura integridade
    pub fn restore_integrity(&mut self, amount: u8) {
        self.integrity = (self.integrity + amount).min(100);
    }
    
    /// Adiciona uma conquista
    #[allow(dead_code)]
    pub fn unlock_achievement(&mut self, id: String, name: String, description: String, xp_reward: u32) {
        // Verifica se já foi desbloqueada
        if self.achievements.iter().any(|a| a.id == id) {
            return;
        }
        
        let achievement = Achievement {
            id,
            name,
            description,
            unlocked_at: Utc::now(),
            xp_reward,
        };
        
        self.achievements.push(achievement);
        self.add_xp(xp_reward);
    }
    
    /// Retorna o título/rank baseado no nível
    pub fn get_rank(&self, i18n: &crate::i18n::I18n) -> String {
        i18n.rank_name(self.level)
    }
    
    /// Retorna a cor do nível (para o prompt)
    #[allow(dead_code)]
    pub fn get_level_color(&self) -> ratatui::style::Color {
        use ratatui::style::Color;
        
        match self.level {
            1..=4 => Color::White,
            5..=9 => Color::Green,
            10..=19 => Color::Cyan,
            20..=29 => Color::Blue,
            30..=49 => Color::Magenta,
            _ => Color::LightMagenta,
        }
    }
    
    /// Incrementa o contador de comandos
    pub fn increment_commands(&mut self) {
        self.total_commands += 1;
    }
    
    /// Verifica se tem uma conquista específica
    pub fn has_achievement(&self, id: &str) -> bool {
        self.achievements.iter().any(|a| a.id == id)
    }
    
    /// Registra comando bem-sucedido
    pub fn record_success(&mut self) {
        self.successful_commands += 1;
        self.command_streak += 1;
    }
    
    /// Registra comando que falhou
    pub fn record_failure(&mut self) {
        self.failed_commands += 1;
        self.command_streak = 0;
    }
    
    /// Retorna taxa de acerto
    pub fn success_rate(&self) -> f64 {
        if self.total_commands == 0 {
            return 0.0;
        }
        (self.successful_commands as f64 / self.total_commands as f64) * 100.0
    }
    
    /// Gera novas quests quando sobe de nível
    pub fn refresh_quests(&mut self, i18n: &crate::i18n::I18n) {
        // Remove quests completadas
        self.active_quests.retain(|q| !q.completed);
        
        // Adiciona novas quests para o nível atual
        let new_quests = crate::game::quests::generate_quests_for_level(self.level, i18n);
        for quest in new_quests {
            // Só adiciona se não existir uma quest com o mesmo ID
            if !self.active_quests.iter().any(|q| q.id == quest.id) {
                self.active_quests.push(quest);
            }
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new(&crate::i18n::I18n::new(crate::i18n::Language::PtBr))
    }
}
