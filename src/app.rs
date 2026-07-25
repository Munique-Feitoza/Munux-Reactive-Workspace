// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use crate::core::parser::CommandType;
use crate::game::state::GameState;
use anyhow::Result;
use fluent::FluentArgs;
use std::collections::HashMap;
use std::path::PathBuf;

/// Enum que define os diferentes modos do Painel Direito (Reactive Panel)
#[derive(Debug, Clone, PartialEq)]
pub enum RightPanelMode {
    /// Tela inicial de boas-vindas
    Welcome,
    
    /// Modo padrão: mostra a árvore de arquivos do diretório atual
    FileTree { path: PathBuf },
    
    /// Mostra o preview de um arquivo com syntax highlighting
    FilePreview { 
        path: PathBuf, 
        content: String,
        language: String,
    },
    
    /// Mostra gráficos de recursos do sistema (CPU, RAM, Processos)
    ResourceMonitor { 
        cpu_usage: f32,
        memory_used: u64,
        memory_total: u64,
        process_count: usize,
    },
    
    /// Modo de alerta vermelho para comandos destrutivos
    DangerZone { 
        warning: String,
        command: String,
    },
    
    /// Mostra mensagens de gamificação (Level Up, conquistas)
    Gamification { 
        message: String,
        celebration: bool,
    },
    
    /// Mostra estatísticas do jogador
    Stats,
    
    /// Mostra quests/missões ativas
    Quests,
    
    /// Easter egg ativado
    EasterEgg {
        content: String,
    },
    
    /// Modo Help - mostra guias com scroll
    Help {
        content: String,
        title: String,
    },

    /// Modo de ajuda específica de comando
    CommandHelp {
        command: String,
        description: String,
        examples: Vec<String>,
        tip: String,
    },
    
    /// Output genérico de comando
    CommandOutput(String),
}

impl RightPanelMode {
    /// Constrói o painel de recursos a partir de um resumo do sistema.
    pub fn resource_from(s: &crate::core::monitor::SystemSummary) -> Self {
        RightPanelMode::ResourceMonitor {
            cpu_usage: s.cpu_usage,
            memory_used: s.memory_used,
            memory_total: s.memory_total,
            process_count: s.process_count,
        }
    }
}

/// Estado da aplicação - "Single Source of Truth"
pub struct App {
    /// Buffer de input do usuário (o que ele está digitando)
    pub input_buffer: String,
    
    /// Histórico de comandos executados
    pub command_history: Vec<String>,
    
    /// Índice atual no histórico (para navegação com setas)
    pub history_index: Option<usize>,
    
    /// Modo atual do painel direito
    pub right_panel_mode: RightPanelMode,
    
    /// Estado de gamificação (XP, nível, integridade)
    pub game_state: GameState,
    
    /// Diretório de trabalho atual
    pub current_dir: PathBuf,
    
    /// Output do último comando executado
    pub last_output: String,
    
    /// Flag para sair da aplicação
    pub should_quit: bool,
    
    /// Flag para indicar se o modo de perigo está ativo
    pub danger_mode_active: bool,
    
    /// Popup ativo (Ghost Mentor)
    pub active_popup: Option<PopupMessage>,

    /// Scroll vertical para o painel direito
    pub scroll: u16,

    /// Info do Git no diretório atual
    pub git_status: Option<crate::core::git::GitStatus>,

    /// Internacionalização
    pub i18n: crate::i18n::I18n,

    /// Sessão SSH ativa (se houver)
    pub ssh_session: Option<crate::core::ssh::SshSession>,

    /// Aliases definidos pelo usuário (`nome` -> `comando`)
    pub aliases: HashMap<String, String>,

    /// Passo atual do tutorial interativo (None = tutorial inativo)
    pub tutorial: Option<usize>,

    /// Teste de digitação em andamento (None = benchmark inativo)
    pub benchmark: Option<crate::game::benchmark::BenchmarkState>,

    /// Comando perigoso aguardando confirmação explícita (`sim`). `None` = nenhum.
    pub pending_command: Option<String>,

    /// Monitor de recursos persistente — uma única instância (refresh incremental).
    /// Evita recriar `System::new_all()` a cada tecla/tick e permite ao `sysinfo`
    /// calcular o delta de CPU corretamente entre refreshes.
    pub monitor: crate::core::monitor::SystemMonitor,

    /// Último resumo do sistema (consumido pelos painéis de monitor e de stats).
    pub system_summary: crate::core::monitor::SystemSummary,

    /// Índice selecionado no navegador de arquivos (painel de árvore + Enter/setas).
    pub file_selection: usize,

    /// A partir de qual índice do histórico o "scrollback" é exibido. `clear`/
    /// `Ctrl+L` apontam isto para o fim, limpando a tela **sem** perder o
    /// histórico navegável pelas setas.
    pub history_view_start: usize,

    /// Listagem do diretório atual, memoizada (limitada a [`MAX_TREE_ENTRIES`]).
    ///
    /// `render_file_tree` roda a cada frame (toda tecla + tick de 1 s) e a
    /// navegação por setas consulta a mesma lista. Sem cache, cada frame pagava
    /// um `read_dir` + um `stat` por entrada + um sort O(n log n). Agora a
    /// listagem é recalculada só por [`Self::refresh_dir_cache`]: ao trocar de
    /// diretório, após comandos que mexem em arquivos e no tick de 1 s enquanto
    /// a árvore está visível — mantendo a reatividade sem custo por tecla.
    dir_cache: Vec<crate::core::filesystem::FileEntry>,

    /// `true` quando o último `refresh_dir_cache` não conseguiu ler o diretório
    /// (permissão negada, diretório removido). Distingue "vazio" de "ilegível"
    /// sem que o render precise reabrir o diretório para descobrir.
    dir_unreadable: bool,

    /// Último preview lido do disco (`caminho -> conteúdo`). Enquanto a pessoa
    /// digita `cat arquivo.txt` o painel reage a cada tecla; sem esta memoização
    /// o arquivo inteiro era relido do disco a cada caractere.
    preview_cache: Option<(PathBuf, String)>,
}

/// Mensagem de popup para o Ghost Mentor
#[derive(Debug, Clone)]
pub struct PopupMessage {
    pub title: String,
    pub content: String,
    pub popup_type: PopupType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PopupType {
    Info,
    Warning,
    Success,
    Tip,
}

/// Palavras que cancelam um modo ativo (tutorial, benchmark). Fonte única.
const CANCEL_WORDS: &[&str] = &["sair", "exit", "stop", "parar", "cancel", "cancelar"];

/// Máximo de entradas exibidas/navegáveis no painel de árvore de arquivos.
pub const MAX_TREE_ENTRIES: usize = 20;

/// Máximo de linhas guardadas da saída de um comando. Acima disso o texto é
/// cortado e um aviso é anexado — ver [`App::sanitize_output`].
const MAX_OUTPUT_LINES: usize = 2_000;

/// Entrada já normalizada e pronta para despacho — ver [`App::prepare_command`].
struct PreparedCommand {
    /// O comando com aliases expandidos e sem espaços nas pontas.
    command: String,
    /// `true` quando este comando perigoso já passou pelo gate de confirmação
    /// (a pessoa respondeu `sim`), então não deve pedir confirmação de novo.
    danger_confirmed: bool,
}

/// O que fazer com a progressão depois de despachar um comando.
enum Progress {
    /// Comando real: alimenta conquistas e quests com o resultado da execução.
    Settle { success: bool },
    /// Nada a liquidar — comando da própria app (`stats`, `help`, `clear`),
    /// easter egg, ou fluxo interrompido (bloqueio, confirmação pendente).
    Skip,
}

/// Verifica se `word` é uma palavra de cancelamento (case-insensitive).
fn is_cancel_word(word: &str) -> bool {
    let w = word.trim().to_lowercase();
    CANCEL_WORDS.contains(&w.as_str())
}

/// Chave Fluent do aviso exibido na zona de perigo, a partir do comando inteiro.
///
/// `rm` tem três graus de gravidade (raiz / recursivo / simples) e continua
/// explícito; o resto virou tabela — antes era uma cadeia de 8 `else if` com
/// CC 9 embutida no meio de `command_to_panel_mode`.
///
/// A ordem das regras é significativa (da mais específica para a mais genérica)
/// e é a mesma de antes: `sudo` vem antes de `dd`, e assim por diante.
fn danger_warning_key(command: &str) -> &'static str {
    /// (gatilhos, chave Fluent). O primeiro gatilho encontrado vence.
    const RULES: &[(&[&str], &str)] = &[
        (&["dd"], "danger-dd"),
        (&["mkfs", "fdisk", "parted"], "danger-fs"),
        (&["chmod", "chown"], "danger-perm"),
        (&["reboot", "shutdown", "poweroff"], "danger-power"),
    ];

