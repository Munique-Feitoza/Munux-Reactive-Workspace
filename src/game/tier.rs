// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

//! Patente do jogador, derivada do nível.
//!
//! Fonte única das faixas de progressão. Antes os cortes viviam duplicados (e
//! divergentes) entre a patente (`rank_name`) e o visual (tema/símbolo/borda):
//! um jogador nível 45 era "Legend" mas ainda via o tema "Elite". Agora tudo
//! deriva daqui.

/// Patente progressiva. Cada degrau representa um marco real de aprendizado.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    Novice,     // níveis 1-9
    Apprentice, // 10-19
    Hacker,     // 20-29
    Elite,      // 30-39
    Legend,     // 40+
}

impl Tier {
    /// Patente correspondente a um nível.
    pub fn from_level(level: u32) -> Self {
        match level {
            0..=9 => Tier::Novice,
            10..=19 => Tier::Apprentice,
            20..=29 => Tier::Hacker,
            30..=39 => Tier::Elite,
            _ => Tier::Legend,
        }
    }

    /// Chave Fluent do nome da patente (em `locales/*/main.ftl`).
    pub fn rank_key(self) -> &'static str {
        match self {
            Tier::Novice => "game-rank-novice",
            Tier::Apprentice => "game-rank-apprentice",
            Tier::Hacker => "game-rank-hacker",
            Tier::Elite => "game-rank-elite",
            Tier::Legend => "game-rank-legend",
        }
    }

    /// Primeiro nível desta patente (onde a pessoa "chegou").
    pub fn min_level(self) -> u32 {
        match self {
            Tier::Novice => 1,
            Tier::Apprentice => 10,
            Tier::Hacker => 20,
            Tier::Elite => 30,
            Tier::Legend => 40,
        }
    }

    /// Próxima patente, se houver (`None` na última).
    pub fn next(self) -> Option<Tier> {
        match self {
            Tier::Novice => Some(Tier::Apprentice),
            Tier::Apprentice => Some(Tier::Hacker),
            Tier::Hacker => Some(Tier::Elite),
            Tier::Elite => Some(Tier::Legend),
            Tier::Legend => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier_boundaries_are_consistent() {
        assert_eq!(Tier::from_level(1), Tier::Novice);
        assert_eq!(Tier::from_level(9), Tier::Novice);
        assert_eq!(Tier::from_level(10), Tier::Apprentice);
        assert_eq!(Tier::from_level(29), Tier::Hacker);
        assert_eq!(Tier::from_level(30), Tier::Elite);
        // O corte que estava divergente: 40+ é Legend (antes o tema só virava em 50).
        assert_eq!(Tier::from_level(40), Tier::Legend);
        assert_eq!(Tier::from_level(45), Tier::Legend);
    }

    /// O estágio visual (6 degraus) precisa **refinar** a patente (5 degraus):
    /// todo nível de um mesmo estágio tem que cair na mesma patente. É esta
    /// guarda que impede as duas tabelas de voltarem a divergir — foi assim que
    /// nasceu o bug em que quem estava no nível 10 já era Aprendiz mas ainda
    /// recebia as dicas de iniciante.
    #[test]
    fn stage_refines_tier() {
        use crate::ui::theme::Stage;
        use std::collections::HashMap;

        let mut tier_of_stage: HashMap<Stage, Tier> = HashMap::new();
        for level in 0..=120u32 {
            let stage = Stage::from_level(level);
            let tier = Tier::from_level(level);
            match tier_of_stage.get(&stage) {
                Some(expected) => assert_eq!(
                    *expected, tier,
                    "nível {}: estágio {:?} cai em duas patentes diferentes",
                    level, stage
                ),
                None => {
                    tier_of_stage.insert(stage, tier);
                }
            }
        }
        assert_eq!(tier_of_stage.len(), 6, "todos os estágios devem ser alcançáveis");
    }

    #[test]
    fn next_tier_starts_where_current_ends() {
        // O nível em que a próxima patente começa bate com o min_level dela.
        for t in [Tier::Novice, Tier::Apprentice, Tier::Hacker, Tier::Elite] {
            let next = t.next().unwrap();
            assert_eq!(Tier::from_level(next.min_level()), next);
        }
        assert!(Tier::Legend.next().is_none());
    }
}
