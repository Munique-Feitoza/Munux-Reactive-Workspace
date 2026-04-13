// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

/// Renderiza o painel esquerdo (Terminal Ativo)
pub fn render_terminal_panel(frame: &mut Frame, app: &App, area: Rect) {
    use crate::ui::theme::Theme;
    
    let theme = app.game_state.get_theme();
    let symbol = Theme::get_prompt_symbol(app.game_state.level);
    
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(Theme::get_border_type(app.game_state.level))
        .borders(Borders::ALL)
        .border_type(Theme::get_border_type(app.game_state.level))
        .title(if let Some(ssh) = &app.ssh_session {
            format!(" 🌐 SSH ACTIVE: {}@{} ", ssh.user, ssh.host)
        } else {
            format!(" {} - {} ", app.i18n.tc("ui-terminal-active"), app.current_dir.display())
        })
        .border_style(Style::default().fg(
            if app.danger_mode_active {
                theme.danger
            } else if app.ssh_session.is_some() {
                Color::Cyan
            } else {
                theme.border
            }
        ));
    
    // Cria o prompt com cor baseada no nível
    let rank = app.game_state.get_rank(&app.i18n);
    
    let mut lines = Vec::new();
    
    // Mostra a saída do último comando se existir
    if !app.last_output.is_empty() {
        for line in app.last_output.lines() {
            let style = if line.starts_with("✓") {
                Style::default().fg(theme.success)
            } else if line.starts_with("✗") {
                Style::default().fg(theme.danger)
            } else {
                Style::default().fg(Color::Gray)
            };
            lines.push(Line::from(Span::styled(line, style)));
        }
        lines.push(Line::from("")); // Linha em branco após output
    }
    
    // Mostra histórico recente (últimos 5 comandos)
    let history_start = app.command_history.len().saturating_sub(5);
    for cmd in &app.command_history[history_start..] {
        let mut prompt_spans = vec![
            Span::styled(format!("{} [{}@munux]", symbol, rank), Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
        ];

        // Adiciona segmento Git se estiver em um repo
        if let Some(git) = &app.git_status {
            prompt_spans.push(Span::styled(" (", Style::default().fg(Color::DarkGray)));
            prompt_spans.push(Span::styled(&git.repo_name, Style::default().fg(Color::LightBlue)));
            prompt_spans.push(Span::styled(":", Style::default().fg(Color::DarkGray)));
            prompt_spans.push(Span::styled(&git.branch, Style::default().fg(Color::LightMagenta)));
            
            // Adiciona indicadores de modificação
            if git.staged > 0 {
                prompt_spans.push(Span::styled(format!(" +{}", git.staged), Style::default().fg(theme.success)));
            }
            if git.modified > 0 {
                prompt_spans.push(Span::styled(format!(" ~{}", git.modified), Style::default().fg(Color::Yellow)));
            }
            if git.untracked > 0 {
                prompt_spans.push(Span::styled(format!(" ?{}", git.untracked), Style::default().fg(Color::Red)));
            }
            
            // Adiciona indicadores de sync (ahead/behind)
            if git.ahead > 0 {
                prompt_spans.push(Span::styled(format!(" ↑{}", git.ahead), Style::default().fg(Color::Cyan)));
            }
            if git.behind > 0 {
                prompt_spans.push(Span::styled(format!(" ↓{}", git.behind), Style::default().fg(Color::Red)));
            }

            prompt_spans.push(Span::styled(")", Style::default().fg(Color::DarkGray)));
        }

        prompt_spans.push(Span::styled("$ ", Style::default().fg(theme.primary)));
        prompt_spans.push(Span::raw(cmd));
        
        lines.push(Line::from(prompt_spans));
    }
    
    // Linha de input atual com syntax highlighting e Git status
    // Linha de input atual
    let mut input_spans = if let Some(ssh) = &app.ssh_session {
        vec![
            Span::styled("🌐 ", Style::default().fg(Color::Cyan)),
            Span::styled(format!("{}@{}", ssh.user, ssh.host), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled(":", Style::default().fg(Color::DarkGray)),
            Span::styled(&ssh.remote_cwd, Style::default().fg(Color::Yellow)),
            Span::styled("$ ", Style::default().fg(Color::Cyan)),
        ]
    } else {
        vec![
            Span::styled(format!("{} [{}@munux]", symbol, rank), Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
        ]
    };

    if app.ssh_session.is_none() {
        if let Some(git) = &app.git_status {
            input_spans.push(Span::styled(" (", Style::default().fg(Color::DarkGray)));
            input_spans.push(Span::styled(&git.repo_name, Style::default().fg(Color::LightBlue)));
            input_spans.push(Span::styled(":", Style::default().fg(Color::DarkGray)));
            input_spans.push(Span::styled(&git.branch, Style::default().fg(Color::LightMagenta)));
            
            // Adiciona indicadores de modificação
            if git.staged > 0 {
                input_spans.push(Span::styled(format!(" +{}", git.staged), Style::default().fg(theme.success)));
            }
            if git.modified > 0 {
                input_spans.push(Span::styled(format!(" ~{}", git.modified), Style::default().fg(Color::Yellow)));
            }
            if git.untracked > 0 {
                input_spans.push(Span::styled(format!(" ?{}", git.untracked), Style::default().fg(Color::Red)));
            }

            // Adiciona indicadores de sync (ahead/behind)
            if git.ahead > 0 {
                input_spans.push(Span::styled(format!(" ↑{}", git.ahead), Style::default().fg(Color::Cyan)));
            }
            if git.behind > 0 {
                input_spans.push(Span::styled(format!(" ↓{}", git.behind), Style::default().fg(Color::Red)));
            }

            input_spans.push(Span::styled(")", Style::default().fg(Color::DarkGray)));
        }

        input_spans.push(Span::styled("$ ", Style::default().fg(theme.primary)));
    }
    
    // Adiciona o input com cores baseadas em validação
    input_spans.extend(colorize_input(&app.input_buffer, &theme));
    
    input_spans.push(Span::styled(
        "█",
        Style::default()
            .fg(theme.get_cursor_color())
            .add_modifier(Modifier::SLOW_BLINK),
    ));
    
    lines.push(Line::from(input_spans));
    
    // Mostra warning se estiver em modo de perigo
    if app.danger_mode_active {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled(
                format!("⚠️ {}: {}", app.i18n.tc("ui-warning"), app.i18n.tc("sys-destructive-detected")),
                Style::default()
                    .fg(Color::Red)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));

    }
    
    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });
    
    frame.render_widget(paragraph, area);
}

