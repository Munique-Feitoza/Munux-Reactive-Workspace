// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::cell::OnceCell;
use std::collections::HashSet;

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

    /// Índice `id -> presente` das conquistas, para consulta O(1).
    /// Derivado de `achievements`, então não é serializado nem comparado.
    #[serde(skip)]
    achievement_ids: OnceCell<HashSet<String>>,
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
            achievement_ids: OnceCell::new(),
        }
    }
    
    /// Adiciona XP e sobe **todos** os níveis alcançados, devolvendo `true` se
    /// houve pelo menos um level up.
    ///
    /// Antes o `if` subia um único nível por chamada: uma concessão grande
    /// (o easter egg `rm -rf /` dá 666 XP; o cheat `xp` de debug, qualquer valor)
    /// deixava o XP muito acima do limiar e a pessoa destravava um nível por
    /// comando até drenar a sobra. Com `add_xp(10_000)` no nível 1 o estado
    /// ficava em `level=2, xp=9900, limiar=120`.
    pub fn add_xp(&mut self, amount: u32) -> bool {
        self.xp = self.xp.saturating_add(amount);

        let mut leveled = false;
        // `level_up` sempre desconta `xp_to_next_level` (> 0) e depois o
        // multiplica por 1.2, então `xp` cai a cada volta e o laço termina.
        while self.xp >= self.xp_to_next_level && self.xp_to_next_level > 0 {
            self.level_up();
            leveled = true;
        }

        leveled
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
    
    /// Retorna o título/rank baseado no nível
    pub fn get_rank(&self, i18n: &crate::i18n::I18n) -> String {
        i18n.rank_name(self.level)
    }

    /// Progresso de XP rumo ao próximo nível, em `0.0..=100.0`.
    /// Protege contra divisão por zero (`xp_to_next_level == 0` => 100%).
    pub fn xp_progress(&self) -> f64 {
        if self.xp_to_next_level == 0 {
            return 100.0;
        }
        ((self.xp as f64 / self.xp_to_next_level as f64) * 100.0).clamp(0.0, 100.0)
    }

    /// Incrementa o contador de comandos
    pub fn increment_commands(&mut self) {
        self.total_commands += 1;
    }
    
    /// Verifica se tem uma conquista específica. O(1) amortizado.
    ///
    /// O `Vec<Achievement>` é a forma serializada (ordem = ordem de desbloqueio,
    /// que a UI usa para "conquistas recentes"); o `HashSet` de ids é o índice
    /// de consulta, reconstruído sob demanda. Cada comando dispara até 15
    /// verificações, e antes cada uma varria o vetor inteiro.
    pub fn has_achievement(&self, id: &str) -> bool {
        self.achievement_ids
            .get_or_init(|| self.achievements.iter().map(|a| a.id.clone()).collect())
            .contains(id)
    }

    /// Registra uma conquista desbloqueada, mantendo o índice coerente.
    /// Único ponto que faz `push` em `achievements`.
    pub fn push_achievement(&mut self, achievement: Achievement) {
        self.achievements.push(achievement);
        // Invalida o índice: será remontado na próxima consulta.
        self.achievement_ids.take();
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
    
    /// Atualiza o streak diário com base na última sessão salva.
    ///
    /// - Mesma data: mantém o streak.
    /// - Dia seguinte: incrementa o streak.
    /// - Mais de um dia: reinicia o streak em 1.
    pub fn update_daily_streak(&mut self) {
        let today = Utc::now().date_naive();
        let last = self.last_session.date_naive();
        let diff = (today - last).num_days();

        if diff == 1 {
            self.daily_streak += 1;
        } else if diff > 1 {
            self.daily_streak = 1;
        }

        self.last_session = Utc::now();
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

#[cfg(test)]
mod tests {
    use super::*;

    fn state() -> GameState {
        GameState::default()
    }

    #[test]
    fn add_xp_levels_up_and_carries_over() {
        let mut s = state();
        assert_eq!((s.level, s.xp_to_next_level), (1, 100));
        assert!(s.add_xp(100));
        assert_eq!(s.level, 2);
        assert_eq!(s.xp, 0);
        assert_eq!(s.xp_to_next_level, 120); // +20% por nível
    }

    #[test]
    fn add_xp_climbs_every_level_the_grant_covers() {
        let mut s = state();
        // 100 + 120 + 144 = 364 XP cobrem exatamente 3 níveis (1 -> 4).
        assert!(s.add_xp(364));
        assert_eq!(s.level, 4);
        assert_eq!(s.xp, 0);

        // Uma concessão enorme não pode deixar XP parado acima do limiar.
        let mut s = state();
        s.add_xp(10_000);
        assert!(
            s.xp < s.xp_to_next_level,
            "sobrou XP acima do limiar: xp={} limiar={}",
            s.xp,
            s.xp_to_next_level
        );
    }

    #[test]
    fn add_xp_below_threshold_does_not_level() {
        let mut s = state();
        assert!(!s.add_xp(50));
        assert_eq!(s.level, 1);
        assert_eq!(s.xp, 50);
    }

    #[test]
    fn level_five_disables_safe_mode() {
        let mut s = state();
        assert!(s.safe_mode);
        for _ in 0..4 {
            let need = s.xp_to_next_level;
            s.add_xp(need);
        }
        assert_eq!(s.level, 5);
        assert!(!s.safe_mode);
    }

    #[test]
    fn streak_resets_on_failure_and_success_rate_is_correct() {
        let mut s = state();
        s.increment_commands();
        s.record_success();
        s.increment_commands();
        s.record_success();
        assert_eq!(s.command_streak, 2);
        assert_eq!(s.successful_commands, 2);

        s.increment_commands();
        s.record_failure();
        assert_eq!(s.command_streak, 0);
        assert_eq!(s.failed_commands, 1);

        assert!((s.success_rate() - 66.666).abs() < 0.1); // 2/3
    }

    /// O índice O(1) é um cache derivado; se ele não for invalidado a cada
    /// desbloqueio, uma conquista nova ficaria invisível para `has_achievement`
    /// e poderia ser concedida duas vezes.
    #[test]
    fn achievement_index_stays_in_sync_with_the_vector() {
        let mut s = state();
        assert!(!s.has_achievement("first_ls")); // consulta -> índice materializa

        s.push_achievement(Achievement {
            id: "first_ls".into(),
            name: "Olho do Tigre".into(),
            description: "Use 'ls'".into(),
            unlocked_at: Utc::now(),
            xp_reward: 15,
        });

        assert!(s.has_achievement("first_ls"), "índice ficou obsoleto após o push");
        assert!(!s.has_achievement("first_cd"));
        assert_eq!(s.achievements.len(), 1);
    }

    /// O índice não é serializado: precisa ser reconstruído a partir do vetor
    /// ao carregar um save.
    #[test]
    fn achievement_index_rebuilds_after_deserialization() {
        let mut s = state();
        s.push_achievement(Achievement {
            id: "first_git".into(),
            name: "n".into(),
            description: "d".into(),
            unlocked_at: Utc::now(),
            xp_reward: 35,
        });

        let json = serde_json::to_string(&s).unwrap();
        let restored: GameState = serde_json::from_str(&json).unwrap();

        assert!(restored.has_achievement("first_git"));
        assert!(!restored.has_achievement("first_ls"));
    }

    #[test]
    fn success_rate_zero_when_no_commands() {
        assert_eq!(state().success_rate(), 0.0);
    }

    #[test]
    fn daily_streak_increments_next_day_and_resets_after_gap() {
        let mut s = state();

        s.daily_streak = 3;
        s.last_session = Utc::now() - chrono::Duration::days(1);
        s.update_daily_streak();
        assert_eq!(s.daily_streak, 4, "dia seguinte deve incrementar");

        s.daily_streak = 3;
        s.last_session = Utc::now() - chrono::Duration::days(5);
        s.update_daily_streak();
        assert_eq!(s.daily_streak, 1, "lacuna > 1 dia deve reiniciar");
    }
}