    if command.contains("rm") {
        if command.contains("-rf") || command.contains("-fr") {
            // Alvo parece ser uma raiz/diretório inteiro (`/*` ou terminando em `/`).
            return if command.contains("/*") || command.ends_with('/') {
                "danger-rm-root"
            } else {
                "danger-rm-rf"
            };
        }
        return "danger-rm";
    }

    // `sudo` casa por prefixo (não por substring): num `dd ... | sudo tee` o
    // aviso relevante é o do `dd`, que é quem de fato destrói dados.
    if command.starts_with("sudo") {
        return "danger-sudo";
    }

    RULES
        .iter()
        .find(|(triggers, _)| triggers.iter().any(|t| command.contains(t)))
        .map(|(_, key)| *key)
        .unwrap_or("danger-generic")
}

impl App {
    /// Cria uma nova instância do App, carregando o progresso salvo se existir
    pub fn new() -> Result<Self> {
        let current_dir = std::env::current_dir()?;

        let git_status = crate::core::git::GitManager::get_status(&current_dir);

        let i18n = crate::i18n::I18n::new(crate::i18n::Language::detect());

        // Tenta carregar o progresso salvo; em caso de save ausente ou inválido,
        // começa um estado novo.
        let (mut game_state, command_history, aliases, restored) =
            match crate::core::persistence::load() {
                Ok(Some(data)) => (data.game_state, data.command_history, data.aliases, true),
                _ => (GameState::new(&i18n), Vec::new(), HashMap::new(), false),
            };

        if restored {
            game_state.update_daily_streak();
        }

        let mut app = Self {
            input_buffer: String::new(),
            command_history,
            history_index: None,
            right_panel_mode: RightPanelMode::Welcome,
            game_state,
            current_dir,
            last_output: String::new(),
            should_quit: false,
            danger_mode_active: false,
            active_popup: None,
            scroll: 0,
            git_status,
            i18n,
            ssh_session: None,
            aliases,
            tutorial: None,
            benchmark: None,
            pending_command: None,
            monitor: crate::core::monitor::SystemMonitor::new(),
            system_summary: crate::core::monitor::SystemSummary::default(),
            file_selection: 0,
            history_view_start: 0,
            dir_cache: Vec::new(),
            dir_unreadable: false,
            preview_cache: None,
        };

        app.refresh_dir_cache();
        Ok(app)
    }

    /// Limpa a tela (saída + scrollback visível) e volta ao painel inicial.
    /// Conceito único de "limpar" compartilhado por `clear`/`cls` e `Ctrl+L`.
    /// O histórico navegável (setas) é preservado.
    pub fn clear_screen(&mut self) {
        self.last_output.clear();
        self.history_view_start = self.command_history.len();
        self.right_panel_mode = RightPanelMode::Welcome;
        self.clear_input();
    }

    /// Entradas do diretório atual (mesma lista do painel de árvore, limitada a
    /// [`MAX_TREE_ENTRIES`]). Fonte única para render e navegação — devolve o
    /// cache, sem tocar no disco.
    pub fn dir_entries(&self) -> &[crate::core::filesystem::FileEntry] {
        &self.dir_cache
    }

    /// `true` quando o diretório atual não pôde ser lido (permissão, remoção).
    pub fn dir_unreadable(&self) -> bool {
        self.dir_unreadable
    }

    /// Relê o diretório atual e reconstrói o cache da árvore de arquivos.
    /// Único ponto que toca o disco para listar; ver [`Self::dir_cache`].
    pub fn refresh_dir_cache(&mut self) {
        match crate::core::filesystem::FileSystemManager::list_directory(&self.current_dir) {
            Ok(mut entries) => {
                entries.truncate(MAX_TREE_ENTRIES);
                self.dir_cache = entries;
                self.dir_unreadable = false;
            }
            Err(_) => {
                self.dir_cache.clear();
                self.dir_unreadable = true;
            }
        }
        // A seleção pode ter ficado além do fim depois de remover arquivos.
        if self.file_selection >= self.dir_cache.len() {
            self.file_selection = self.dir_cache.len().saturating_sub(1);
        }

        // O conteúdo memoizado pode ter ficado obsoleto junto com a listagem.
        self.preview_cache = None;
    }

    /// `true` quando o navegador de arquivos está ativo: árvore visível e sem
    /// nada digitado (aí as setas navegam arquivos em vez do histórico).
    pub fn is_browsing_files(&self) -> bool {
        self.input_buffer.is_empty()
            && matches!(self.right_panel_mode, RightPanelMode::FileTree { .. })
    }

    /// Move a seleção do navegador (`delta` negativo = sobe).
    pub fn move_file_selection(&mut self, delta: i32) {
        let len = self.dir_cache.len();
        if len == 0 {
            self.file_selection = 0;
            return;
        }
        self.file_selection = if delta < 0 {
            self.file_selection.saturating_sub(1)
        } else {
            (self.file_selection + 1).min(len - 1)
        };
    }

    /// Abre a entrada selecionada: entra no diretório ou mostra o preview do
    /// arquivo. Usa o campo `FileEntry.path`.
    pub fn open_selected_entry(&mut self) {
        let Some(entry) = self.dir_cache.get(self.file_selection) else { return };
        let path = entry.path.clone();
        let is_dir = entry.is_dir;
        let name = entry.name.clone();

        if is_dir {
            if let Some(p) = path.to_str() {
                let _ = self.change_directory(p);
            }
            self.file_selection = 0;
        } else {
            let language = crate::core::parser::CommandParser::detect_language(&name);
            let content = self.read_preview(&path).unwrap_or_default();
            self.right_panel_mode = RightPanelMode::FilePreview { path, content, language };
            self.scroll = 0;
        }
    }

    /// Atualiza o monitor de sistema persistente e guarda o resumo. Se o painel
    /// de recursos estiver ativo, reflete os novos valores nele.
    pub fn refresh_monitor(&mut self) {
        self.system_summary = self.monitor.get_system_summary();
        if matches!(self.right_panel_mode, RightPanelMode::ResourceMonitor { .. }) {
            self.right_panel_mode = RightPanelMode::resource_from(&self.system_summary);
        }
    }

    /// Atalho para traduzir uma chave com um único argumento string.
    /// (Casos multi-argumento montam `FluentArgs` inline.) Delega à fonte única
    /// em [`crate::i18n::I18n::t1`], que a camada de UI também usa.
    fn t1(&self, key: &str, name: &'static str, value: impl Into<String>) -> String {
        self.i18n.t1(key, name, value)
    }

    /// Grava o progresso atual em disco (chamado após cada comando e ao sair).
    /// Falhas de I/O são silenciosas para não interromper a sessão do usuário.
    pub fn save_progress(&self) {
        let _ = crate::core::persistence::save(&self.game_state, &self.command_history, &self.aliases);
    }

    /// Expande recursivamente o alias da primeira palavra de `command`.
    ///
    /// Ex.: com `gs` -> `git status`, a entrada `gs -s` vira `git status -s`.
    /// A profundidade é limitada para evitar loops (`a=b`, `b=a`).
    fn expand_alias(&self, command: &str) -> String {
        let mut current = command.trim().to_string();
        for _ in 0..10 {
            let (head, rest) = match current.split_once(char::is_whitespace) {
                Some((h, r)) => (h.to_string(), r.to_string()),
                None => (current.clone(), String::new()),
            };
            match self.aliases.get(&head) {
                Some(expansion) => {
                    current = if rest.is_empty() {
                        expansion.clone()
                    } else {
                        format!("{} {}", expansion, rest)
                    };
                }
                None => break,
            }
        }
        current
    }

