use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

use crate::app::App;
use crate::instance::SimStatus;
use crate::ui::widgets::status_badge;

pub fn render(f: &mut Frame<'_>, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(area);

    let mut items: Vec<ListItem> = Vec::new();
    if app.instances.is_empty() {
        items.push(ListItem::new(
            "No instances yet. Press n, l, or o to create one.",
        ));
    } else {
        for (idx, inst) in app.instances.iter().enumerate() {
            let marker = if idx == app.selected { ">" } else { " " };
            let line = format!(
                "{} sim-{} [{}] {} | {}",
                marker,
                inst.id,
                inst.sim_type.as_str(),
                status_badge(&inst.status),
                inst.progress_text()
            );
            let mut item = ListItem::new(line);
            if idx == app.selected {
                item = item.style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                );
            }
            items.push(item);
        }
    }

    let list = List::new(items).block(Block::default().title("Instances").borders(Borders::ALL));
    f.render_widget(list, chunks[0]);

    let detail_text = if let Some(inst) = app.selected_instance() {
        let status = match &inst.status {
            SimStatus::Pending => "pending",
            SimStatus::Running { .. } => "running",
            SimStatus::Completed => "completed",
        };
        format!(
            "ID: sim-{}\nType: {}\nStatus: {}\nSeed: {}\nTeams:\n- {}\n- {}{}\n\nOutcome:\n{}\n\nTip: Press Enter or v to open live detail view.",
            inst.id,
            inst.sim_type.as_str(),
            status,
            inst.seed,
            inst.teams.first().cloned().unwrap_or_default(),
            inst.teams.get(1).cloned().unwrap_or_default(),
            if inst.teams.len() > 2 {
                format!(
                    "\n- {}\n- {}",
                    inst.teams.get(2).cloned().unwrap_or_default(),
                    inst.teams.get(3).cloned().unwrap_or_default()
                )
            } else {
                String::new()
            },
            inst.outcome_summary()
        )
    } else {
        "No selection".to_string()
    };

    let details = Paragraph::new(detail_text)
        .block(
            Block::default()
                .title("Selected Instance")
                .borders(Borders::ALL),
        )
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(details, chunks[1]);
}
