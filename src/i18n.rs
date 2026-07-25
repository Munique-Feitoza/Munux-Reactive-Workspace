// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use crate::ui::theme::Stage;
use std::collections::HashMap;
use fluent::{FluentBundle, FluentResource, FluentArgs};
use include_dir::{include_dir, Dir};
use serde::{Deserialize, Serialize};
use unic_langid::LanguageIdentifier;

static LOCALES_DIR: Dir<'_> = include_dir!("locales");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    PtBr,
    EnUs,
}

impl Language {
    pub fn detect() -> Self {
        match sys_locale::get_locale() {
            Some(locale) if locale.starts_with("pt") => Language::PtBr,
            _ => Language::EnUs,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Language::PtBr => "pt-BR",
            Language::EnUs => "en-US",
        }
    }

    pub fn to_langid(self) -> LanguageIdentifier {
        self.as_str().parse().expect("Invalid LangID")
    }
}

pub struct I18n {
    pub lang: Language,
    bundles: HashMap<Language, FluentBundle<FluentResource>>,
}

impl I18n {
    pub fn new(lang: Language) -> Self {
        let mut bundles = HashMap::new();
        
        // Load all available locales
        for entry in LOCALES_DIR.dirs() {
            let lang_str = entry.path().file_name().unwrap().to_str().unwrap();
            let lang_enum = match lang_str {
                "pt-BR" => Language::PtBr,
                "en-US" => Language::EnUs,
                _ => continue,
            };

            let mut bundle = FluentBundle::new(vec![lang_enum.to_langid()]);
            
            if let Some(file) = entry.get_file(format!("{}/main.ftl", lang_str)) {
                let content = file.contents_utf8().expect("Invalid UTF-8 in FTL");
                let resource = FluentResource::try_new(content.to_string())
                    .expect("Failed to parse FTL");
                bundle.add_resource(resource).expect("Failed to add resource");
            }
            
            bundles.insert(lang_enum, bundle);
        }

        Self { lang, bundles }
    }

    /// Primary translation method with keys and optional arguments
    pub fn t(&self, key: &str, args: Option<&FluentArgs>) -> String {
        let bundle = self.bundles.get(&self.lang)
            .or_else(|| self.bundles.get(&Language::EnUs)) // Fallback to EN
            .expect("No bundles loaded");

        let msg = bundle.get_message(key)
            .or_else(|| {
                // If not found in current lang, try fallback (English)
                if self.lang != Language::EnUs {
                    self.bundles.get(&Language::EnUs)?.get_message(key)
                } else {
                    None
                }
            });

        match msg {
            Some(m) => {
                let pattern = m.value().expect("Message has no value");
                let mut errors = vec![];
                bundle.format_pattern(pattern, args, &mut errors).to_string()
            }
            None => format!("[MISSING: {}]", key),
        }
    }

    /// Shorthand for simple translations without arguments
    pub fn tc(&self, key: &str) -> String {
        self.t(key, None)
    }

    /// Traduz `key` com um único argumento string. Casos multi-argumento montam
    /// o `FluentArgs` na mão.
    pub fn t1(&self, key: &str, name: &'static str, value: impl Into<String>) -> String {
        let mut args = FluentArgs::new();
        args.set(name, value.into());
        self.t(key, Some(&args))
    }

