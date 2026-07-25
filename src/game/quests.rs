// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

//! Quests/missões.
//!
//! Um objetivo é descrito por **dados**, não por uma variante de enum própria:
//! um [`Trigger`] (o que o comando precisa ter) e as chaves Fluent do texto de
//! progresso. Antes cada tipo de quest era uma variante, e acrescentar uma
//! exigia editar três `match` de 17 braços — `update_progress` (CC 46),
//! `get_progress_text` (CC 31) e `is_complete` (CC 18), 95 de complexidade
//! somada. Agora uma quest nova é uma entrada em [`generate_quests_for_level`].

use crate::i18n::I18n;
use fluent::{FluentArgs, FluentValue};
use serde::{Deserialize, Serialize};

/// O que um comando precisa ter para satisfazer um objetivo.
///
/// Os seis formatos cobrem todos os tipos de quest existentes, e nenhum deles
/// conhece a quest a que pertence.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Trigger {
    /// Qualquer comando conta.
    Any,
    /// A linha começa com este prefixo (`git`, `ssh`, `grep`, `vim`…).
    Prefix(String),
    /// A linha começa com **algum** destes prefixos (`pacman`/`apt`/`yay`).
    AnyPrefix(Vec<String>),
    /// Começa com o prefixo **e** menciona o alvo (`touch relatorio.txt`).
    PrefixMentioning { prefix: String, target: String },
    /// A linha contém este texto em qualquer posição (`|`).
    Contains(String),
    /// Começa com algum dos prefixos **e** termina com o sufixo (`./x.sh`).
    PrefixAndSuffix { prefixes: Vec<String>, suffix: String },
}

impl Trigger {
    /// `true` se `command` satisfaz este gatilho.
    pub fn matches(&self, command: &str) -> bool {
        match self {
            Trigger::Any => true,
            Trigger::Prefix(p) => command.starts_with(p.as_str()),
            Trigger::AnyPrefix(ps) => ps.iter().any(|p| command.starts_with(p.as_str())),
            Trigger::PrefixMentioning { prefix, target } => {
                command.starts_with(prefix.as_str()) && command.contains(target.as_str())
            }
            Trigger::Contains(needle) => command.contains(needle.as_str()),
            Trigger::PrefixAndSuffix { prefixes, suffix } => {
                command.ends_with(suffix.as_str())
                    && prefixes.iter().any(|p| command.starts_with(p.as_str()))
            }
        }
    }
}

/// Textos de progresso de um objetivo, como chaves Fluent.
///
/// Os argumentos disponíveis nas mensagens são sempre os mesmos — `$subject`,
/// `$current` e `$count` —, então acrescentar uma quest não exige um formato de
/// argumento novo.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProgressText {
    /// Chave exibida enquanto o objetivo está pendente.
    pub todo: String,
    /// Chave exibida quando concluído. Vazia quando a mesma mensagem serve para
    /// os dois estados (contadores já mostram `current/count`).
    #[serde(default)]
    pub done: String,
    /// Valor de `$subject`: nome de arquivo, diretório, editor, comando…
    #[serde(default)]
    pub subject: String,
}

impl ProgressText {
    /// Contador: a mesma mensagem nos dois estados, sem sujeito.
    fn counter(todo: &str) -> Self {
        Self { todo: todo.into(), done: String::new(), subject: String::new() }
    }

    /// Marcador pendente/concluído, sem sujeito.
    #[allow(dead_code)] // usado pelos testes e por quests futuras (git/ssh/pipe…)
    fn flag(todo: &str, done: &str) -> Self {
        Self { todo: todo.into(), done: done.into(), subject: String::new() }
    }

    /// Marcador com sujeito (`$subject`).
    fn about(todo: &str, done: &str, subject: &str) -> Self {
        Self { todo: todo.into(), done: done.into(), subject: subject.into() }
    }
}

/// Objetivo da quest.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QuestObjective {
    /// Realizar `target` vezes uma ação que case com `trigger`. Um marcador
    /// simples é apenas `target: 1`.
    Action {
        trigger: Trigger,
        target: u32,
        current: u32,
        text: ProgressText,
    },
    /// Alcançar um nível. Não depende de comando algum.
    ReachLevel { level: u32 },
}

impl QuestObjective {
    /// `true` se o objetivo já foi atingido.
    ///
    /// `ReachLevel` é avaliado em [`Quest::update_progress`] (depende do nível,
    /// não de um contador interno) e por isso nunca se declara completo sozinho.
    fn is_complete(&self) -> bool {
        match self {
            QuestObjective::Action { target, current, .. } => current >= target,
            QuestObjective::ReachLevel { .. } => false,
        }
    }

    /// Marcador: uma única ocorrência do gatilho conclui.
    fn once(trigger: Trigger, text: ProgressText) -> Self {
        QuestObjective::Action { trigger, target: 1, current: 0, text }
    }

