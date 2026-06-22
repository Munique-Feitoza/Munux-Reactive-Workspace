// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

//! Fonte única do catálogo de comandos conhecidos.
//!
//! Antes desta tabela, a "lista de comandos" vivia (divergente) em quatro
//! lugares: `parser::classify_command`, `completion::BUILTIN_COMMANDS`,
//! `terminal::valid_commands` e `terminal::dangerous_commands`. Manter tudo
//! aqui garante que classificação, autocomplete e coloração nunca discordem.
//!
//! `rm` NÃO está na tabela: depende das flags (ver `parser::classify_command`).

use crate::core::parser::CommandType;

/// (nome do comando, tipo). Ordem irrelevante; busca é por igualdade exata.
pub const COMMANDS: &[(&str, CommandType)] = &[
    // Navegação
    ("cd", CommandType::Navigation),
    ("ls", CommandType::Navigation),
    ("ll", CommandType::Navigation),
    ("la", CommandType::Navigation),
    ("pwd", CommandType::Navigation),
    ("dirs", CommandType::Navigation),
    ("pushd", CommandType::Navigation),
    ("popd", CommandType::Navigation),
    ("tree", CommandType::Navigation),
    // Operações de arquivo
    ("mkdir", CommandType::FileOperation),
    ("touch", CommandType::FileOperation),
    ("cp", CommandType::FileOperation),
    ("mv", CommandType::FileOperation),
    ("rmdir", CommandType::FileOperation),
    ("ln", CommandType::FileOperation),
    // Visualização de arquivos
    ("cat", CommandType::FileViewing),
    ("less", CommandType::FileViewing),
    ("more", CommandType::FileViewing),
    ("head", CommandType::FileViewing),
    ("tail", CommandType::FileViewing),
    ("nano", CommandType::FileViewing),
    ("vim", CommandType::FileViewing),
    ("vi", CommandType::FileViewing),
    ("emacs", CommandType::FileViewing),
    // Monitoramento
    ("top", CommandType::SystemMonitoring),
    ("htop", CommandType::SystemMonitoring),
    ("ps", CommandType::SystemMonitoring),
    ("free", CommandType::SystemMonitoring),
    ("df", CommandType::SystemMonitoring),
    ("du", CommandType::SystemMonitoring),
    ("vmstat", CommandType::SystemMonitoring),
    ("iostat", CommandType::SystemMonitoring),
    ("uptime", CommandType::SystemMonitoring),
    // Busca / inspeção
    ("grep", CommandType::Search),
    ("find", CommandType::Search),
    ("locate", CommandType::Search),
    ("which", CommandType::Search),
    ("whereis", CommandType::Search),
    ("file", CommandType::Search),
    ("stat", CommandType::Search),
    // Git
    ("git", CommandType::VersionControl),
    // Gerenciadores de pacotes
    ("pacman", CommandType::PackageManager),
    ("yay", CommandType::PackageManager),
    ("paru", CommandType::PackageManager),
    ("pamac", CommandType::PackageManager),
    ("makepkg", CommandType::PackageManager),
    ("apt", CommandType::PackageManager),
    ("apt-get", CommandType::PackageManager),
    ("apt-cache", CommandType::PackageManager),
    ("aptitude", CommandType::PackageManager),
    ("dpkg", CommandType::PackageManager),
    ("add-apt-repository", CommandType::PackageManager),
    ("dnf", CommandType::PackageManager),
    ("yum", CommandType::PackageManager),
    ("rpm", CommandType::PackageManager),
    ("zypper", CommandType::PackageManager),
    ("snap", CommandType::PackageManager),
    ("flatpak", CommandType::PackageManager),
    ("appimage", CommandType::PackageManager),
    // Ferramentas de rede
    ("ping", CommandType::NetworkTools),
    ("curl", CommandType::NetworkTools),
    ("wget", CommandType::NetworkTools),
    ("ssh", CommandType::NetworkTools),
    ("scp", CommandType::NetworkTools),
    ("rsync", CommandType::NetworkTools),
    ("netstat", CommandType::NetworkTools),
    ("ip", CommandType::NetworkTools),
    ("ifconfig", CommandType::NetworkTools),
    ("nmap", CommandType::NetworkTools),
    // Compressão / arquivamento
    ("tar", CommandType::Compression),
    ("zip", CommandType::Compression),
    ("unzip", CommandType::Compression),
    ("gzip", CommandType::Compression),
    ("gunzip", CommandType::Compression),
    ("bzip2", CommandType::Compression),
    ("bunzip2", CommandType::Compression),
    ("7z", CommandType::Compression),
    ("rar", CommandType::Compression),
    ("unrar", CommandType::Compression),
    // Processamento de texto
    ("sed", CommandType::TextProcessing),
    ("awk", CommandType::TextProcessing),
    ("cut", CommandType::TextProcessing),
    ("sort", CommandType::TextProcessing),
    ("uniq", CommandType::TextProcessing),
    ("wc", CommandType::TextProcessing),
    ("tr", CommandType::TextProcessing),
    ("diff", CommandType::TextProcessing),
    ("patch", CommandType::TextProcessing),
    ("echo", CommandType::TextProcessing),
    ("man", CommandType::TextProcessing),
    ("history", CommandType::TextProcessing),
    // Administração do sistema
    ("systemctl", CommandType::SystemAdmin),
    ("service", CommandType::SystemAdmin),
    ("journalctl", CommandType::SystemAdmin),
    ("dmesg", CommandType::SystemAdmin),
    ("uname", CommandType::SystemAdmin),
    ("hostname", CommandType::SystemAdmin),
    // Comandos perigosos (bloqueados no modo seguro e pintados de vermelho)
    ("rm", CommandType::FileOperation), // base; `classify_command` eleva a Dangerous com flags
    ("sudo", CommandType::Dangerous),
    ("dd", CommandType::Dangerous),
    ("mkfs", CommandType::Dangerous),
    ("fdisk", CommandType::Dangerous),
    ("parted", CommandType::Dangerous),
    ("chmod", CommandType::Dangerous),
    ("chown", CommandType::Dangerous),
    ("chgrp", CommandType::Dangerous),
    ("kill", CommandType::Dangerous),
    ("killall", CommandType::Dangerous),
    ("reboot", CommandType::Dangerous),
    ("shutdown", CommandType::Dangerous),
    ("poweroff", CommandType::Dangerous),
    ("halt", CommandType::Dangerous),
    // Comandos especiais do Munux / da própria app
    ("stats", CommandType::MunuxSpecial),
    ("quests", CommandType::MunuxSpecial),
    ("missions", CommandType::MunuxSpecial),
    ("achievements", CommandType::MunuxSpecial),
    ("help", CommandType::MunuxSpecial),
    ("tip", CommandType::MunuxSpecial),
    ("tutorial", CommandType::MunuxSpecial),
    ("benchmark", CommandType::MunuxSpecial),
    ("alias", CommandType::MunuxSpecial),
    ("unalias", CommandType::MunuxSpecial),
    ("clear", CommandType::MunuxSpecial),
    ("cls", CommandType::MunuxSpecial),
    ("exit", CommandType::MunuxSpecial),
    ("quit", CommandType::MunuxSpecial),
    ("logout", CommandType::MunuxSpecial),
    // Easter eggs (a execução real fica em `game::easter_eggs`)
    ("sl", CommandType::EasterEgg),
    ("cowsay", CommandType::EasterEgg),
    ("fortune", CommandType::EasterEgg),
    ("matrix", CommandType::EasterEgg),
    ("hack", CommandType::EasterEgg),
    ("konami", CommandType::EasterEgg),
];

/// Tipo do comando base (primeira palavra), se conhecido na tabela.
pub fn command_type(first_word: &str) -> Option<CommandType> {
    COMMANDS
        .iter()
        .find(|(name, _)| *name == first_word)
        .map(|(_, ty)| ty.clone())
}

/// Nomes de todos os comandos conhecidos (para autocomplete/builtins).
pub fn names() -> impl Iterator<Item = &'static str> {
    COMMANDS.iter().map(|(name, _)| *name)
}
