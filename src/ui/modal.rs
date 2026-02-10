use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph};

use crate::app::{App, CreateDraft, OverlayModal};
use crate::data::TEAMS;

pub fn render_create(f: &mut Frame<'_>, area: Rect, app: &App, draft: &CreateDraft) {
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

pub fn render_overlay(f: &mut Frame<'_>, area: Rect, app: &App, modal: OverlayModal) {
    let popup = centered_rect(90, 86, area);
    f.render_widget(Clear, popup);

    let frame = Block::default()
        .title(modal.title())
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White).bg(Color::Black));
    f.render_widget(frame, popup);

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(4),
            Constraint::Length(1),
        ])
        .split(popup);

    let Some(inst) = app.selected_instance() else {
        f.render_widget(Paragraph::new("No selected instance"), inner[1]);
        return;
    };

    let all_lines: Vec<String> = match modal {
        OverlayModal::Stats => {
            if inst.stats_lines.is_empty() {
                vec!["No stats available yet. Start a simulation first.".to_string()]
            } else {
                inst.stats_lines.clone()
            }
        }
        OverlayModal::Competition => {
            if inst.competition_lines.is_empty() {
                vec!["No standings/bracket available for this simulation yet.".to_string()]
            } else {
                inst.competition_lines.clone()
            }
        }
        OverlayModal::History => {
            if inst.history_lines.is_empty() {
                vec!["No history entries yet.".to_string()]
            } else {
                inst.history_lines.clone()
            }
        }
    };

    let viewport = (inner[1].height as usize).saturating_sub(2).max(1);
    let max_start = all_lines.len().saturating_sub(viewport);
    let start = app.overlay_scroll.min(max_start);
    let end = (start + viewport).min(all_lines.len());
    let visible = &all_lines[start..end];
    let items: Vec<ListItem> = visible
        .iter()
        .map(|line| ListItem::new(line.clone()))
        .collect();

    let top = Paragraph::new(format!(
        "sim-{} [{}]  lines {}-{} / {}",
        inst.id,
        inst.sim_type.as_str(),
        start + 1,
        end,
        all_lines.len()
    ));
    f.render_widget(top, inner[0]);

    let list = List::new(items).block(Block::default().borders(Borders::ALL));
    f.render_widget(list, inner[1]);

    let help = Paragraph::new("j/k or up/down to scroll, Esc or q to close");
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