    /// Contador: `target` ocorrências do gatilho concluem.
    fn times(trigger: Trigger, target: u32, text: ProgressText) -> Self {
        QuestObjective::Action { trigger, target, current: 0, text }
    }
}

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

impl Quest {
    /// Argumentos uniformes das mensagens de progresso.
    fn args(subject: &str, current: u32, count: u32) -> FluentArgs<'static> {
        let mut args = FluentArgs::new();
        args.set("subject", FluentValue::from(subject.to_string()));
        args.set("current", FluentValue::from(current));
        args.set("count", FluentValue::from(count));
        args
    }

    /// Retorna o progresso em texto.
    pub fn get_progress_text(&self, i18n: &I18n) -> String {
        match &self.objective {
            QuestObjective::ReachLevel { level } => {
                let mut args = FluentArgs::new();
                args.set("level", FluentValue::from(*level));
                i18n.t("quest-progress-reach-level", Some(&args))
            }
            QuestObjective::Action { target, current, text, .. } => {
                let key = if current >= target && !text.done.is_empty() {
                    &text.done
                } else {
                    &text.todo
                };
                i18n.t(key, Some(&Self::args(&text.subject, *current, *target)))
            }
        }
    }

    /// Verifica se a quest está completa.
    pub fn is_complete(&self) -> bool {
        self.objective.is_complete()
    }

    /// Atualiza o progresso da quest. Retorna `true` **apenas na transição**
    /// para completa, para que a recompensa seja concedida uma única vez.
    pub fn update_progress(&mut self, command: &str, current_level: u32) -> bool {
        if self.completed {
            return false;
        }
        let was_complete = self.is_complete();

        match &mut self.objective {
            QuestObjective::ReachLevel { level } => {
                if current_level >= *level {
                    self.completed = true;
                    return true;
                }
                return false;
            }
            QuestObjective::Action { trigger, target, current, .. } => {
                // O teto evita que o contador passe do alvo em quests já
                // concluídas mas ainda listadas.
                if trigger.matches(command) && current < target {
                    *current += 1;
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

/// Monta uma quest. Só existe para deixar [`generate_quests_for_level`] legível.
fn quest(
    id: &str,
    title_key: &str,
    desc_key: &str,
    objective: QuestObjective,
    xp_reward: u32,
    i18n: &I18n,
) -> Quest {
    Quest {
        id: id.to_string(),
        title: i18n.tc(title_key),
        description: i18n.tc(desc_key),
        objective,
        xp_reward,
        completed: false,
    }
}

/// Gera as quests iniciais ou novas quests baseadas no nível.
pub fn generate_quests_for_level(level: u32, i18n: &I18n) -> Vec<Quest> {
    match level {
        1..=4 => vec![
            quest(
                "intro_ls",
                "quest-explorer-title",
                "quest-explorer-desc",
                QuestObjective::once(
                    Trigger::Prefix("ls".into()),
                    ProgressText::about("quest-progress-run", "", "ls"),
                ),
                20,
                i18n,
            ),
            quest(
                "intro_pwd",
                "quest-location-title",
                "quest-location-desc",
                QuestObjective::once(
                    Trigger::Prefix("pwd".into()),
                    ProgressText::about("quest-progress-run", "", "pwd"),
                ),
                15,
                i18n,
            ),
            quest(
                "intro_mkdir",
                "quest-architect-title",
                "quest-architect-desc",
                QuestObjective::once(
                    Trigger::PrefixMentioning { prefix: "mkdir".into(), target: "munux".into() },
                    ProgressText::about(
                        "quest-progress-dir-create",
                        "quest-progress-dir-created",
                        "munux",
                    ),
                ),
                30,
                i18n,
            ),
        ],
        5..=9 => vec![
            quest(
                "apprentice_cat",
                "quest-reader-title",
                "quest-reader-desc",
                QuestObjective::once(
                    Trigger::PrefixMentioning { prefix: "cat".into(), target: "README".into() },
                    ProgressText::about(
                        "quest-progress-read-action",
                        "quest-progress-read",
                        "README",
                    ),
                ),
                40,
                i18n,
            ),
            quest(
                "apprentice_rm",
                "quest-cleaner-title",
                "quest-cleaner-desc",
                QuestObjective::once(
                    Trigger::PrefixMentioning { prefix: "rm".into(), target: "tmp".into() },
                    ProgressText::about(
                        "quest-progress-delete-action",
                        "quest-progress-deleted",
                        "tmp",
                    ),
                ),
                35,
                i18n,
            ),
        ],
        _ => vec![quest(
            "xp_grind",
            "quest-focus-title",
            "quest-focus-desc",
            QuestObjective::times(
                Trigger::Any,
                10,
                ProgressText::counter("quest-progress-any-command"),
            ),
            50,
            i18n,
        )],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::{I18n, Language};

    fn quest_with(objective: QuestObjective) -> Quest {
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
        let mut q = quest_with(QuestObjective::once(
            Trigger::Prefix("ls".into()),
            ProgressText::counter("quest-progress-run"),
        ));
        assert!(q.update_progress("ls -la", 1));
        assert!(q.completed);
    }

    #[test]
    fn execute_command_ignores_non_matching() {
        let mut q = quest_with(QuestObjective::once(
            Trigger::Prefix("ls".into()),
            ProgressText::counter("quest-progress-run"),
        ));
        assert!(!q.update_progress("cd /tmp", 1));
        assert!(!q.completed);
    }

    #[test]
    fn execute_any_commands_counts_up_to_target() {
        let mut q = quest_with(QuestObjective::times(
            Trigger::Any,
            3,
            ProgressText::counter("quest-progress-any-command"),
        ));
        assert!(!q.update_progress("ls", 1));
        assert!(!q.update_progress("pwd", 1));
        assert!(q.update_progress("cd x", 1)); // 3º comando completa
        assert!(q.completed);
    }

    #[test]
    fn counter_never_passes_the_target() {
        let mut q = quest_with(QuestObjective::times(
            Trigger::Any,
            2,
            ProgressText::counter("quest-progress-any-command"),
        ));
        for _ in 0..10 {
            q.update_progress("ls", 1);
        }
        let QuestObjective::Action { current, target, .. } = &q.objective else {
            panic!("o objetivo trocou de forma");
        };
        assert_eq!(current, target, "o contador não pode ultrapassar o alvo");
    }

    #[test]
    fn reach_level_completes_at_threshold() {
        let mut q = quest_with(QuestObjective::ReachLevel { level: 5 });
        assert!(!q.update_progress("seja o que for", 4));
        assert!(q.update_progress("seja o que for", 5));
        assert!(q.completed);
    }

    #[test]
    fn pipe_and_git_are_detected() {
        let mut p = quest_with(QuestObjective::once(
            Trigger::Contains("|".into()),
            ProgressText::flag("quest-progress-pipe-todo", "quest-progress-pipe-done"),
        ));
        assert!(p.update_progress("ps aux | grep init", 1));

        let mut g = quest_with(QuestObjective::once(
            Trigger::Prefix("git".into()),
            ProgressText::flag("quest-progress-git-todo", "quest-progress-git-done"),
        ));
        assert!(g.update_progress("git status", 1));
    }

    #[test]
    fn completion_triggers_only_once() {
        let mut q = quest_with(QuestObjective::once(
            Trigger::Prefix("git".into()),
            ProgressText::flag("quest-progress-git-todo", "quest-progress-git-done"),
        ));
        assert!(q.update_progress("git status", 1)); // transição -> true
        assert!(!q.update_progress("git log", 1)); // já completa -> false
    }

    #[test]
    fn script_trigger_needs_prefix_and_suffix() {
        let t = Trigger::PrefixAndSuffix {
            prefixes: vec!["./".into(), "bash".into()],
            suffix: ".sh".into(),
        };
        assert!(t.matches("./deploy.sh"));
        assert!(t.matches("bash deploy.sh"));
        assert!(!t.matches("./deploy")); // sem sufixo
        assert!(!t.matches("cat deploy.sh")); // sem prefixo
    }

    #[test]
    fn any_prefix_matches_each_alternative() {
        let t = Trigger::AnyPrefix(vec!["pacman".into(), "apt".into(), "yay".into()]);
        assert!(t.matches("pacman -Syu"));
        assert!(t.matches("yay -S neovim"));
        assert!(!t.matches("dnf install x"));
    }

    #[test]
    fn generated_quests_have_unique_ids_per_level() {
        let i18n = I18n::new(Language::PtBr);
        for level in [1, 5, 12] {
            let quests = generate_quests_for_level(level, &i18n);
            assert!(!quests.is_empty(), "nível {} ficou sem quests", level);
            for (i, a) in quests.iter().enumerate() {
                assert!(
                    !quests.iter().skip(i + 1).any(|b| b.id == a.id),
                    "id duplicado '{}' no nível {}",
                    a.id,
                    level
                );
            }
        }
    }

    /// Toda chave Fluent citada por uma quest gerada precisa existir nos dois
    /// locales. Sem esta guarda, um erro de digitação numa chave só apareceria
    /// como `[MISSING: ...]` na tela da pessoa.
    #[test]
    fn every_generated_quest_resolves_its_progress_text() {
        for lang in [Language::PtBr, Language::EnUs] {
            let i18n = I18n::new(lang);
            for level in [1, 5, 12] {
                for mut q in generate_quests_for_level(level, &i18n) {
                    let pending = q.get_progress_text(&i18n);
                    assert!(!pending.starts_with("[MISSING"), "{} -> {}", q.id, pending);

                    // E também no estado concluído.
                    if let QuestObjective::Action { target, current, .. } = &mut q.objective {
                        *current = *target;
                    }
                    let done = q.get_progress_text(&i18n);
                    assert!(!done.starts_with("[MISSING"), "{} -> {}", q.id, done);
                }
            }
        }
    }
}
