use std::collections::VecDeque;

use crate::export::simulation_to_csv_bytes;
use crate::sim::{run_simulation, PreparedSimulation, SimOutcome, SimulationType};
use crate::utils::Rng;

pub const MAX_LOG_LINES: usize = 1000;

#[derive(Debug, Clone)]
pub enum SimStatus {
    Pending,
    Running {
        frame_index: usize,
        total_frames: usize,
    },
    Completed,
}

#[derive(Debug, Clone)]
pub struct SimulationInstance {
    pub id: usize,
    pub sim_type: SimulationType,
    pub teams: Vec<String>,
    pub seed: u64,
    pub status: SimStatus,
    pub scoreboard: String,
    pub logs: VecDeque<String>,
    pub stats_lines: Vec<String>,
    pub competition_lines: Vec<String>,
    pub history_lines: Vec<String>,
    prepared: Option<PreparedSimulation>,
}

impl SimulationInstance {
    pub fn new(id: usize, sim_type: SimulationType, teams: Vec<String>, seed: u64) -> Self {
        Self {
            id,
            sim_type,
            teams,
            seed,
            status: SimStatus::Pending,
            scoreboard: "Waiting for kickoff...".to_string(),
            logs: VecDeque::with_capacity(MAX_LOG_LINES),
            stats_lines: Vec::new(),
            competition_lines: Vec::new(),
            history_lines: Vec::new(),
            prepared: None,
        }
    }

    pub fn start(&mut self) {
        if !matches!(self.status, SimStatus::Pending) {
            return;
        }

        let mut rng = Rng::new(self.seed);
        let prepared = run_simulation(self.sim_type, &self.teams, &mut rng);
        let total_frames = prepared.frames.len();
        self.stats_lines.clear();
        self.competition_lines.clear();
        self.history_lines.clear();
        self.status = SimStatus::Running {
            frame_index: 0,
            total_frames,
        };
        self.prepared = Some(prepared);
        self.push_log(format!(
            "Instance sim-{} started (seed={})",
            self.id, self.seed
        ));
    }

    pub fn tick(&mut self, frames_to_advance: usize) {
        if self.prepared.is_none() {
            return;
        }

        let total_available = self.prepared.as_ref().map(|p| p.frames.len()).unwrap_or(0);

        let SimStatus::Running {
            mut frame_index,
            total_frames,
        } = self.status.clone()
        else {
            return;
        };

        for _ in 0..frames_to_advance {
            if frame_index >= total_available {
                self.status = SimStatus::Completed;
                self.push_log("Simulation completed.".to_string());
                return;
            }

            let frame = self
                .prepared
                .as_ref()
                .and_then(|p| p.frames.get(frame_index).cloned())
                .expect("frame exists while running");
            self.scoreboard = frame.scoreboard;
            for line in frame.logs {
                self.push_log(line);
            }
            if let Some(stats) = frame.stats_lines {
                self.stats_lines = stats;
            }
            if let Some(comp) = frame.competition_lines {
                self.competition_lines = comp;
            }
            for item in frame.history_append {
                self.history_lines.push(item);
            }
            frame_index += 1;
        }

        if frame_index >= total_frames {
            self.status = SimStatus::Completed;
            self.push_log("Simulation completed.".to_string());
        } else {
            self.status = SimStatus::Running {
                frame_index,
                total_frames,
            };
        }
    }

    pub fn clone_as(&self, new_id: usize, new_seed: u64) -> Self {
        Self::new(new_id, self.sim_type, self.teams.clone(), new_seed)
    }

    pub fn progress_text(&self) -> String {
        match &self.status {
            SimStatus::Pending => "Ready to start".to_string(),
            SimStatus::Running {
                frame_index,
                total_frames,
            } => format!("Running {}/{}", frame_index, total_frames),
            SimStatus::Completed => "Completed".to_string(),
        }
    }

    pub fn export_csv(&self) -> Result<Vec<u8>, String> {
        let Some(prepared) = &self.prepared else {
            return Err("Simulation has not run yet".to_string());
        };
        simulation_to_csv_bytes(prepared).map_err(|e| format!("CSV export failed: {e}"))
    }

    pub fn outcome_summary(&self) -> String {
        let Some(prepared) = &self.prepared else {
            return "No result yet".to_string();
        };

        match &prepared.outcome {
            SimOutcome::Single(m) => {
                format!("{} {}-{} {}", m.home, m.home_goals, m.away_goals, m.away)
            }
            SimOutcome::League { champion, .. } => format!("Champion: {}", champion),
            SimOutcome::Knockout { champion } => format!("Champion: {}", champion),
        }
    }

    fn push_log(&mut self, line: String) {
        if self.logs.len() == MAX_LOG_LINES {
            self.logs.pop_front();
        }
        self.logs.push_back(line);
    }
}
