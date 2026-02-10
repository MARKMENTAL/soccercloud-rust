pub mod dashboard;
pub mod detail;
pub mod modal;
pub mod widgets;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::App;

pub fn draw(f: &mut Frame<'_>, app: &App) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(2),
        ])
        .split(f.area());

    let header = Paragraph::new("MentalNet SoccerCloud | n/l/o create | s start | c clone | d delete | e export | v detail | t stats | g standings | h history | q quit")
        .block(Block::default().title("Dashboard").borders(Borders::ALL))
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(header, areas[0]);

    if app.show_detail {
        detail::render(f, areas[1], app);
    } else {
        dashboard::render(f, areas[1], app);
    }

    let footer = Paragraph::new(format!(
        "{} | speed={} (1/2/4/0) | create modal: m=manual p=cpu [ ] team Enter=create Esc=cancel | view modal: j/k scroll Esc/q close",
        app.status_line,
        app.speed.label()
    ))
    .block(Block::default().borders(Borders::ALL).title("Status"))
    .style(Style::default().fg(Color::Green));
    f.render_widget(footer, areas[2]);

    if let Some(draft) = &app.create_draft {
        modal::render_create(f, f.area(), app, draft);
    }

    if let Some(kind) = app.overlay_modal {
        modal::render_overlay(f, f.area(), app, kind);
    }
}
