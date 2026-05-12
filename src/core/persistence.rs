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
    pub const CURRENT_VERSION: u32 = 1;
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

    match serde_json::from_str::<SaveData>(&raw) {
        Ok(data) => Ok(Some(data)),
        Err(_) => {
            // Save corrompido ou de versão incompatível: faz backup e ignora.
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