    /// Trata os comandos `alias` / `unalias`. Retorna `true` se o comando foi
    /// um comando de alias (e já foi processado).
    fn handle_alias_command(&mut self, command: &str) -> bool {
        let trimmed = command.trim();

        // `alias` ou `aliases`: lista todos.
        if trimmed == "alias" || trimmed == "aliases" {
            if self.aliases.is_empty() {
                self.last_output = self.i18n.tc("sys-alias-none");
            } else {
                let mut lines: Vec<String> =
                    self.aliases.iter().map(|(k, v)| format!("  {} = {}", k, v)).collect();
                lines.sort();
                self.last_output =
                    format!("{}\n{}", self.i18n.tc("sys-alias-list-title"), lines.join("\n"));
            }
            self.clear_input();
            return true;
        }

        // `unalias nome`: remove.
        if let Some(name) = trimmed.strip_prefix("unalias ") {
            let name = name.trim();
            self.last_output = if self.aliases.remove(name).is_some() {
                self.t1("sys-alias-removed", "name", name)
            } else {
                self.t1("sys-alias-missing", "name", name)
            };
            self.clear_input();
            return true;
        }

        // `alias nome=comando` ou `alias nome='comando com args'`.
        if let Some(rest) = trimmed.strip_prefix("alias ") {
            if let Some((name, value)) = rest.split_once('=') {
                let name = name.trim();
                let value = value.trim().trim_matches(|c| c == '\'' || c == '"').trim();
                if name.is_empty() || value.is_empty() {
                    self.last_output = self.i18n.tc("sys-alias-usage");
                } else if name.contains(char::is_whitespace) {
                    self.last_output = self.i18n.tc("sys-alias-no-spaces");
                } else {
                    self.aliases.insert(name.to_string(), value.to_string());
                    let mut args = FluentArgs::new();
                    args.set("name", name);
                    args.set("value", value);
                    self.last_output = self.i18n.t("sys-alias-created", Some(&args));
                }
            } else {
                self.last_output = self.i18n.tc("sys-alias-usage");
            }
            self.clear_input();
            return true;
        }

        false
    }

    /// Trata os comandos `tutorial` / `tutorial sair`. Retorna `true` se o
    /// comando foi de tutorial (e já foi processado).
    fn handle_tutorial_command(&mut self, command: &str) -> bool {
        let trimmed = command.trim();
        let first = trimmed.split_whitespace().next().unwrap_or("");
        if first != "tutorial" {
            return false;
        }

        let arg = trimmed.split_whitespace().nth(1).unwrap_or("");
        if is_cancel_word(arg) {
            if self.tutorial.is_some() {
                self.tutorial = None;
                self.last_output = self.i18n.tc("sys-tutorial-ended");
            } else {
                self.last_output = self.i18n.tc("sys-tutorial-none");
            }
        } else {
            self.tutorial = Some(0);
            self.last_output = self.i18n.tc("sys-tutorial-started");
            self.show_popup(
                self.i18n.tc("sys-tutorial-mode-title"),
                crate::game::tutorial::step_text(0, &self.i18n),
                PopupType::Tip,
            );
        }

        self.clear_input();
        true
    }

    /// Verifica se `command` completa o passo atual do tutorial e, em caso
    /// afirmativo, avança (ou conclui o tutorial com bônus de XP).
    fn advance_tutorial(&mut self, command: &str) {
        use crate::game::tutorial;

        let Some(step_idx) = self.tutorial else { return };
        let Some(step) = tutorial::STEPS.get(step_idx) else {
            self.tutorial = None;
            return;
        };
        if !step.matches(command) {
            return;
        }

        let next = step_idx + 1;
        if next < tutorial::STEPS.len() {
            self.tutorial = Some(next);
            self.show_popup(
                self.i18n.tc("sys-tutorial-step-done-title"),
                tutorial::step_text(next, &self.i18n),
                PopupType::Success,
            );
        } else {
            self.tutorial = None;
            self.game_state.add_xp(tutorial::COMPLETION_XP);
            let mut args = FluentArgs::new();
            args.set("xp", tutorial::COMPLETION_XP);
            self.show_popup(
                self.i18n.tc("sys-tutorial-complete-title"),
                self.i18n.t("sys-tutorial-complete-body", Some(&args)),
                PopupType::Success,
            );
        }
    }

    /// Trata o comando `benchmark` (iniciar) e `benchmark sair`. Retorna `true`
    /// se o comando foi de benchmark e já foi processado. A *avaliação* da frase
    /// digitada acontece em `finish_benchmark`, chamada no início de
    /// `execute_command` quando um benchmark está ativo.
    fn handle_benchmark_command(&mut self, command: &str) -> bool {
        let trimmed = command.trim();
        if trimmed.split_whitespace().next() != Some("benchmark") {
            return false;
        }

        if is_cancel_word(trimmed.split_whitespace().nth(1).unwrap_or("")) {
            self.last_output = self.i18n.tc("sys-bench-none");
        } else {
            let state = crate::game::benchmark::BenchmarkState::start();
            self.last_output = self.t1("sys-bench-start", "phrase", state.prompt.clone());
            self.show_popup(
                self.i18n.tc("sys-bench-popup-title"),
                self.t1("sys-bench-popup-body", "phrase", state.prompt.clone()),
                PopupType::Info,
            );
            self.benchmark = Some(state);
        }

        self.clear_input();
        true
    }

    /// Avalia o texto digitado contra a frase do benchmark ativo, mostra o
    /// resultado e concede XP.
    fn finish_benchmark(&mut self, typed: &str) {
        let Some(state) = self.benchmark.take() else { return };
        let seconds = state.started_at.elapsed().as_secs_f64();
        let result = crate::game::benchmark::score(&state.prompt, typed.trim(), seconds);

        self.game_state.add_xp(result.xp);
        let mut args = FluentArgs::new();
        args.set("seconds", format!("{:.1}", result.seconds)); // precisão fixa em Rust
        args.set("wpm", result.wpm);
        args.set("accuracy", result.accuracy);
        args.set("xp", result.xp);
        let summary = self.i18n.t("sys-bench-result", Some(&args));
        self.last_output = summary.clone();
        self.show_popup(self.i18n.tc("sys-bench-result-title"), summary, PopupType::Success);
    }

    /// Quantidade de linhas roláveis do painel direito atual. Painéis de altura
    /// fixa (Welcome, Stats, árvore…) devolvem 0 e não rolam.
    fn scrollable_lines(&self) -> usize {
        match &self.right_panel_mode {
            RightPanelMode::Help { content, .. }
            | RightPanelMode::FilePreview { content, .. }
            | RightPanelMode::CommandOutput(content) => content.lines().count(),
            _ => 0,
        }
    }

    /// Rola o painel direito, limitado ao conteúdo. Fonte única de scroll —
    /// PageUp/PageDown e a roda do mouse passam por aqui.
    ///
    /// Antes cada um chamava `saturating_add`/`saturating_sub` direto no campo,
    /// sem teto: bastava segurar PageDown para o painel virar uma área vazia sem
    /// pista de como voltar.
    pub fn scroll_by(&mut self, delta: i32) {
        // Deixa a última linha alcançável, sem permitir passar dela.
        let max = self.scrollable_lines().saturating_sub(1).min(u16::MAX as usize) as u16;
        let target = i64::from(self.scroll) + i64::from(delta);
        self.scroll = target.clamp(0, i64::from(max)) as u16;
    }

    /// Atualiza o buffer de input (chamado a cada tecla)
    pub fn update_input(&mut self, character: char) {
        self.input_buffer.push(character);
        // Trigger de análise reativa do parser
        self.analyze_input();
    }
    
    /// Remove o último caractere do buffer
    pub fn delete_char(&mut self) {
        self.input_buffer.pop();
        self.analyze_input();
    }
    
