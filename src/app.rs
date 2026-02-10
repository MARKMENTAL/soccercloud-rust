use std::fs::File;
use std::io::{self, Write};
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::data::TEAMS;
use crate::instance::{SimStatus, SimulationInstance};
use crate::sim::SimulationType;
use crate::ui;
use crate::utils::{derive_seed, Rng};

const MAX_INSTANCES: usize = 100;

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum Speed {
    X1,
    X2,
    X4,
    Instant,
}

impl Speed {
    pub fn frames_per_tick(self) -> usize {
        match self {
            Speed::X1 => 1,
            Speed::X2 => 2,
            Speed::X4 => 4,
            Speed::Instant => 200,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Speed::X1 => "1x",
            Speed::X2 => "2x",
            Speed::X4 => "4x",
            Speed::Instant => "Instant",
        }
    }
}

pub struct App {
    pub base_seed: u64,
    pub speed: Speed,
    pub instances: Vec<SimulationInstance>,
    pub selected: usize,
    pub show_detail: bool,
    pub status_line: String,
    next_id: usize,
}

impl App {
    pub fn new(base_seed: u64, speed: Speed) -> Self {
        Self {
            base_seed,
            speed,
            instances: Vec::with_capacity(MAX_INSTANCES),
            selected: 0,
            show_detail: false,
            status_line: format!("Ready. Seed={base_seed}, Speed={}", speed.label()),
            next_id: 0,
        }
    }

    pub fn create_instance(&mut self, sim_type: SimulationType) {
        if self.instances.len() >= MAX_INSTANCES {
            self.status_line = format!("Instance limit reached ({MAX_INSTANCES})");
            return;
        }
        let id = self.next_id;
        let seed = derive_seed(self.base_seed, id as u64 + 1);
        let teams = self.random_teams_for_mode(sim_type, seed);
        let instance = SimulationInstance::new(id, sim_type, teams, seed);
        self.instances.push(instance);
        self.selected = self.instances.len().saturating_sub(1);
        self.next_id += 1;
        self.status_line = format!("Created sim-{id}");
    }

    pub fn start_selected(&mut self) {
        if let Some(inst) = self.instances.get_mut(self.selected) {
            inst.start();
            self.status_line = format!("Started sim-{}", inst.id);
        }
    }

    pub fn clone_selected(&mut self) {
        if self.instances.len() >= MAX_INSTANCES {
            self.status_line = format!("Instance limit reached ({MAX_INSTANCES})");
            return;
        }
        let Some(existing) = self.instances.get(self.selected).cloned() else {
            return;
        };

        let new_id = self.next_id;
        let new_seed = derive_seed(self.base_seed, new_id as u64 + 1);
        let cloned = existing.clone_as(new_id, new_seed);
        self.instances.push(cloned);
        self.selected = self.instances.len().saturating_sub(1);
        self.next_id += 1;
        self.status_line = format!("Cloned sim-{} -> sim-{}", existing.id, new_id);
    }

    pub fn delete_selected(&mut self) {
        if self.instances.is_empty() {
            return;
        }
        let removed = self.instances.remove(self.selected);
        if self.selected >= self.instances.len() {
            self.selected = self.instances.len().saturating_sub(1);
        }
        self.status_line = format!("Deleted sim-{}", removed.id);
    }

    pub fn export_selected(&mut self) {
        let Some(inst) = self.instances.get(self.selected) else {
            return;
        };

        match inst.export_csv() {
            Ok(bytes) => {
                let file_name = format!("sim-{}-{}.csv", inst.id, inst.sim_type.as_str());
                match File::create(&file_name).and_then(|mut f| f.write_all(&bytes)) {
                    Ok(_) => {
                        self.status_line = format!("Exported {}", file_name);
                    }
                    Err(e) => {
                        self.status_line = format!("Export failed: {e}");
                    }
                }
            }
            Err(e) => self.status_line = e,
        }
    }

    pub fn cycle_speed(&mut self, speed: Speed) {
        self.speed = speed;
        self.status_line = format!("Speed set to {}", self.speed.label());
    }

    fn random_teams_for_mode(&self, sim_type: SimulationType, seed: u64) -> Vec<String> {
        let count = match sim_type {
            SimulationType::Single => 2,
            SimulationType::League4 | SimulationType::Knockout4 => 4,
        };
        unique_random_teams(count, seed)
    }

    pub fn tick(&mut self) {
        let frames = self.speed.frames_per_tick();
        for inst in &mut self.instances {
            if matches!(inst.status, SimStatus::Running { .. }) {
                inst.tick(frames);
            }
        }
    }

    pub fn selected_instance(&self) -> Option<&SimulationInstance> {
        self.instances.get(self.selected)
    }

    pub fn select_prev(&mut self) {
        if self.instances.is_empty() {
            return;
        }
        if self.selected == 0 {
            self.selected = self.instances.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    pub fn select_next(&mut self) {
        if self.instances.is_empty() {
            return;
        }
        self.selected = (self.selected + 1) % self.instances.len();
    }
}

pub fn run_tui(mut app: App) -> io::Result<()> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut running = true;
    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    while running {
        terminal.draw(|f| ui::draw(f, &app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => running = false,
                        KeyCode::Up | KeyCode::Char('k') => app.select_prev(),
                        KeyCode::Down | KeyCode::Char('j') => app.select_next(),
                        KeyCode::Char('n') => app.create_instance(SimulationType::Single),
                        KeyCode::Char('l') => app.create_instance(SimulationType::League4),
                        KeyCode::Char('o') => app.create_instance(SimulationType::Knockout4),
                        KeyCode::Char('s') => app.start_selected(),
                        KeyCode::Char('c') => app.clone_selected(),
                        KeyCode::Char('d') => app.delete_selected(),
                        KeyCode::Enter | KeyCode::Char('v') => app.show_detail = !app.show_detail,
                        KeyCode::Char('e') => app.export_selected(),
                        KeyCode::Char('1') => app.cycle_speed(Speed::X1),
                        KeyCode::Char('2') => app.cycle_speed(Speed::X2),
                        KeyCode::Char('4') => app.cycle_speed(Speed::X4),
                        KeyCode::Char('0') => app.cycle_speed(Speed::Instant),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn unique_random_teams(count: usize, seed: u64) -> Vec<String> {
    let mut rng = Rng::new(seed);
    let mut pool: Vec<&str> = TEAMS.to_vec();
    let mut picked = Vec::with_capacity(count);
    while picked.len() < count && !pool.is_empty() {
        let i = rng.range_usize(pool.len());
        picked.push(pool.remove(i).to_string());
    }
    picked
}