    /// Carrega um **bloco de conteúdo** do locale atual, com fallback para
    /// en-US. `relative` é o caminho dentro do diretório do idioma
    /// (ex.: `"guides/arch.txt"`).
    ///
    /// Textos longos e formatados — guias de distro, arte ASCII dos easter eggs
    /// — não cabem bem no Fluent: valores multilinha exigem indentação de
    /// continuação, quebram com linhas em branco no meio e obrigariam a escapar
    /// as chaves `{}` que aparecem em exemplos de código. Ficam em arquivos
    /// `.txt` por idioma, embutidos no binário pelo mesmo `include_dir!`.
    ///
    /// Devolve `&'static str` porque o conteúdo é compilado junto ao binário.
    pub fn content(&self, relative: &str) -> Option<&'static str> {
        let read = |lang: Language| {
            LOCALES_DIR
                .get_file(format!("{}/{}", lang.as_str(), relative))
                .and_then(|file| file.contents_utf8())
        };
        read(self.lang).or_else(|| read(Language::EnUs))
    }

    /// Traduz `key` retornando `None` quando a chave não existe (em vez do
    /// sentinela `[MISSING: ...]`). Fonte única do teste de "chave ausente".
    pub fn try_t(&self, key: &str) -> Option<String> {
        let value = self.t(key, None);
        if value.starts_with("[MISSING") {
            None
        } else {
            Some(value)
        }
    }

    /// Implementation for specific helpers to maintain compatibility or ease of use
    pub fn xp_label(&self, current: u32, next: u32, percent: f64) -> String {
        format!("XP: {}/{} ({:.0}%)", current, next, percent)
    }

    pub fn rank_name(&self, level: u32) -> String {
        // Faixas vêm da fonte única `game::tier::Tier`.
        self.tc(crate::game::tier::Tier::from_level(level).rank_key())
    }

    pub fn welcome_title(&self) -> String { self.tc("ui-welcome-title") }
    pub fn resource_title(&self) -> String { self.tc("ui-resource-title") }
    pub fn danger_title(&self) -> String { self.tc("ui-danger-zone-title") }
    pub fn level_up_title(&self) -> String { self.tc("ui-level-up-title") }
    pub fn cpu_usage_label(&self) -> String { self.tc("ui-cpu-usage") }
    
    pub fn navigation_title(&self) -> String { self.tc("ui-navigation") }
    pub fn level_message(&self, level: u32) -> String {
        let key = match level {
            1 => "game-msg-level-1",
            2..=4 => "game-msg-level-2",
            5 => "game-msg-level-5",
            6..=9 => "game-msg-level-6",
            10 => "game-msg-level-10",
            11..=19 => "game-msg-level-11",
            20 => "game-msg-level-20",
            21..=29 => "game-msg-level-21",
            30 => "game-msg-level-30",
            31..=49 => "game-msg-level-31",
            50 => "game-msg-level-50",
            _ => "game-msg-level-default",
        };
        self.tc(key)
    }

    pub fn art_tag(&self, mode: &str) -> String {
        let key = format!("game-art-{}-tag", mode.to_lowercase());
        self.try_t(&key)
            .unwrap_or_else(|| format!("[{}]", mode.to_uppercase()))
    }

    /// Comandos sugeridos na tela inicial, conforme o estágio do jogador.
    ///
    /// As faixas vêm da fonte única [`Stage`]. Antes esta tabela cortava em 10
    /// enquanto todas as outras cortavam em 9: quem estava no nível 10 já era
    /// Aprendiz e já via o tema Hacker, mas continuava recebendo as sugestões
    /// do bloco iniciante.
    pub fn level_commands(&self, level: u32) -> Vec<(&'static str, String)> {
        match Stage::from_level(level) {
            Stage::Beginner => vec![
                ("ls", self.tc("hint-ls")),
                ("pwd", self.tc("hint-pwd")),
                ("mkdir [nome]", self.tc("hint-mkdir")),
            ],
            Stage::Terminal => vec![
                ("cat [arquivo]", self.tc("hint-cat")),
                ("rm [arquivo]", self.tc("hint-rm")),
                ("cp [orig] [dest]", self.tc("hint-cp")),
                ("mv [orig] [dest]", self.tc("hint-mv")),
            ],
            _ => vec![
                ("ssh [user]@[host]", self.tc("hint-ssh")),
                ("ps aux | grep ...", self.tc("hint-grep")),
                ("systemctl status ...", self.tc("hint-systemctl")),
            ],
        }
    }

    pub fn esc_to_back(&self) -> String { self.tc("ui-esc-to-back") }
    pub fn scroll_hint(&self) -> String { self.tc("ui-scroll-hint") }
    
    pub fn preview_title(&self, filename: &str) -> String {
        let mut args = FluentArgs::new();
        args.set("filename", filename);
        self.t("ui-preview-title", Some(&args))
    }

    pub fn achievement_info(&self, id: &str) -> (String, String) {
        let name_key = format!("achievement-{}-name", id);
        let desc_key = format!("achievement-{}-desc", id);
        (self.tc(&name_key), self.tc(&desc_key))
    }

    pub fn command_hint(&self, cmd: &str) -> Option<String> {
        self.try_t(&format!("hint-{}", cmd))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Chaves visíveis ao usuário introduzidas/externalizadas. O teste garante
    /// que existem nos DOIS locales (e, de quebra, que os `.ftl` parseiam — um
    /// erro de parse faria a chave cair no fallback `[MISSING: ...]`).
    const REQUIRED_KEYS: &[&str] = &[
        // patente
        "ui-next-rank", "ui-max-rank",
        // zona de perigo / confirmação
        "ui-attention-max", "ui-command-detected", "ui-risk", "ui-data-loss",
        "ui-unstable-system", "ui-irreversible-damage", "ui-available-actions",
        "ui-cancel-rec", "ui-execute-anyway", "ui-backup-tip",
        "sys-danger-confirm", "sys-danger-cancelled",
        // alias
        "sys-alias-none", "sys-alias-list-title", "sys-alias-removed", "sys-alias-missing",
        "sys-alias-usage", "sys-alias-no-spaces", "sys-alias-created",
        // tutorial
        "sys-tutorial-ended", "sys-tutorial-none", "sys-tutorial-started",
        "sys-tutorial-mode-title", "sys-tutorial-step-done-title",
        "sys-tutorial-complete-title", "sys-tutorial-complete-body",
        // benchmark
        "sys-bench-none", "sys-bench-cancelled", "sys-bench-result-title",
        "sys-bench-popup-title", "sys-bench-result", "sys-bench-start", "sys-bench-popup-body",
        // ssh
        "sys-error", "sys-ssh-disconnected", "sys-ssh-cd-ok", "sys-ssh-exec-error",
        "sys-ssh-connecting", "sys-ssh-connected", "sys-ssh-conn-title", "sys-ssh-conn-body",
        "sys-ssh-fail", "sys-ssh-fail-title", "sys-ssh-fail-body",
        // comandos especiais / help
        "sys-showing-stats", "sys-showing-quests", "sys-tip-title", "sys-tip-body",
        "sys-tip-showing", "sys-help-cmd", "sys-help-showing-title", "sys-help-showing",
        "help-system-title", "help-system-body",
        // execução de comandos
        "sys-cd-ok", "sys-cd-notfound", "sys-ls-listed", "sys-cmd-ok", "sys-cmd-error",
        "sys-cmd-exec-error", "sys-quest-complete",
        // level up / conquistas
        "sys-levelup-title", "sys-levelup-body", "sys-achievement-title", "sys-achievement-announce",
        // dicas educativas
        "hint-err-rm-isdir", "hint-err-rmdir-notempty", "hint-err-cat-isdir",
        "hint-err-cd-notdir", "hint-err-mkdir-dots", "hint-err-permission", "hint-err-notfound",
        // avisos da zona de perigo
        "danger-rm-root", "danger-rm-rf", "danger-rm", "danger-sudo", "danger-dd",
        "danger-fs", "danger-perm", "danger-power", "danger-generic",
        // diversos
        "ui-top-processes", "ui-browse-hint", "ui-back-to-normal",
        "sys-file-not-found", "sys-files-found",
        // v0.3.1 — strings que estavam fixas em português no código
        "sys-output-truncated", "ui-command-help-title", "ui-unnamed-file",
        "ui-danger-tip-label", "ui-popup-close", "ui-file-too-large",
        "sys-ssh-hostkey-mismatch", "sys-ssh-hostkey-unverifiable",
        "sys-ssh-err-connect", "sys-ssh-err-handshake", "sys-ssh-err-auth", "sys-ssh-err-nodir",
        "tutorial-header", "tutorial-hint", "tutorial-exit-note",
        "egg-cowsay-default",
        "guide-arch-title", "guide-debian-title", "guide-fedora-title",
        "guide-opensuse-title", "guide-general-title",
    ];

    /// Os dois locales precisam declarar exatamente o mesmo conjunto de chaves.
    ///
    /// A lista `REQUIRED_KEYS` protege as chaves que alguém lembrou de
    /// cadastrar; esta guarda é estrutural e pega o caso geral — uma chave nova
    /// adicionada só no pt-BR cairia calada no fallback em inglês (ou vice-versa).
    #[test]
    fn both_locales_declare_the_same_keys() {
        fn keys_of(lang: &str) -> std::collections::BTreeSet<String> {
            let file = LOCALES_DIR
                .get_file(format!("{lang}/main.ftl"))
                .unwrap_or_else(|| panic!("locale {lang} ausente"));
            file.contents_utf8()
                .expect("FTL não é UTF-8")
                .lines()
                .filter_map(|line| {
                    // Uma mensagem Fluent é `chave = valor` na coluna 0.
                    let line = line.trim_end();
                    if line.starts_with([' ', '\t', '#']) || line.is_empty() {
                        return None;
                    }
                    line.split_once('=').map(|(k, _)| k.trim().to_string())
                })
                .collect()
        }

        let pt = keys_of("pt-BR");
        let en = keys_of("en-US");

        let only_pt: Vec<_> = pt.difference(&en).collect();
        let only_en: Vec<_> = en.difference(&pt).collect();

        assert!(only_pt.is_empty(), "chaves só em pt-BR: {:?}", only_pt);
        assert!(only_en.is_empty(), "chaves só em en-US: {:?}", only_en);
        assert!(pt.len() > 200, "parse suspeito: só {} chaves encontradas", pt.len());
    }

    #[test]
    fn all_required_keys_resolve_in_both_locales() {
        for lang in [Language::PtBr, Language::EnUs] {
            let i18n = I18n::new(lang);
            for key in REQUIRED_KEYS {
                let value = i18n.tc(key);
                assert!(
                    !value.starts_with("[MISSING"),
                    "chave '{}' ausente/ilegível no locale {:?} (valor: {})",
                    key, lang, value
                );
            }
        }
    }
}