    /// Limpa o buffer de input
    pub fn clear_input(&mut self) {
        self.input_buffer.clear();
        self.analyze_input();
    }
    
    /// Completa o input atual (Tab). Quando há mais de um candidato, estende o
    /// buffer até o maior prefixo comum e lista as opções no output.
    pub fn autocomplete(&mut self) {
        // Em sessão SSH a completação de caminhos seria contra o disco local
        // (errado), então só ajuda com nomes de comando — o módulo lida bem com
        // isso quando o diretório informado não tem entradas correspondentes.
        let result = crate::core::completion::complete(&self.input_buffer, &self.current_dir);
        self.input_buffer = result.new_input;

        if !result.suggestions.is_empty() {
            self.last_output = format!("⇥ {}", result.suggestions.join("   "));
        }

        self.history_index = None;
        self.analyze_input();
    }

    /// Navega para o comando anterior no histórico
    pub fn history_previous(&mut self) {
        if self.command_history.is_empty() {
            return;
        }
        
        let new_index = match self.history_index {
            None => Some(self.command_history.len() - 1),
            Some(0) => Some(0),
            Some(idx) => Some(idx - 1),
        };
        
        self.history_index = new_index;
        if let Some(idx) = new_index {
            self.input_buffer = self.command_history[idx].clone();
        }
    }
    
    /// Navega para o próximo comando no histórico
    pub fn history_next(&mut self) {
        if self.command_history.is_empty() {
            return;
        }
        
        match self.history_index {
            None => {},
            Some(idx) if idx >= self.command_history.len() - 1 => {
                self.history_index = None;
                self.input_buffer.clear();
            },
            Some(idx) => {
                self.history_index = Some(idx + 1);
                self.input_buffer = self.command_history[idx + 1].clone();
            },
        }
    }
    
    /// Executa o comando atual.
    ///
    /// Orquestrador em três tempos — **preparar** a entrada, **despachar** para
    /// o handler certo e **liquidar** a progressão. Cada etapa vive numa função
    /// própria; antes tudo isso era um único corpo de 382 linhas com CC 57.
    pub fn execute_command(&mut self) -> Result<()> {
        // 1. Preparação: a entrada pode nem ser um comando (resposta de
        //    confirmação, frase do benchmark) ou já ser consumida por um modo
        //    ativo (alias, tutorial, sessão SSH).
        let Some(prepared) = self.prepare_command() else {
            return Ok(());
        };

        // 2. Despacho.
        let progress = self.dispatch_command(&prepared);

        // 3. Liquidação: conquistas e quests só correm para comandos de verdade.
        if let Progress::Settle { success } = progress {
            self.settle_progress(&prepared.command, success);
        }

        self.clear_input();
        Ok(())
    }

    /// Consome a entrada atual e devolve o comando pronto para despacho, ou
    /// `None` quando a entrada já foi totalmente tratada aqui.
    ///
    /// Concentra tudo que **precede** um comando: gate de confirmação de perigo,
    /// benchmark em andamento, histórico, expansão de alias, comandos de modo
    /// (alias/benchmark/tutorial) e a sessão SSH ativa.
    fn prepare_command(&mut self) -> Option<PreparedCommand> {
        if self.input_buffer.trim().is_empty() {
            return None;
        }

        // Gate de confirmação: se há um comando perigoso pendente, esta entrada é
        // a resposta. Só `sim`/`s`/`yes`/`y` re-injeta o comando para execução;
        // qualquer outra coisa cancela.
        let mut danger_confirmed = false;
        if let Some(pending) = self.pending_command.take() {
            let answer = self.input_buffer.trim().to_lowercase();
            if matches!(answer.as_str(), "sim" | "s" | "yes" | "y") {
                self.input_buffer = pending;
                danger_confirmed = true;
            } else {
                self.clear_input();
                self.last_output = self.i18n.tc("sys-danger-cancelled");
                self.right_panel_mode = RightPanelMode::Welcome;
                return None;
            }
        }

        // Modo benchmark ativo: a entrada é a frase digitada, não um comando.
        if self.benchmark.is_some() {
            self.resolve_benchmark_entry();
            return None;
        }

        // Adiciona ao histórico (o comando confirmado já foi registrado na 1ª tentativa).
        if !danger_confirmed {
            self.command_history.push(self.input_buffer.clone());
        }
        self.history_index = None;

        let command = self.input_buffer.trim().to_string();

        // Numa sessão SSH o que vale é o shell remoto: nem alias, nem tutorial,
        // nem benchmark locais se aplicam.
        if self.ssh_session.is_some() {
            self.handle_ssh_session(&command);
            return None;
        }

        if self.handle_alias_command(&command) {
            return None;
        }
        let command = self.expand_alias(&command);

        if self.handle_benchmark_command(&command) {
            return None;
        }

        if self.handle_tutorial_command(&command) {
            return None;
        }
        self.advance_tutorial(&command);

        if self.handle_ssh_connect(&command) {
            return None;
        }

        Some(PreparedCommand { danger_confirmed, command })
    }

    /// Avalia a entrada durante um benchmark ativo: `benchmark sair` cancela,
    /// qualquer outra coisa é a frase digitada.
    fn resolve_benchmark_entry(&mut self) {
        let typed = self.input_buffer.clone();
        let mut words = typed.split_whitespace();
        let is_cancel = words.next() == Some("benchmark")
            && words.next().map(is_cancel_word).unwrap_or(false);

        if is_cancel {
            self.benchmark = None;
            self.last_output = self.i18n.tc("sys-bench-cancelled");
        } else {
            self.finish_benchmark(&typed);
        }
        self.clear_input();
    }

    /// Trata `ssh user@host` abrindo uma sessão remota. Retorna `true` se o
    /// comando era uma conexão SSH (bem-sucedida ou não) e já foi processado.
    fn handle_ssh_connect(&mut self, command: &str) -> bool {
        let Some(target) = command
            .strip_prefix("ssh ")
            .and_then(|rest| rest.split_whitespace().next())
        else {
            return false;
        };
        // Sem `user@host` não é uma conexão: deixa cair no shell (`ssh --help`).
        let Some((user, host)) = target.split_once('@') else {
            return false;
        };
        let (user, host) = (user.to_string(), host.to_string());

        let mut args = FluentArgs::new();
        args.set("user", user.clone());
        args.set("host", host.clone());
        self.last_output = self.i18n.t("sys-ssh-connecting", Some(&args));

        // A conexão bloqueia por um instante — o TUI é síncrono.
        match crate::core::ssh::SshSession::connect(&user, &host) {
            Ok(session) => {
                let cwd = session.remote_cwd.clone();
                self.ssh_session = Some(session);
                self.right_panel_mode = RightPanelMode::Welcome;

                let mut args = FluentArgs::new();
                args.set("host", host.clone());
                args.set("dir", cwd.clone());
                self.last_output = self.i18n.t("sys-ssh-connected", Some(&args));

                let mut args = FluentArgs::new();
                args.set("user", user);
                args.set("host", host);
                args.set("dir", cwd);
                let title = self.i18n.tc("sys-ssh-conn-title");
                let body = self.i18n.t("sys-ssh-conn-body", Some(&args));
                self.show_popup(title, body, PopupType::Success);
            }
            Err(e) => {
                let msg = self.describe_ssh_error(&e);
                self.last_output = self.t1("sys-ssh-fail", "msg", msg.clone());

                let mut args = FluentArgs::new();
                args.set("target", target);
                args.set("msg", msg);
                let title = self.i18n.tc("sys-ssh-fail-title");
                let body = self.i18n.t("sys-ssh-fail-body", Some(&args));
                self.show_popup(title, body, PopupType::Warning);
            }
        }

        self.input_buffer.clear();
        true
    }

