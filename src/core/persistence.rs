// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

//! Persistência de progresso entre sessões.
//!
//! Salva e carrega o estado de gamificação (XP, nível, conquistas, streak) e o
//! histórico de comandos em um arquivo JSON dentro do diretório de dados do
//! usuário (`$XDG_DATA_HOME/munux/state.json`, com fallback para
//! `~/.local/share/munux/state.json`).

use crate::game::state::GameState;
use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Limite de comandos guardados no histórico persistido (evita arquivos enormes).
const MAX_HISTORY: usize = 500;

/// Dados gravados em disco.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    /// Versão do formato de save (para migrações futuras).
    pub version: u32,
    /// Estado de gamificação completo.
    pub game_state: GameState,
    /// Histórico de comandos executados.
    #[serde(default)]
    pub command_history: Vec<String>,
    /// Aliases definidos pelo usuário (`nome` -> `comando`).
    #[serde(default)]
    pub aliases: HashMap<String, String>,
}

impl SaveData {
    /// Versão atual do formato.
    ///
    /// - **1** — formato inicial.
    /// - **2** — `QuestObjective` deixou de ter uma variante por tipo de quest
    ///   (ver `game::quests`). Migrado por [`migrate`].
    pub const CURRENT_VERSION: u32 = 2;
}

/// Adapta um save de formato antigo ao atual, preservando tudo que ainda faz
/// sentido.
///
/// **v1 → v2:** as quests em andamento usavam variantes que não existem mais e
/// não têm equivalente automático; são descartadas e o app regenera as do nível
/// atual no próximo `refresh_quests`. XP, nível, conquistas, streaks, histórico
/// e aliases são preservados integralmente.
fn migrate(mut raw: serde_json::Value) -> serde_json::Value {
    let version = raw.get("version").and_then(|v| v.as_u64()).unwrap_or(0);

    if version < 2 {
        if let Some(state) = raw.get_mut("game_state").and_then(|g| g.as_object_mut()) {
            state.insert("active_quests".into(), serde_json::Value::Array(Vec::new()));
        }
        if let Some(obj) = raw.as_object_mut() {
            obj.insert("version".into(), SaveData::CURRENT_VERSION.into());
        }
    }

    raw
}

/// Retorna o caminho do arquivo de save, criando o diretório pai se necessário.
pub fn save_path() -> Result<PathBuf> {
    let base = if let Ok(xdg) = std::env::var("XDG_DATA_HOME") {
        PathBuf::from(xdg)
    } else if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".local").join("share")
    } else {
        PathBuf::from(".")
    };

    let dir = base.join("munux");
    fs::create_dir_all(&dir)
        .with_context(|| format!("não foi possível criar o diretório de dados: {}", dir.display()))?;
    Ok(dir.join("state.json"))
}

/// Carrega o save do disco, se existir e for válido.
///
/// Retorna `Ok(None)` quando ainda não há save (primeira execução) ou quando o
/// arquivo está corrompido — nesses casos o chamador deve seguir com um estado novo.
pub fn load() -> Result<Option<SaveData>> {
    let path = save_path()?;
    if !path.exists() {
        return Ok(None);
    }

    let raw = fs::read_to_string(&path)
        .with_context(|| format!("não foi possível ler o save: {}", path.display()))?;

    // Passa por `Value` primeiro para poder migrar formatos antigos antes de
    // exigir que o JSON case com a struct atual.
    let parsed = serde_json::from_str::<serde_json::Value>(&raw)
        .ok()
        .map(migrate)
        .and_then(|value| serde_json::from_value::<SaveData>(value).ok());

    match parsed {
        Some(data) => Ok(Some(data)),
        None => {
            // Save corrompido: faz backup e começa do zero (nunca sobrescreve
            // silenciosamente o arquivo original).
            let backup = path.with_extension("json.bak");
            let _ = fs::rename(&path, &backup);
            Ok(None)
        }
    }
}