/// Coloriza o input baseado na validade do comando
fn colorize_input<'a>(input: &'a str, theme: &crate::ui::theme::Theme) -> Vec<Span<'a>> {
    if input.is_empty() {
        return vec![];
    }
    
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.is_empty() {
        return vec![Span::raw(input)];
    }
    
    let command = parts[0];
    
    // Lista de comandos válidos conhecidos (expandida com gerenciadores de pacotes)
    let valid_commands = [
        // Navegação e arquivos
        "ls", "cd", "pwd", "cat", "nano", "vim", "vi", "emacs", "mkdir", "touch", "rm", "rmdir",
        "cp", "mv", "echo", "grep", "find", "locate", "top", "htop", "ps", "free", "df", "du",
        "chmod", "chown", "chgrp", "ln", "file", "stat", "tree",
        // Rede
        "curl", "wget", "ping", "ssh", "scp", "rsync", "netstat", "ip", "ifconfig", "nmap",
        // Compressão
        "tar", "zip", "unzip", "gzip", "gunzip", "bzip2", "bunzip2", "7z", "rar", "unrar",
        // Texto
        "man", "help", "clear", "exit", "history", "which", "whereis", "head", "tail",
        "less", "more", "sed", "awk", "sort", "uniq", "wc", "diff", "patch", "cut", "tr",
        // Git
        "git",
        // Gerenciadores de Pacotes - Arch/Manjaro
        "pacman", "yay", "paru", "pamac", "makepkg",
        // Gerenciadores de Pacotes - Debian/Ubuntu
        "apt", "apt-get", "apt-cache", "aptitude", "dpkg", "add-apt-repository",
        // Gerenciadores de Pacotes - Fedora/RHEL
        "dnf", "yum", "rpm",
        // Gerenciadores de Pacotes - openSUSE
        "zypper",
        // Gerenciadores de Pacotes - Universal
        "snap", "flatpak", "appimage",
        // Sistema
        "systemctl", "service", "journalctl", "dmesg", "uname", "hostname", "uptime",
        "reboot", "shutdown", "poweroff", "halt",
        // Munux especiais
        "stats", "quests", "missions", "achievements", "xp",
        // Easter eggs
        "sl", "cowsay", "fortune", "matrix", "hack", "konami",
    ];
    
    // Comandos perigosos
    let dangerous_commands = ["rm", "sudo", "dd", "mkfs", "fdisk", "kill", "killall", 
                             "reboot", "shutdown", "poweroff", "halt"];
    
    let mut spans = Vec::new();
    let mut current_pos = 0;
    
    for (i, part) in parts.iter().enumerate() {
        // Adiciona espaços entre palavras
        if i > 0 {
            if let Some(space_start) = input[current_pos..].find(part) {
                let spaces = &input[current_pos..current_pos + space_start];
                spans.push(Span::raw(spaces));
                current_pos += space_start;
            }
        }
        
        if i == 0 {
            // Primeira palavra é o comando
            let color = if dangerous_commands.contains(&command) {
                theme.danger
            } else if valid_commands.contains(&command) {
                theme.success
            } else {
                theme.warning // Comando desconhecido/inválido
            };
            
            spans.push(Span::styled(
                *part,
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ));
        } else {
            // Argumentos em cor neutra
            let color = if part.starts_with('-') {
                theme.accent // Flags/opções
            } else if part.contains('/') || part.contains('.') {
                theme.secondary // Caminhos/arquivos
            } else {
                theme.text
            };
            
            spans.push(Span::styled(*part, Style::default().fg(color)));
        }
        
        current_pos += part.len();
    }
    
    // Adiciona qualquer texto restante (espaços finais, etc)
    if current_pos < input.len() {
        spans.push(Span::raw(&input[current_pos..]));
    }
    
    spans
}