    /// Descreve uma falha de SSH no idioma do usuário.
    ///
    /// As falhas que a interface exibe são erros **tipados**
    /// ([`crate::core::ssh::SshError`]) justamente para poderem ser traduzidas
    /// aqui: o `core` não escolhe idioma. Erros que escapem da `libssh2` sem
    /// tipagem própria são repassados como estão (texto da biblioteca, em
    /// inglês) — melhor isso do que engolir a causa real.
    fn describe_ssh_error(&self, error: &anyhow::Error) -> String {
        use crate::core::ssh::SshError;

        let Some(ssh_error) = error.downcast_ref::<SshError>() else {
            return error.to_string();
        };

        let mut args = FluentArgs::new();
        match ssh_error {
            SshError::Connect { host, port } => {
                args.set("host", host.clone());
                args.set("port", *port);
            }
            SshError::HostKeyMismatch { host, known_hosts } => {
                args.set("host", host.clone());
                args.set("file", known_hosts.clone());
            }
            SshError::HostKeyUnverifiable { host } => args.set("host", host.clone()),
            SshError::RemoteDirNotFound { path } => args.set("path", path.clone()),
            SshError::Handshake | SshError::AuthFailed => {}
        }

        self.i18n.t(ssh_error.message_key(), Some(&args))
    }

    /// Despacha o comando para o handler correspondente e diz se a progressão
    /// deve ser liquidada.
    fn dispatch_command(&mut self, prepared: &PreparedCommand) -> Progress {
        use crate::core::parser::CommandParser;
        use crate::game::achievements::AchievementChecker;
        use crate::game::easter_eggs::EasterEggs;

        let command = &prepared.command;
        let cmd_type = CommandParser::classify_command(command);

        // Modo seguro: comando bloqueado antes de qualquer efeito.
        if !CommandParser::is_safe_command(command, self.game_state.safe_mode) {
            self.last_output = self.i18n.tc("sys-access-denied");
            self.show_popup(
                self.i18n.tc("sys-access-denied-title"),
                self.i18n.tc("sys-access-denied-body"),
                PopupType::Warning,
            );
            self.game_state.record_failure();
            return Progress::Skip;
        }

        if matches!(cmd_type, CommandType::Dangerous) {
            // Comando perigoso liberado (modo seguro desligado): exige confirmação
            // explícita em vez de executar direto no Enter.
            if !prepared.danger_confirmed {
                self.pending_command = Some(command.clone());
                self.clear_input();
                self.last_output = self.i18n.tc("sys-danger-confirm");
                self.right_panel_mode = self.command_to_panel_mode(command, &cmd_type);
                return Progress::Skip;
            }
            self.game_state.damage_integrity(10);
        }

        // Easter eggs vêm antes dos comandos internos.
        if let Some(output) = EasterEggs::check(command, &self.i18n) {
            self.last_output = output.clone();
            self.right_panel_mode = RightPanelMode::EasterEgg { content: output };

            if let Some(achievement) =
                AchievementChecker::check_easter_egg(&mut self.game_state, command, &self.i18n)
            {
                self.award_achievement(achievement, false);
            }
            self.clear_input();
            return Progress::Skip;
        }

        // Comandos especiais do Munux (stats/quests/achievements/tip/help).
        if self.handle_special_command(command) {
            return Progress::Skip;
        }

        self.run_command(command, &cmd_type)
    }

    /// Executa o comando propriamente dito: os embutidos (`cd`, `ls`, `exit`,
    /// `clear`, `xp` de debug) ou, no fim da linha, o shell do sistema.
    fn run_command(&mut self, command: &str, cmd_type: &CommandType) -> Progress {
        if let Some(path) = command.strip_prefix("cd ") {
            return self.run_cd(path.trim());
        }

        // Comando secreto para testar progressão de nível — só em builds de
        // debug. Em release o ramo é eliminado e `xp` cai no shell (sem cheat).
        if cfg!(debug_assertions) {
            if let Some(arg) = command.strip_prefix("xp ") {
                self.run_xp_cheat(arg.trim());
                return Progress::Settle { success: true };
            }
        }

        if command == "exit" || command == "quit" {
            self.should_quit = true;
            return Progress::Settle { success: true };
        }

        if command == "clear" || command == "cls" {
            // limpa buffer e tela; histórico navegável preservado
            self.clear_screen();
            return Progress::Skip;
        }

        if crate::core::commands::is_listing(command.split_whitespace().next().unwrap_or("")) {
            return self.run_listing(command, cmd_type);
        }

        self.run_shell(command, cmd_type)
    }

    /// `cd <caminho>`. Conta para stats/quests como qualquer comando real.
    fn run_cd(&mut self, path: &str) -> Progress {
        self.game_state.increment_commands();

        match self.change_directory(path) {
            Ok(_) => {
                self.last_output =
                    self.t1("sys-cd-ok", "dir", self.current_dir.display().to_string());
                self.game_state.record_success();
                self.right_panel_mode = RightPanelMode::FileTree {
                    path: self.current_dir.clone(),
                };
                Progress::Settle { success: true }
            }
            Err(e) => {
                self.last_output = self.t1("sys-error", "msg", e.to_string());
                self.game_state.record_failure();
                Progress::Settle { success: false }
            }
        }
    }

    /// `ls`/`ll`/`la`: sem output no painel esquerdo, só atualiza a árvore.
    fn run_listing(&mut self, command: &str, cmd_type: &CommandType) -> Progress {
        self.refresh_dir_cache();
        self.last_output = self.i18n.tc("sys-ls-listed");
        self.right_panel_mode = RightPanelMode::FileTree {
            path: self.current_dir.clone(),
        };

        let xp = crate::game::logic::calculate_xp_reward(command, cmd_type, true);
        self.game_state.add_xp(xp);
        self.game_state.increment_commands();
        self.game_state.record_success();

        Progress::Settle { success: true }
    }

    /// Concede XP diretamente (cheat de desenvolvimento, só em build de debug).
    fn run_xp_cheat(&mut self, arg: &str) {
        let Ok(amount) = arg.parse::<u32>() else {
            self.last_output = self.i18n.tc("sys-xp-usage");
            return;
        };

        if self.game_state.add_xp(amount) {
            self.last_output = format!(
                "{}: {} | {}: {} | XP: {} | {}",
                self.i18n.tc("ui-level"),
                self.game_state.level,
                self.i18n.tc("ui-rank"),
                self.game_state.get_rank(&self.i18n),
                self.game_state.xp,
                self.i18n.level_message(self.game_state.level)
            );
            self.game_state.refresh_quests(&self.i18n);
        } else {
            let mut args = FluentArgs::new();
            args.set("amount", amount);
            args.set("current", self.game_state.xp);
            args.set("total", self.game_state.xp_to_next_level);
            args.set("next", self.game_state.level + 1);
            self.last_output = self.i18n.t("sys-xp-gain", Some(&args));
        }
    }

    /// Executa o comando no shell do sistema e reflete o resultado na UI.
    fn run_shell(&mut self, command: &str, cmd_type: &CommandType) -> Progress {
        use crate::core::shell::ShellExecutor;

        // Injeta flags de cor para comandos comuns se não estiverem presentes.
        let color_command = Self::prepare_color_command(command);

        let output = match ShellExecutor::execute(&color_command, &self.current_dir) {
            Ok(output) => output,
            Err(e) => {
                self.last_output = self.t1("sys-cmd-exec-error", "msg", e.to_string());
                self.game_state.increment_commands();
                self.game_state.record_failure();
                return Progress::Settle { success: false };
            }
        };

        // `output.success` é a fonte da verdade do resultado. Antes o sucesso era
        // deduzido de `last_output.starts_with("✗")` — ou seja, a lógica de
        // conquistas dependia do glifo de uma string traduzível.
        let success = output.success;

        let body = if success {
            output.combined_output()
        } else {
            self.add_educational_hints(command, &output.combined_output())
        };

        self.last_output = self
            .i18n
            .tc(if success { "sys-cmd-ok" } else { "sys-cmd-error" });

        self.right_panel_mode = RightPanelMode::CommandOutput(self.decorate_output(command, &body));
        self.scroll = 0;

        if success {
            let old_level = self.game_state.level;
            let xp = crate::game::logic::calculate_xp_reward(command, cmd_type, true);
            let leveled_up = self.game_state.add_xp(xp);
            self.game_state.increment_commands();
            self.game_state.record_success();
            self.game_state.restore_integrity(5);

            if leveled_up {
                self.announce_level_up(old_level);
            }
        } else {
            self.game_state.increment_commands();
            self.game_state.record_failure();
            self.game_state.damage_integrity(3);
        }

        // O comando pode ter mexido no diretório (inclusive via redirecionamento),
        // então a listagem em cache é refeita sempre — um `read_dir` por comando
        // digitado, não por frame.
        self.refresh_dir_cache();

        // Já a troca de painel segue o catálogo (`core::commands`), não uma lista
        // paralela de prefixos.
        if crate::core::commands::mutates_files(command.split_whitespace().next().unwrap_or("")) {
            self.right_panel_mode = RightPanelMode::FileTree {
                path: self.current_dir.clone(),
            };
        }

        Progress::Settle { success }
    }

