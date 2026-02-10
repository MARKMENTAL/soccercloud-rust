use std::collections::HashSet;
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
    pub create_draft: Option<CreateDraft>,
    pub status_line: String,
    next_id: usize,
}

#[derive(Debug, Clone)]
pub struct TeamSlotDraft {
    pub is_cpu: bool,
    pub team_idx: usize,
}

#[derive(Debug, Clone)]
pub struct CreateDraft {
    pub mode: SimulationType,
    pub slots: Vec<TeamSlotDraft>,
    pub selected_slot: usize,
}

impl CreateDraft {
    pub fn new(mode: SimulationType) -> Self {
        let count = slot_count_for_mode(mode);
        let mut slots = Vec::with_capacity(count);
        for i in 0..count {
            slots.push(TeamSlotDraft {
                is_cpu: true,
                team_idx: i % TEAMS.len(),
            });
        }
        Self {
            mode,
            slots,
            selected_slot: 0,
        }
    }

    pub fn mode_label(&self) -> &'static str {
        match self.mode {
            SimulationType::Single => "Single Match",
            SimulationType::League4 => "4-Team League",
            SimulationType::Knockout4 => "4-Team Knockout",
        }
    }
}

impl App {
    pub fn new(base_seed: u64, speed: Speed) -> Self {
        Self {
            base_seed,
            speed,
            instances: Vec::with_capacity(MAX_INSTANCES),
            selected: 0,
            show_detail: false,
            create_draft: None,
            status_line: format!("Ready. Seed={base_seed}, Speed={}", speed.label()),
            next_id: 0,
        }
    }

    pub fn open_create_draft(&mut self, sim_type: SimulationType) {
        self.create_draft = Some(CreateDraft::new(sim_type));
        self.status_line = format!("Create modal: {}", sim_type.as_str());
    }

    pub fn cancel_create_draft(&mut self) {
        self.create_draft = None;
        self.status_line = "Create canceled".to_string();
    }

    pub fn next_instance_seed_preview(&self) -> u64 {
        derive_seed(self.base_seed, self.next_id as u64 + 1)
    }

    pub fn draft_select_prev_slot(&mut self) {
        let Some(draft) = self.create_draft.as_mut() else {
            return;
        };
        if draft.selected_slot == 0 {
            draft.selected_slot = draft.slots.len().saturating_sub(1);
        } else {
            draft.selected_slot -= 1;
        }
    }

    pub fn draft_select_next_slot(&mut self) {
        let Some(draft) = self.create_draft.as_mut() else {
            return;
        };
        if draft.slots.is_empty() {
            return;
        }
        draft.selected_slot = (draft.selected_slot + 1) % draft.slots.len();
    }

    pub fn draft_set_selected_manual(&mut self) {
        let Some(draft) = self.create_draft.as_mut() else {
            return;
        };
        if let Some(slot) = draft.slots.get_mut(draft.selected_slot) {
            slot.is_cpu = false;
        }
    }

    pub fn draft_set_selected_cpu(&mut self) {
        let Some(draft) = self.create_draft.as_mut() else {
            return;
        };
        if let Some(slot) = draft.slots.get_mut(draft.selected_slot) {
            slot.is_cpu = true;
        }
    }

    pub fn draft_cycle_selected_team(&mut self, delta: i32) {
        let Some(draft) = self.create_draft.as_mut() else {
            return;
        };
        let Some(slot) = draft.slots.get_mut(draft.selected_slot) else {
            return;
        };
        if slot.is_cpu {
            return;
        }

        let len = TEAMS.len() as i32;
        let mut idx = slot.team_idx as i32 + delta;
        if idx < 0 {
            idx = len - 1;
        } else if idx >= len {
            idx = 0;
        }
        slot.team_idx = idx as usize;
    }

