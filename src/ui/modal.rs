use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph};

use crate::app::{App, CreateDraft};
use crate::data::TEAMS;

pub fn render(f: &mut Frame<'_>, area: Rect, app: &App, draft: &CreateDraft) {
    let popup = centered_rect(70, 70, area);
    f.render_widget(Clear, popup);

    let frame = Block::default()
        .title("Create Simulation")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White).bg(Color::Black));
    f.render_widget(frame, popup);

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(4),
            Constraint::Length(2),
        ])
        .split(popup);

    let top = Paragraph::new(format!(
        "Mode: {} | next-seed={} | select slot with up/down",
        draft.mode_label(),
        app.next_instance_seed_preview()
    ));
    f.render_widget(top, inner[0]);

    let mut rows = Vec::with_capacity(draft.slots.len());
    for (i, slot) in draft.slots.iter().enumerate() {
        let slot_name = slot_label(draft.slots.len(), i);
        let content = if slot.is_cpu {
            format!("{}: CPU auto-fill", slot_name)
        } else {
            format!("{}: MANUAL -> {}", slot_name, TEAMS[slot.team_idx])
        };
        let mut item = ListItem::new(content);
        if i == draft.selected_slot {
            item = item.style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            );
        }
        rows.push(item);
    }
    let list = List::new(rows).block(Block::default().title("Team Slots").borders(Borders::ALL));
    f.render_widget(list, inner[1]);

    let help = Paragraph::new(
        "m=manual, p=cpu, [ / ] or left/right change manual team, Enter=create, Esc=cancel",
    );
    f.render_widget(help, inner[2]);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1]);

    horizontal[1]
}

fn slot_label(total: usize, i: usize) -> &'static str {
    if total == 2 {
        match i {
            0 => "Home",
            1 => "Away",
            _ => "Slot",
        }
    } else {
        match i {
            0 => "Team A",
            1 => "Team B",
            2 => "Team C",
            3 => "Team D",
            _ => "Slot",
        }
    }
}