    /// Prefixa a saída do comando com o prompt estilizado (ANSI) do painel direito.
    fn decorate_output(&self, command: &str, body: &str) -> String {
        use crate::ui::theme::Theme;

        let symbol = Theme::get_prompt_symbol(self.game_state.level);
        let rank = self.game_state.get_rank(&self.i18n);
        // Prompt verde neon com o comando em ciano brilhante.
        let prompt = format!(
            "\x1b[1;32m{} [{}@munux]$ \x1b[0m\x1b[1;36m{}\x1b[0m",
            symbol, rank, command
        );
        format!("{}\n{}", prompt, self.sanitize_output(body))
    }

    /// Popup + painel de celebração ao subir de nível.
    fn announce_level_up(&mut self, old_level: u32) {
        self.show_level_up_popup(old_level, self.game_state.level);
        self.game_state.refresh_quests(&self.i18n);

        let mut args = FluentArgs::new();
        args.set("level", fluent::FluentValue::from(self.game_state.level));
        self.right_panel_mode = RightPanelMode::Gamification {
            message: self.i18n.t("sys-level-up-msg", Some(&args)),
            celebration: true,
        };
    }

    /// Liquida a progressão após um comando real: conquistas (comando, streak,
    /// nível) e avanço das quests ativas.
    fn settle_progress(&mut self, command: &str, success: bool) {
        use crate::game::achievements::AchievementChecker;

        if let Some(achievement) =
            AchievementChecker::check_command(&mut self.game_state, command, success, &self.i18n)
        {
            self.award_achievement(achievement, true);
        }
        if let Some(achievement) =
            AchievementChecker::check_streak(&mut self.game_state, &self.i18n)
        {
            self.award_achievement(achievement, false);
        }
        if let Some(achievement) = AchievementChecker::check_level(&mut self.game_state, &self.i18n)
        {
            self.award_achievement(achievement, false);
        }

        // Avança as quests e coleta as que fecharam (o empréstimo mutável de
        // `active_quests` termina antes de mexer no resto do estado).
        let current_level = self.game_state.level;
        let completed: Vec<(String, u32)> = self
            .game_state
            .active_quests
            .iter_mut()
            .filter(|q| !q.completed)
            .filter_map(|quest| {
                quest
                    .update_progress(command, current_level)
                    .then(|| (quest.title.clone(), quest.xp_reward))
            })
            .collect();

        for (title, xp) in completed {
            let mut args = FluentArgs::new();
            args.set("title", title);
            args.set("xp", xp);
            let quest_msg = self.i18n.t("sys-quest-complete", Some(&args));
            self.last_output = format!("{}{}", self.last_output, quest_msg);
            self.game_state.add_xp(xp);
        }
    }
    
    /// Muda o diretório atual
    fn change_directory(&mut self, path: &str) -> Result<()> {
        let new_path = if path.starts_with('/') {
            PathBuf::from(path)
        } else {
            self.current_dir.join(path)
        };
        
        if new_path.exists() && new_path.is_dir() {
            // Tenta canonicalizar o caminho para resolver .. e .
            if let Ok(canon_path) = std::fs::canonicalize(&new_path) {
                self.current_dir = canon_path.clone();
                self.right_panel_mode = RightPanelMode::FileTree { path: canon_path };
            } else {
                // Fallback se falhar
                self.current_dir = new_path.clone();
                self.right_panel_mode = RightPanelMode::FileTree { path: new_path };
            }
            // O diretório mudou: a listagem em cache não vale mais.
            self.file_selection = 0;
            self.refresh_dir_cache();
            Ok(())
        } else {
            anyhow::bail!("{}", self.t1("sys-cd-notfound", "path", path))
        }
    }
    
