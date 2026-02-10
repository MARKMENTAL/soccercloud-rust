use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Tabs};

use crate::app::App;
use crate::instance::SimStatus;

pub fn render(f: &mut Frame<'_>, area: Rect, app: &App) {
    let Some(inst) = app.selected_instance() else {
        let empty = Paragraph::new("No selected instance")
            .block(Block::default().title("Detail").borders(Borders::ALL));
        f.render_widget(empty, area);
        return;
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(6),
            Constraint::Length(12),
        ])
        .split(area);

    let score = Paragraph::new(inst.scoreboard.clone())
        .block(
            Block::default()
                .title(format!("sim-{} Scoreboard", inst.id))
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black));
    f.render_widget(score, chunks[0]);

    let middle = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(chunks[1]);

    let logs: Vec<ListItem> = inst
        .logs
        .iter()
        .rev()
        .take((middle[0].height as usize).saturating_sub(2))
        .rev()
        .map(|line| ListItem::new(line.clone()))
        .collect();
    let log_widget =
        List::new(logs).block(Block::default().title("Live Log").borders(Borders::ALL));
    f.render_widget(log_widget, middle[0]);

    let mut right_lines: Vec<ListItem> = Vec::new();
    let status = match &inst.status {
        SimStatus::Pending => "pending",
        SimStatus::Running { .. } => "running",
        SimStatus::Completed => "completed",
    };
    right_lines.push(ListItem::new(format!("Status: {}", status)));
    right_lines.push(ListItem::new(format!("Seed: {}", inst.seed)));
    right_lines.push(ListItem::new(format!("Mode: {}", inst.sim_type.as_str())));
    right_lines.push(ListItem::new(""));
    right_lines.push(ListItem::new("Teams:"));
    for t in &inst.teams {
        right_lines.push(ListItem::new(format!("- {}", t)));
    }
    let side = List::new(right_lines).block(
        Block::default()
            .title("Instance Info")
            .borders(Borders::ALL),
    );
    f.render_widget(side, middle[1]);

    let bottom = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(34),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(chunks[2]);

    let stats = List::new(
        inst.stats_lines
            .iter()
            .map(|s| ListItem::new(s.clone()))
            .collect::<Vec<_>>(),
    )
    .block(Block::default().title("Stats").borders(Borders::ALL));
    let comp = List::new(
        inst.competition_lines
            .iter()
            .map(|s| ListItem::new(s.clone()))
            .collect::<Vec<_>>(),
    )
    .block(Block::default().title("Competition").borders(Borders::ALL));
    let hist = List::new(
        inst.history_lines
            .iter()
            .map(|s| ListItem::new(s.clone()))
            .collect::<Vec<_>>(),
    )
    .block(Block::default().title("History").borders(Borders::ALL));

    f.render_widget(stats, bottom[0]);
    f.render_widget(comp, bottom[1]);
    f.render_widget(hist, bottom[2]);

    let tabs = Tabs::new(vec!["Dashboard", "Detail"]).select(1);
    let _ = tabs;
}
