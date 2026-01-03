// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use crate::app::App;
use ratatui::{
    backend::Backend,
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
        .title(format!(" Terminal Ativo - {} ", app.current_dir.display()))
        .border_style(Style::default().fg(
            if app.danger_mode_active {
                theme.danger
            } else {
                theme.border
            }
        ));
    
    // Cria o prompt com cor baseada no nível
    let rank = app.game_state.get_rank();
    
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
        lines.push(Line::from(vec![
            Span::styled(
                format!("{} [{}@munux]$ ", symbol, rank),
                Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
            ),
            Span::raw(cmd),
        ]));
    }
    
    // Linha de input atual com syntax highlighting
    let mut input_spans = vec![
        Span::styled(
            format!("{} [{}@munux]$ ", symbol, rank),
            Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
        ),
    ];
    
    // Adiciona o input com cores baseadas em validação
    input_spans.extend(colorize_input(&app.input_buffer, &theme));
    
    input_spans.push(Span::styled(
        "█",
        Style::default()
            .fg(theme.accent)
            .add_modifier(Modifier::SLOW_BLINK),
    ));
    
    lines.push(Line::from(input_spans));
    
    // Mostra warning se estiver em modo de perigo
    if app.danger_mode_active {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled(
                "⚠ AVISO: Comando potencialmente destrutivo detectado!",
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