/// Grava o estado atual no disco de forma atômica (escreve em arquivo temporário
/// e renomeia), para não corromper o save em caso de interrupção.
pub fn save(
    game_state: &GameState,
    command_history: &[String],
    aliases: &HashMap<String, String>,
) -> Result<()> {
    let path = save_path()?;

    let history: Vec<String> = if command_history.len() > MAX_HISTORY {
        command_history[command_history.len() - MAX_HISTORY..].to_vec()
    } else {
        command_history.to_vec()
    };

    let mut game_state = game_state.clone();
    game_state.last_session = Utc::now();

    let data = SaveData {
        version: SaveData::CURRENT_VERSION,
        game_state,
        command_history: history,
        aliases: aliases.clone(),
    };

    let json = serde_json::to_string_pretty(&data).context("falha ao serializar o save")?;

    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, json).with_context(|| format!("falha ao escrever {}", tmp.display()))?;
    fs::rename(&tmp, &path).with_context(|| format!("falha ao mover save para {}", path.display()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::{I18n, Language};

    #[test]
    fn roundtrip_serialization() {
        let i18n = I18n::new(Language::PtBr);
        let mut state = GameState::new(&i18n);
        state.add_xp(250);
        state.total_commands = 7;

        let history = vec!["ls".to_string(), "git status".to_string()];

        let mut aliases = HashMap::new();
        aliases.insert("gs".to_string(), "git status".to_string());

        let data = SaveData {
            version: SaveData::CURRENT_VERSION,
            game_state: state.clone(),
            command_history: history.clone(),
            aliases: aliases.clone(),
        };

        let json = serde_json::to_string_pretty(&data).unwrap();
        let parsed: SaveData = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.version, SaveData::CURRENT_VERSION);
        assert_eq!(parsed.game_state.xp, state.xp);
        assert_eq!(parsed.game_state.level, state.level);
        assert_eq!(parsed.game_state.total_commands, 7);
        assert_eq!(parsed.command_history, history);
        assert_eq!(parsed.aliases.get("gs").map(String::as_str), Some("git status"));
    }

    /// Um save v1 (com o formato antigo de `QuestObjective`) precisa continuar
    /// carregando: as quests se perdem, mas XP, nível, conquistas e streak não.
    #[test]
    fn v1_save_migrates_without_losing_progress() {
        let v1 = serde_json::json!({
            "version": 1,
            "game_state": {
                "xp": 42,
                "level": 7,
                "xp_to_next_level": 250,
                "integrity": 80,
                "safe_mode": false,
                "achievements": [{
                    "id": "first_command",
                    "name": "Primeiro Contato",
                    "description": "Execute seu primeiro comando",
                    "unlocked_at": "2026-01-01T00:00:00Z",
                    "xp_reward": 10
                }],
                "created_at": "2026-01-01T00:00:00Z",
                "last_session": "2026-01-02T00:00:00Z",
                "total_commands": 123,
                "daily_streak": 4,
                "command_streak": 9,
                "last_achievement": null,
                // Formato v1: variante que não existe mais.
                "active_quests": [{
                    "id": "intro_ls",
                    "title": "Explorador",
                    "description": "Liste os arquivos",
                    "objective": { "ExecuteCommand": { "command": "ls", "count": 1, "current": 0 } },
                    "xp_reward": 20,
                    "completed": false
                }],
                "successful_commands": 100,
                "failed_commands": 23
            },
            "command_history": ["ls", "cd /tmp"],
            "aliases": { "gs": "git status" }
        });

        let migrated = migrate(v1);
        let data: SaveData =
            serde_json::from_value(migrated).expect("save v1 deveria migrar para v2");

        assert_eq!(data.version, SaveData::CURRENT_VERSION);
        assert_eq!(data.game_state.level, 7);
        assert_eq!(data.game_state.xp, 42);
        assert_eq!(data.game_state.total_commands, 123);
        assert_eq!(data.game_state.daily_streak, 4);
        assert_eq!(data.game_state.command_streak, 9);
        assert_eq!(data.game_state.achievements.len(), 1);
        assert_eq!(data.command_history, vec!["ls", "cd /tmp"]);
        assert_eq!(data.aliases.get("gs").map(String::as_str), Some("git status"));
        // Só as quests em andamento se perdem — o app regenera as do nível.
        assert!(data.game_state.active_quests.is_empty());
    }

    /// Ida e volta pelo disco de verdade: `save` -> `load`, e um save v1 real
    /// sendo migrado ao ser carregado.
    ///
    /// Os dois cenários ficam no **mesmo** teste de propósito: ambos dependem de
    /// `XDG_DATA_HOME`, que é global ao processo, e os testes do Rust rodam em
    /// paralelo. Um único teste mexendo nessa variável não corre risco de corrida.
    #[test]
    fn save_and_load_roundtrip_through_disk() {
        let dir = std::env::temp_dir().join(format!("munux-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        std::env::set_var("XDG_DATA_HOME", &dir);

        let i18n = I18n::new(Language::PtBr);
        let mut state = GameState::new(&i18n);
        state.add_xp(150);
        state.total_commands = 9;

        let history = vec!["ls".to_string(), "git status".to_string()];
        let mut aliases = HashMap::new();
        aliases.insert("gs".to_string(), "git status".to_string());

        save(&state, &history, &aliases).expect("save falhou");

        let loaded = load().expect("load falhou").expect("save deveria existir");
        assert_eq!(loaded.version, SaveData::CURRENT_VERSION);
        assert_eq!(loaded.game_state.level, state.level);
        assert_eq!(loaded.game_state.total_commands, 9);
        assert_eq!(loaded.command_history, history);
        assert_eq!(loaded.aliases.get("gs").map(String::as_str), Some("git status"));

        // Agora um save v1 no disco: precisa carregar migrado, não virar backup.
        let path = save_path().unwrap();
        fs::write(
            &path,
            r#"{"version":1,"game_state":{"xp":42,"level":7,"xp_to_next_level":250,
               "integrity":80,"safe_mode":false,"achievements":[],
               "created_at":"2026-01-01T00:00:00Z","last_session":"2026-01-02T00:00:00Z",
               "total_commands":123,"daily_streak":4,"command_streak":9,
               "last_achievement":null,
               "active_quests":[{"id":"intro_ls","title":"t","description":"d",
                 "objective":{"ExecuteCommand":{"command":"ls","count":1,"current":0}},
                 "xp_reward":20,"completed":false}],
               "successful_commands":100,"failed_commands":23},
               "command_history":["ls"],"aliases":{}}"#,
        )
        .unwrap();

        let migrated = load().expect("load do v1 falhou").expect("v1 deveria carregar");
        assert_eq!(migrated.game_state.level, 7, "o nível se perdeu na migração");
        assert_eq!(migrated.game_state.total_commands, 123);
        assert!(migrated.game_state.active_quests.is_empty());
        assert!(
            !path.with_extension("json.bak").exists(),
            "um save v1 válido não pode ser tratado como corrompido"
        );

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn history_is_capped_on_save() {
        // Garante que a regra de corte mantém apenas os últimos MAX_HISTORY itens.
        let long_history: Vec<String> = (0..MAX_HISTORY + 50).map(|i| i.to_string()).collect();
        let trimmed: Vec<String> = if long_history.len() > MAX_HISTORY {
            long_history[long_history.len() - MAX_HISTORY..].to_vec()
        } else {
            long_history.clone()
        };

        assert_eq!(trimmed.len(), MAX_HISTORY);
        assert_eq!(trimmed.first().unwrap(), "50");
        assert_eq!(trimmed.last().unwrap(), &(MAX_HISTORY + 49).to_string());
    }
}