    /// Trata os comandos especiais do Munux (stats/quests/achievements/tip/help).
    /// Retorna `true` se o comando foi um deles (e já foi processado) — extraído
    /// de `execute_command` para reduzir o God Object.
    fn handle_special_command(&mut self, command: &str) -> bool {
        if command == "stats" {
            self.refresh_monitor(); // snapshot fresco de CPU/RAM ao abrir o painel
            self.right_panel_mode = RightPanelMode::Stats;
            self.last_output = self.i18n.tc("sys-showing-stats");
            self.input_buffer.clear();
            true
        } else if command == "quests" || command == "missions" {
            self.right_panel_mode = RightPanelMode::Quests;
            self.last_output = self.i18n.tc("sys-showing-quests");
            self.input_buffer.clear();
            true
        } else if command == "achievements" {
            self.last_output = format!(
                "{}: {}/100\n\n{}:\n{}",
                self.i18n.tc("ui-achievements"),
                self.game_state.achievements.len(),
                self.i18n.tc("ui-last-unlocked"),
                self.game_state
                    .achievements
                    .iter()
                    .rev()
                    .take(5)
                    .map(|a| format!("  • {} - {}", a.name, a.description))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            self.input_buffer.clear();
            true
        } else if command == "tip" {
            let title = self.i18n.tc("sys-tip-title");
            let body = self.i18n.tc("sys-tip-body");
            self.show_popup(title, body, PopupType::Tip);
            self.last_output = self.i18n.tc("sys-tip-showing");
            self.input_buffer.clear();
            true
        } else if command.starts_with("help") {
            self.scroll = 0; // começa o guia do topo (PageUp/PageDown rolam a partir daqui)
            let args: Vec<&str> = command.split_whitespace().collect();
            if args.len() > 1 {
                let topic = args[1].to_lowercase();

                if let Some((cmd, desc, examples, tip)) = self.get_command_help(&topic) {
                    self.right_panel_mode = RightPanelMode::CommandHelp {
                        command: cmd,
                        description: desc,
                        examples,
                        tip,
                    };
                    self.last_output = self.t1("sys-help-cmd", "topic", topic);
                } else {
                    // Conteúdo e título vêm juntos da fonte única (sempre
                    // coerentes) e já no idioma ativo.
                    let (content, title) =
                        crate::game::distro_guide::DistroGuide::get(&topic, &self.i18n);
                    self.last_output = self.t1("sys-help-showing-title", "title", title.clone());
                    self.right_panel_mode = RightPanelMode::Help { content, title };
                }
            } else {
                self.right_panel_mode = RightPanelMode::Help {
                    content: self.i18n.tc("help-system-body"),
                    title: self.i18n.tc("help-system-title"),
                };
                self.last_output = self.i18n.tc("sys-help-showing");
            }
            self.input_buffer.clear();
            true
        } else {
            false
        }
    }

    /// Decide o modo do painel direito a partir do comando digitado.
    ///
    /// Vive na camada `app` (não no `core`): produz `RightPanelMode` (tipo de UI)
    /// e usa `i18n`. O `core::parser` apenas classifica e busca arquivos.
    ///
    /// Recebe `cmd_type` pronto: quem chama já classificou, e reclassificar aqui
    /// dobrava o trabalho no caminho quente (uma vez por tecla digitada).
    fn command_to_panel_mode(&self, input: &str, cmd_type: &CommandType) -> RightPanelMode {
        use crate::core::parser::CommandParser;

        let trimmed = input.trim();
        let current_dir = &self.current_dir;

        match cmd_type {
            CommandType::Dangerous => RightPanelMode::DangerZone {
                warning: self.i18n.tc(danger_warning_key(trimmed)),
                command: trimmed.to_string(),
            },

            CommandType::FileViewing => {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 2 {
                    let filename = parts[1];
                    let matches = CommandParser::find_matching_files(current_dir, filename);

                    if matches.len() == 1 {
                        RightPanelMode::FilePreview {
                            path: matches[0].clone(),
                            content: String::new(),
                            language: CommandParser::detect_language(filename),
                        }
                    } else if matches.len() > 1 {
                        let suggestions = matches
                            .iter()
                            .filter_map(|p| p.file_name()?.to_str().map(|s| s.to_string()))
                            .collect::<Vec<_>>()
                            .join("\n  → ");
                        RightPanelMode::FilePreview {
                            path: current_dir.join(filename),
                            content: format!(
                                "{}\n\n  → {}",
                                self.i18n.tc("sys-files-found"),
                                suggestions
                            ),
                            language: "text".to_string(),
                        }
                    } else {
                        RightPanelMode::FilePreview {
                            path: current_dir.join(filename),
                            content: self.t1("sys-file-not-found", "name", filename),
                            language: "text".to_string(),
                        }
                    }
                } else {
                    RightPanelMode::FileTree { path: current_dir.clone() }
                }
            }

            // Parte do último resumo conhecido em vez de zeros: evita um frame
            // com gauges zerados antes do `refresh_monitor` do chamador.
            CommandType::SystemMonitoring => RightPanelMode::resource_from(&self.system_summary),

            _ => RightPanelMode::FileTree { path: current_dir.clone() },
        }
    }

    /// Analisa o input em tempo real e atualiza o modo do painel direito
    fn analyze_input(&mut self) {
        use crate::core::parser::CommandParser;

        let input = self.input_buffer.trim();

        if self.ssh_session.is_some() {
            // Em modo SSH, desabilita a análise reativa de arquivos locais
            return;
        }

        if input.is_empty() {
            // Se o input estiver vazio, só volta para Welcome se NÂO estiver mostrando output de comando
            if !matches!(self.right_panel_mode, RightPanelMode::CommandOutput(_)) {
                self.right_panel_mode = RightPanelMode::Welcome;
            }
            self.danger_mode_active = false;
            return;
        }

        // Classifica uma única vez e repassa: `command_to_panel_mode` reaproveita.
        let cmd_type = CommandParser::classify_command(input);

        // Comandos especiais e easter eggs não mexem no painel agora — quem
        // decide é o `execute_command`.
        if matches!(cmd_type, CommandType::MunuxSpecial | CommandType::EasterEgg) {
            return;
        }

        // Determina o modo do painel (camada app, com i18n)
        let mode = self.command_to_panel_mode(input, &cmd_type);

        // O painel de árvore só aparece em comando de listagem de verdade. A
        // comparação é por token exato (`core::commands`): com `starts_with`,
        // `lsof` e `last` abriam a árvore.
        let base = input.split_whitespace().next().unwrap_or("");
        if matches!(mode, RightPanelMode::FileTree { .. })
            && !crate::core::commands::is_listing(base)
        {
            self.right_panel_mode = RightPanelMode::Welcome;
            self.danger_mode_active = false;
            return;
        }

        self.right_panel_mode = mode;

        // Atualiza o flag de perigo
        self.danger_mode_active = matches!(
            self.right_panel_mode,
            RightPanelMode::DangerZone { .. }
        );

        // Se estiver em modo monitor, atualiza as métricas (instância única).
        if matches!(self.right_panel_mode, RightPanelMode::ResourceMonitor { .. }) {
            self.refresh_monitor();
        }

        self.load_preview_content();
    }

    /// Preenche o conteúdo de um `FilePreview` que veio vazio do
    /// `command_to_panel_mode`.
    ///
    /// O conteúdo é memoizado por caminho: sem isso, digitar `cat arquivo.txt`
    /// relia o arquivo inteiro (até 1 MB) do disco **a cada caractere digitado**.
    fn load_preview_content(&mut self) {
        let RightPanelMode::FilePreview { path, content, .. } = &self.right_panel_mode else {
            return;
        };
        if !content.is_empty() {
            return; // já veio preenchido (mensagem de erro/sugestões)
        }
        let path = path.clone();

        let content = match &self.preview_cache {
            Some((cached_path, cached)) if *cached_path == path => cached.clone(),
            _ => match self.read_preview(&path) {
                Some(text) => {
                    self.preview_cache = Some((path.clone(), text.clone()));
                    text
                }
                None => return,
            },
        };

        let language = crate::core::parser::CommandParser::detect_language(
            path.file_name().and_then(|n| n.to_str()).unwrap_or(""),
        );
        self.right_panel_mode = RightPanelMode::FilePreview { path, content, language };
    }

    /// Lê um arquivo para preview e já anexa, traduzido, o aviso de corte que o
    /// `core` sinaliza como dado (`FilePreview::truncated_at`).
    fn read_preview(&self, path: &std::path::Path) -> Option<String> {
        use crate::core::filesystem::FileSystemManager;

        let preview = FileSystemManager::read_file_preview(path).ok()?;
        let Some(total) = preview.truncated_at else {
            return Some(preview.content);
        };
        Some(format!(
            "{}\n\n{}",
            self.t1("ui-file-too-large", "bytes", total.to_string()),
            preview.content
        ))
    }
    
    /// Mostra um popup (Ghost Mentor)
    pub fn show_popup(&mut self, title: String, content: String, popup_type: PopupType) {
        self.active_popup = Some(PopupMessage {
            title,
            content,
            popup_type,
        });
    }
    
    /// Fecha o popup ativo
    pub fn close_popup(&mut self) {
        self.active_popup = None;
    }
    
    /// Mostra popup de Level Up
    fn show_level_up_popup(&mut self, old_level: u32, new_level: u32) {
        let rank = self.game_state.get_rank(&self.i18n);
        let message = self.i18n.level_message(new_level);
        let mut args = FluentArgs::new();
        args.set("old", old_level);
        args.set("new", new_level);
        args.set("rank", rank);
        args.set("msg", message);
        let title = self.i18n.tc("sys-levelup-title");
        let body = self.i18n.t("sys-levelup-body", Some(&args));
        self.show_popup(title, body, PopupType::Success);
    }

    /// Mostra popup de conquista desbloqueada
    fn show_achievement_popup(&mut self, name: &str, description: &str) {
        let title = self.i18n.tc("sys-achievement-title");
        self.show_popup(
            title,
            format!("{}\n\n{}", name, description),
            PopupType::Success,
        );
    }

    /// Aplica os efeitos de desbloquear uma conquista: marca como última,
    /// concede o XP, abre o popup e — se `announce` — anexa a notificação ao
    /// output do terminal.
    fn award_achievement(&mut self, achievement: crate::game::state::Achievement, announce: bool) {
        self.game_state.last_achievement = Some(achievement.clone());
        self.game_state.add_xp(achievement.xp_reward);
        self.show_achievement_popup(&achievement.name, &achievement.description);
        if announce {
            let mut args = FluentArgs::new();
            args.set("name", achievement.name.clone());
            args.set("desc", achievement.description.clone());
            args.set("xp", achievement.xp_reward);
            let announce_msg = self.i18n.t("sys-achievement-announce", Some(&args));
            self.last_output = format!("{}\n\n{}", self.last_output, announce_msg);
        }
    }
    
    /// Anexa uma dica educativa quando a saída de erro casa um padrão conhecido.
    ///
    /// Os *matchers* inspecionam a saída do shell (que segue o locale do SO),
    /// por isso cobrem PT e EN; o **texto** da dica vem dos `.ftl` (locale do app).
    fn add_educational_hints(&self, command: &str, error_output: &str) -> String {
        let mut output = error_output.to_string();
        let lower = error_output.to_lowercase();

        let is_dir = lower.contains("diretório") || lower.contains("is a directory");
        let not_empty = lower.contains("não vazio") || lower.contains("not empty");
        let not_a_dir = lower.contains("não é um diretório") || lower.contains("not a directory");
        let permission = lower.contains("permissão negada") || lower.contains("permission denied");
        let not_found =
            lower.contains("comando não encontrado") || lower.contains("command not found");

        let hint_key = if command.starts_with("rm ") && is_dir {
            Some("hint-err-rm-isdir")
        } else if command.starts_with("rmdir ") && not_empty {
            Some("hint-err-rmdir-notempty")
        } else if command.starts_with("cat ") && is_dir {
            Some("hint-err-cat-isdir")
        } else if command.starts_with("cd ") && not_a_dir {
            Some("hint-err-cd-notdir")
        } else if command.starts_with("mkdir ") && command.contains('.') {
            Some("hint-err-mkdir-dots")
        } else if permission {
            Some("hint-err-permission")
        } else if not_found {
            Some("hint-err-notfound")
        } else {
            None
        };

        if let Some(key) = hint_key {
            output.push_str(&self.i18n.tc(key));
        }
        output
    }
    /// Retorna dados estruturados de ajuda para um comando
    fn get_command_help(&self, command: &str) -> Option<(String, String, Vec<String>, String)> {
        match command {
            "ls" => Some((
                "ls".to_string(),
                self.i18n.tc("help-ls-desc"),
                vec!["ls".to_string(), "ls -la".to_string(), "ls /home".to_string()],
                self.i18n.tc("help-ls-hint")
            )),
            "cd" => Some((
                "cd".to_string(),
                self.i18n.tc("help-cd-desc"),
                vec!["cd Documentos".to_string(), "cd ..".to_string(), "cd ~".to_string()],
                self.i18n.tc("help-cd-hint")
            )),
             "grep" => Some((
                "grep".to_string(),
                self.i18n.tc("help-grep-desc"),
                vec!["grep 'texto' arquivo.txt".to_string(), "cat arquivo | grep 'erro'".to_string()],
                self.i18n.tc("help-grep-hint")
            )),
            "cat" => Some((
                "cat".to_string(),
                self.i18n.tc("help-cat-desc"),
                vec!["cat arquivo.txt".to_string()],
                self.i18n.tc("help-cat-hint")
            )),
            "sudo" => Some((
                "sudo".to_string(),
                self.i18n.tc("help-sudo-desc"),
                vec!["sudo pacman -Syu".to_string(), "sudo reboot".to_string()],
                self.i18n.tc("help-sudo-hint")
            )),
            _ => None
        }
    }
    
    /// Extrai o caminho de um comando `cd ...` (sem o prefixo). Fonte única —
    /// antes o parse acontecia em dois lugares (cd local e cd remoto/SSH).
    fn parse_cd_arg(command: &str) -> &str {
        command.trim().strip_prefix("cd ").map(str::trim).unwrap_or("")
    }

    /// Trata comandos dentro de uma sessão SSH ativa (exit/cd/exec remoto).
    /// Extraído de `execute_command` para reduzir o God Object.
    fn handle_ssh_session(&mut self, command: &str) {
        // `exit`/`logout` encerram a sessão (tratado antes do borrow de ssh_session).
        if command == "exit" || command == "logout" {
            self.ssh_session = None;
            self.last_output = self.i18n.tc("sys-ssh-disconnected");
            self.right_panel_mode = RightPanelMode::Welcome;
            self.input_buffer.clear();
            return;
        }

        let Some(ssh_session) = &mut self.ssh_session else { return };

        if command.starts_with("cd ") {
            let path = Self::parse_cd_arg(command);
            // O empréstimo mutável termina no `map`: `describe_ssh_error` e
            // `t1` precisam de `&self`.
            let moved = ssh_session.change_dir(path).map(|_| ssh_session.remote_cwd.clone());

            match moved {
                Ok(cwd) => {
                    self.last_output = self.t1("sys-ssh-cd-ok", "dir", cwd);
                    self.right_panel_mode =
                        RightPanelMode::CommandOutput(self.last_output.clone());
                }
                Err(e) => {
                    // Passa pela tradução: o `core` devolve `SshError` tipado.
                    let msg = self.describe_ssh_error(&e);
                    self.last_output = self.t1("sys-error", "msg", msg);
                }
            }
        } else {
            // Mesma injeção de cores do shell local (git/ls/grep/pacman/yay/tree/ip).
            let cmd_to_run = Self::prepare_color_command(command);
            // O empréstimo mutável de `ssh_session` termina aqui: `sanitize_output`
            // precisa de `&self` (usa i18n para o aviso de truncamento).
            let executed = ssh_session.execute(&cmd_to_run).map(|(stdout, stderr, _code)| {
                let output = if !stdout.is_empty() { stdout } else { stderr };
                let prompt = format!(
                    "\x1b[1;36m{}@{} \x1b[0m\x1b[1;33m{}\x1b[0m$ \x1b[1m{}\x1b[0m",
                    ssh_session.user, ssh_session.host, ssh_session.remote_cwd, command
                );
                (prompt, output)
            });

            match executed {
                Ok((prompt, output)) => {
                    self.last_output = self.sanitize_output(&output);
                    self.right_panel_mode =
                        RightPanelMode::CommandOutput(format!("{}\n{}", prompt, self.last_output));
                }
                Err(e) => {
                    let mut args = FluentArgs::new();
                    args.set("msg", e.to_string());
                    self.last_output = self.i18n.t("sys-ssh-exec-error", Some(&args));
                }
            }
        }

        self.input_buffer.clear();
    }

    /// Limpa, limita e sanitiza o output para exibição no TUI. Devolve o texto
    /// pronto, já com o aviso de truncamento traduzido quando houve corte.
    ///
    /// O teto de [`MAX_OUTPUT_LINES`] não é cosmético: a saída fica guardada no
    /// `RightPanelMode::CommandOutput` e é **reconvertida de ANSI a cada frame**
    /// (toda tecla + tick de 1 s). Sem limite, um `cat` de arquivo grande fazia
    /// o custo por frame — e o consumo de memória — crescer sem fronteira.
    fn sanitize_output(&self, content: &str) -> String {
        // Passada única: conta o total e guarda só as primeiras linhas.
        let mut kept: Vec<String> = Vec::new();
        let mut total = 0usize;
        for line in content.lines() {
            total += 1;
            if kept.len() < MAX_OUTPUT_LINES {
                // Tabs saltam o cursor e `\r` sobrepõe texto no TUI.
                kept.push(line.replace('\t', "    ").replace('\r', ""));
            }
        }

        let mut out = kept.join("\n");
        if total > kept.len() {
            out.push('\n');
            out.push_str(&self.t1(
                "sys-output-truncated",
                "lines",
                (total - kept.len()).to_string(),
            ));
        }
        out
    }

    /// Injeta flags de cor para comandos comuns em ambientes sem TTY
    /// Injeta flags de cor (`--color=always` etc.) em comandos comuns quando
    /// ausentes. Separa base e argumentos com `split_once` (seguro p/ UTF-8, sem
    /// fatiamento por offset). Usada tanto no shell local quanto no SSH.
    fn prepare_color_command(command: &str) -> String {
        let trimmed = command.trim();
        if trimmed.is_empty() {
            return command.to_string();
        }

        let (cmd, args) = match trimmed.split_once(char::is_whitespace) {
            Some((c, a)) => (c, a.trim()),
            None => (trimmed, ""),
        };

        let colorized = match cmd {
            "git" => format!("git -c color.ui=always {}", args),
            "ls" if !trimmed.contains("--color") => format!("ls --color=always {}", args),
            "grep" if !trimmed.contains("--color") => format!("grep --color=always {}", args),
            "pacman" | "yay" if !trimmed.contains("--color") => {
                format!("{} --color=always {}", cmd, args)
            }
            "tree" if !trimmed.contains("-C") => format!("tree -C {}", args),
            "ip" if !trimmed.contains("-c") && !trimmed.contains("-color") => {
                format!("ip -c {}", args)
            }
            _ => return command.to_string(),
        };

        colorized.trim().to_string()
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new().expect("Falha ao criar aplicação")
    }
}
