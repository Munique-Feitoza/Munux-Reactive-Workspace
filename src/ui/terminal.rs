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
    
    // Segmento Git montado uma única vez (mesmo repo p/ histórico e input).
    let git_segment = app
        .git_status
        .as_ref()
        .map(|git| git_segment_spans(git, &theme));

    // Mostra histórico recente (últimos 5), nunca antes do ponto de "clear".
    let history_start = app
        .command_history
        .len()
        .saturating_sub(5)
        .max(app.history_view_start.min(app.command_history.len()));
    for cmd in &app.command_history[history_start..] {
        let mut prompt_spans = vec![local_prompt_prefix(symbol, &rank, &theme)];

        if let Some(seg) = &git_segment {
            prompt_spans.extend(seg.iter().cloned());
        }

        prompt_spans.push(prompt_dollar(&theme));
        prompt_spans.push(Span::raw(cmd));

        lines.push(Line::from(prompt_spans));
    }

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
        vec![local_prompt_prefix(symbol, &rank, &theme)]
    };

    if app.ssh_session.is_none() {
        if let Some(seg) = &git_segment {
            input_spans.extend(seg.iter().cloned());
        }

        input_spans.push(prompt_dollar(&theme));
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

/// Span do prefixo do prompt local: `{símbolo} [{rank}@munux]`. Fonte única —
/// antes era montado no prompt do histórico e no do input.
fn local_prompt_prefix(
    symbol: &str,
    rank: &str,
    theme: &crate::ui::theme::Theme,
) -> Span<'static> {
    Span::styled(
        format!("{} [{}@munux]", symbol, rank),
        Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
    )
}

/// Span do sufixo `$ ` do prompt local.
fn prompt_dollar(theme: &crate::ui::theme::Theme) -> Span<'static> {
    Span::styled("$ ", Style::default().fg(theme.primary))
}

/// Monta os spans do segmento Git do prompt: `(repo:branch +s ~m ?u ↑a ↓b)`.
/// Fonte única — antes este bloco era copiado byte-a-byte no prompt do histórico
/// e no prompt do input atual.
fn git_segment_spans(
    git: &crate::core::git::GitStatus,
    theme: &crate::ui::theme::Theme,
) -> Vec<Span<'static>> {
    let mut spans = vec![
        Span::styled(" (", Style::default().fg(Color::DarkGray)),
        Span::styled(git.repo_name.clone(), Style::default().fg(Color::LightBlue)),
        Span::styled(":", Style::default().fg(Color::DarkGray)),
        Span::styled(git.branch.clone(), Style::default().fg(Color::LightMagenta)),
    ];

    if git.staged > 0 {
        spans.push(Span::styled(format!(" +{}", git.staged), Style::default().fg(theme.success)));
    }
    if git.modified > 0 {
        spans.push(Span::styled(format!(" ~{}", git.modified), Style::default().fg(Color::Yellow)));
    }
    if git.untracked > 0 {
        spans.push(Span::styled(format!(" ?{}", git.untracked), Style::default().fg(Color::Red)));
    }
    if git.ahead > 0 {
        spans.push(Span::styled(format!(" ↑{}", git.ahead), Style::default().fg(Color::Cyan)));
    }
    if git.behind > 0 {
        spans.push(Span::styled(format!(" ↓{}", git.behind), Style::default().fg(Color::Red)));
    }

    spans.push(Span::styled(")", Style::default().fg(Color::DarkGray)));
    spans
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
    
    // Classificação derivada da fonte única (`core::commands`): a cor do comando
    // segue exatamente o classificador, eliminando as antigas listas divergentes.
    // Usa o input completo para captar `rm -rf` como perigoso.
    let cmd_type = crate::core::parser::CommandParser::classify_command(input);

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
            // Primeira palavra é o comando — cor pela classificação única.
            let color = match cmd_type {
                crate::core::parser::CommandType::Dangerous => theme.danger,
                crate::core::parser::CommandType::Unknown => theme.warning,
                _ => theme.success,
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