    pub fn confirm_create_draft(&mut self) {
        if self.instances.len() >= MAX_INSTANCES {
            self.status_line = format!("Instance limit reached ({MAX_INSTANCES})");
            return;
        }

        let Some(draft) = self.create_draft.clone() else {
            return;
        };

        let id = self.next_id;
        let seed = derive_seed(self.base_seed, id as u64 + 1);

        let teams = match resolve_teams_from_slots(&draft.slots, seed) {
            Ok(v) => v,
            Err(e) => {
                self.status_line = e;
                return;
            }
        };

        let instance = SimulationInstance::new(id, draft.mode, teams, seed);
        self.instances.push(instance);
        self.selected = self.instances.len().saturating_sub(1);
        self.next_id += 1;
        self.status_line = format!("Created sim-{id}");
        self.create_draft = None;
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
                    if app.create_draft.is_some() {
                        match key.code {
                            KeyCode::Esc => app.cancel_create_draft(),
                            KeyCode::Enter => app.confirm_create_draft(),
                            KeyCode::Up | KeyCode::Char('k') | KeyCode::Tab => {
                                app.draft_select_prev_slot()
                            }
                            KeyCode::Down | KeyCode::Char('j') => app.draft_select_next_slot(),
                            KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('[') => {
                                app.draft_cycle_selected_team(-1)
                            }
                            KeyCode::Right | KeyCode::Char('l') | KeyCode::Char(']') => {
                                app.draft_cycle_selected_team(1)
                            }
                            KeyCode::Char('m') => app.draft_set_selected_manual(),
                            KeyCode::Char('p') => app.draft_set_selected_cpu(),
                            _ => {}
                        }
                        continue;
                    }

                    match key.code {
                        KeyCode::Char('q') => running = false,
                        KeyCode::Up | KeyCode::Char('k') => app.select_prev(),
                        KeyCode::Down | KeyCode::Char('j') => app.select_next(),
                        KeyCode::Char('n') => app.open_create_draft(SimulationType::Single),
                        KeyCode::Char('l') => app.open_create_draft(SimulationType::League4),
                        KeyCode::Char('o') => app.open_create_draft(SimulationType::Knockout4),
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

fn slot_count_for_mode(sim_type: SimulationType) -> usize {
    match sim_type {
        SimulationType::Single => 2,
        SimulationType::League4 | SimulationType::Knockout4 => 4,
    }
}

fn resolve_teams_from_slots(slots: &[TeamSlotDraft], seed: u64) -> Result<Vec<String>, String> {
    let mut seen = HashSet::new();
    let mut cpu_count = 0usize;

    for slot in slots {
        if slot.is_cpu {
            cpu_count += 1;
            continue;
        }
        let team = TEAMS
            .get(slot.team_idx)
            .ok_or_else(|| "Manual team selection is out of range".to_string())?;
        if !seen.insert(*team) {
            return Err(format!("Duplicate manual team: {team}"));
        }
    }

    let mut remaining: Vec<&str> = TEAMS
        .iter()
        .copied()
        .filter(|team| !seen.contains(team))
        .collect();

    if remaining.len() < cpu_count {
        return Err("Not enough teams left for CPU auto-fill".to_string());
    }

    let mut rng = Rng::new(seed);
    let mut output: Vec<String> = Vec::with_capacity(slots.len());

    for slot in slots {
        if slot.is_cpu {
            let i = rng.range_usize(remaining.len());
            output.push(remaining.remove(i).to_string());
        } else {
            let team = TEAMS
                .get(slot.team_idx)
                .ok_or_else(|| "Manual team selection is out of range".to_string())?;
            output.push((*team).to_string());
        }
    }

    Ok(output)
}

pub fn resolve_quick_single_teams(
    home: Option<&str>,
    away: Option<&str>,
    selection_seed: u64,
) -> Result<Vec<String>, String> {
    let mut slots = vec![
        TeamSlotDraft {
            is_cpu: home.is_none(),
            team_idx: 0,
        },
        TeamSlotDraft {
            is_cpu: away.is_none(),
            team_idx: 1,
        },
    ];

    if let Some(home_team) = home {
        let idx = TEAMS
            .iter()
            .position(|t| *t == home_team)
            .ok_or_else(|| format!("Unknown home team: {home_team}"))?;
        slots[0].team_idx = idx;
    }
    if let Some(away_team) = away {
        let idx = TEAMS
            .iter()
            .position(|t| *t == away_team)
            .ok_or_else(|| format!("Unknown away team: {away_team}"))?;
        slots[1].team_idx = idx;
    }

    resolve_teams_from_slots(&slots, selection_seed)
}
