use std::collections::{HashSet, VecDeque};
use std::io;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use actix_web::http::header;
use actix_web::{web, App as ActixApp, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use crate::app::Speed;
use crate::data::{display_name, TEAMS};
use crate::instance::{SimStatus, SimulationInstance};
use crate::sim::SimulationType;
use crate::utils::{derive_seed, Rng};

const WEB_PORT: u16 = 9009;

#[derive(Clone)]
struct SharedState {
    inner: Arc<Mutex<WebState>>,
}

struct WebState {
    base_seed: u64,
    speed: Speed,
    next_id: usize,
    instances: Vec<SimulationInstance>,
}

impl WebState {
    fn new(base_seed: u64, speed: Speed) -> Self {
        Self {
            base_seed,
            speed,
            next_id: 0,
            instances: Vec::new(),
        }
    }

    fn next_seed(&self) -> u64 {
        derive_seed(self.base_seed, self.next_id as u64 + 1)
    }

    fn tick(&mut self) {
        let frames = self.speed.frames_per_tick();
        for inst in &mut self.instances {
            if matches!(inst.status, SimStatus::Running { .. }) {
                inst.tick(frames);
            }
        }
    }

    fn simulation_mut(&mut self, id: usize) -> Option<&mut SimulationInstance> {
        self.instances.iter_mut().find(|s| s.id == id)
    }

    fn simulation(&self, id: usize) -> Option<&SimulationInstance> {
        self.instances.iter().find(|s| s.id == id)
    }

    fn remove_simulation(&mut self, id: usize) -> bool {
        let before = self.instances.len();
        self.instances.retain(|s| s.id != id);
        self.instances.len() != before
    }
}

#[derive(Debug, Serialize)]
struct ErrorDto {
    error: String,
}

#[derive(Debug, Serialize)]
struct TeamDto {
    name: String,
    display_name: String,
}

#[derive(Debug, Serialize)]
struct SimulationSummaryDto {
    id: usize,
    mode: String,
    status: String,
    seed: u64,
    teams: Vec<String>,
    title: String,
    progress: String,
    scoreboard: String,
    outcome: String,
}

#[derive(Debug, Serialize)]
struct SimulationDetailDto {
    id: usize,
    mode: String,
    status: String,
    seed: u64,
    teams: Vec<String>,
    title: String,
    progress: String,
    scoreboard: String,
    outcome: String,
    logs: Vec<String>,
    stats_lines: Vec<String>,
    competition_lines: Vec<String>,
    history_lines: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct CreateSimulationRequest {
    mode: String,
    teams: Option<Vec<String>>,
    auto_fill: Option<bool>,
}

#[derive(Debug, Serialize)]
struct CreateSimulationResponse {
    id: usize,
}

fn sim_type_label(sim_type: SimulationType) -> &'static str {
    match sim_type {
        SimulationType::Single => "Single Match",
        SimulationType::League4 => "4-Team League",
        SimulationType::Knockout4 => "4-Team Knockout",
    }
}

fn mode_to_sim_type(mode: &str) -> Option<SimulationType> {
    match mode {
        "single" => Some(SimulationType::Single),
        "league4" => Some(SimulationType::League4),
        "knockout4" => Some(SimulationType::Knockout4),
        _ => None,
    }
}

fn required_team_count(sim_type: SimulationType) -> usize {
    match sim_type {
        SimulationType::Single => 2,
        SimulationType::League4 | SimulationType::Knockout4 => 4,
    }
}

fn status_label(status: &SimStatus) -> &'static str {
    match status {
        SimStatus::Pending => "pending",
        SimStatus::Running { .. } => "running",
        SimStatus::Completed => "completed",
    }
}

fn simulation_title(sim: &SimulationInstance) -> String {
    match sim.sim_type {
        SimulationType::Single => {
            if sim.teams.len() == 2 {
                format!("Match: {} vs {}", sim.teams[0], sim.teams[1])
            } else {
                "Match".to_string()
            }
        }
        _ => sim_type_label(sim.sim_type).to_string(),
    }
}

fn to_summary(sim: &SimulationInstance) -> SimulationSummaryDto {
    SimulationSummaryDto {
        id: sim.id,
        mode: sim.sim_type.as_str().to_string(),
        status: status_label(&sim.status).to_string(),
        seed: sim.seed,
        teams: sim.teams.clone(),
        title: simulation_title(sim),
        progress: sim.progress_text(),
        scoreboard: sim.scoreboard.clone(),
        outcome: sim.outcome_summary(),
    }
}

fn to_detail(sim: &SimulationInstance) -> SimulationDetailDto {
    SimulationDetailDto {
        id: sim.id,
        mode: sim.sim_type.as_str().to_string(),
        status: status_label(&sim.status).to_string(),
        seed: sim.seed,
        teams: sim.teams.clone(),
        title: simulation_title(sim),
        progress: sim.progress_text(),
        scoreboard: sim.scoreboard.clone(),
        outcome: sim.outcome_summary(),
        logs: vecdeque_to_vec(&sim.logs),
        stats_lines: sim.stats_lines.clone(),
        competition_lines: sim.competition_lines.clone(),
        history_lines: sim.history_lines.clone(),
    }
}

fn vecdeque_to_vec(items: &VecDeque<String>) -> Vec<String> {
    items.iter().cloned().collect()
}

fn resolve_teams(
    sim_type: SimulationType,
    provided_teams: Option<Vec<String>>,
    auto_fill: bool,
    seed: u64,
) -> Result<Vec<String>, String> {
    let required = required_team_count(sim_type);
    let mut selected = provided_teams.unwrap_or_default();
    let mut seen = HashSet::new();

    if selected.len() > required {
        return Err(format!(
            "mode {} accepts at most {} teams",
            sim_type.as_str(),
            required
        ));
    }

    for team in &selected {
        if !TEAMS.contains(&team.as_str()) {
            return Err(format!("Unknown team: {team}"));
        }
        if !seen.insert(team.clone()) {
            return Err(format!("Duplicate team: {team}"));
        }
    }

    if !auto_fill && selected.len() != required {
        return Err(format!(
            "mode {} requires exactly {} teams when auto_fill=false",
            sim_type.as_str(),
            required
        ));
    }

    if auto_fill {
        let mut pool: Vec<&str> = TEAMS
            .iter()
            .copied()
            .filter(|team| !seen.contains(*team))
            .collect();

        let mut rng = Rng::new(seed);
        while selected.len() < required {
            if pool.is_empty() {
                return Err("Not enough teams available for auto-fill".to_string());
            }
            let i = rng.range_usize(pool.len());
            selected.push(pool.remove(i).to_string());
        }
    }

    Ok(selected)
}

async fn index_html() -> impl Responder {
    HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, "text/html; charset=utf-8"))
        .body(include_str!("../index.html"))
}

async fn data_js() -> impl Responder {
    HttpResponse::Ok()
        .insert_header((
            header::CONTENT_TYPE,
            "application/javascript; charset=utf-8",
        ))
        .body(include_str!("../data.js"))
}

async fn sc_logo_jpg() -> impl Responder {
    HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, "image/jpeg"))
        .body(include_bytes!("../sc-logo.jpg").as_slice())
}

async fn api_teams() -> impl Responder {
    let items: Vec<TeamDto> = TEAMS
        .iter()
        .map(|team| TeamDto {
            name: (*team).to_string(),
            display_name: display_name(team),
        })
        .collect();
    HttpResponse::Ok().json(items)
}

async fn api_list_simulations(state: web::Data<SharedState>) -> impl Responder {
    let guard = match state.inner.lock() {
        Ok(g) => g,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorDto {
                error: "state lock poisoned".to_string(),
            })
        }
    };
    let mut sims = guard.instances.iter().map(to_summary).collect::<Vec<_>>();
    sims.sort_by_key(|s| s.id);
    HttpResponse::Ok().json(sims)
}

async fn api_get_simulation(
    path: web::Path<usize>,
    state: web::Data<SharedState>,
) -> impl Responder {
    let id = path.into_inner();
    let guard = match state.inner.lock() {
        Ok(g) => g,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorDto {
                error: "state lock poisoned".to_string(),
            })
        }
    };
    if let Some(sim) = guard.simulation(id) {
        return HttpResponse::Ok().json(to_detail(sim));
    }
    HttpResponse::NotFound().json(ErrorDto {
        error: format!("simulation {id} not found"),
    })
}

async fn api_create_simulation(
    payload: web::Json<CreateSimulationRequest>,
    state: web::Data<SharedState>,
) -> impl Responder {
    let mut guard = match state.inner.lock() {
        Ok(g) => g,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorDto {
                error: "state lock poisoned".to_string(),
            })
        }
    };

    let Some(sim_type) = mode_to_sim_type(payload.mode.as_str()) else {
        return HttpResponse::BadRequest().json(ErrorDto {
            error: format!("Unsupported mode: {}", payload.mode),
        });
    };

    let id = guard.next_id;
    let seed = guard.next_seed();
    let auto_fill = payload.auto_fill.unwrap_or(true);
    let teams = match resolve_teams(sim_type, payload.teams.clone(), auto_fill, seed) {
        Ok(v) => v,
        Err(e) => return HttpResponse::BadRequest().json(ErrorDto { error: e }),
    };

    let sim = SimulationInstance::new(id, sim_type, teams, seed);
    guard.instances.push(sim);
    guard.next_id += 1;

    HttpResponse::Created().json(CreateSimulationResponse { id })
}

async fn api_start_simulation(
    path: web::Path<usize>,
    state: web::Data<SharedState>,
) -> impl Responder {
    let id = path.into_inner();
    let mut guard = match state.inner.lock() {
        Ok(g) => g,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorDto {
                error: "state lock poisoned".to_string(),
            })
        }
    };

    if let Some(sim) = guard.simulation_mut(id) {
        sim.start();
        return HttpResponse::Ok().json(to_summary(sim));
    }

    HttpResponse::NotFound().json(ErrorDto {
        error: format!("simulation {id} not found"),
    })
}

async fn api_clone_simulation(
    path: web::Path<usize>,
    state: web::Data<SharedState>,
) -> impl Responder {
    let id = path.into_inner();
    let mut guard = match state.inner.lock() {
        Ok(g) => g,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorDto {
                error: "state lock poisoned".to_string(),
            })
        }
    };

    let Some(existing) = guard.simulation(id).cloned() else {
        return HttpResponse::NotFound().json(ErrorDto {
            error: format!("simulation {id} not found"),
        });
    };

    let new_id = guard.next_id;
    let new_seed = guard.next_seed();
    let clone = existing.clone_as(new_id, new_seed);
    guard.instances.push(clone);
    guard.next_id += 1;

    HttpResponse::Created().json(CreateSimulationResponse { id: new_id })
}

async fn api_delete_simulation(
    path: web::Path<usize>,
    state: web::Data<SharedState>,
) -> impl Responder {
    let id = path.into_inner();
    let mut guard = match state.inner.lock() {
        Ok(g) => g,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorDto {
                error: "state lock poisoned".to_string(),
            })
        }
    };

    if guard.remove_simulation(id) {
        return HttpResponse::NoContent().finish();
    }

    HttpResponse::NotFound().json(ErrorDto {
        error: format!("simulation {id} not found"),
    })
}

async fn api_export_csv(path: web::Path<usize>, state: web::Data<SharedState>) -> impl Responder {
    let id = path.into_inner();
    let guard = match state.inner.lock() {
        Ok(g) => g,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorDto {
                error: "state lock poisoned".to_string(),
            })
        }
    };

    let Some(sim) = guard.simulation(id) else {
        return HttpResponse::NotFound().json(ErrorDto {
            error: format!("simulation {id} not found"),
        });
    };

    let csv = match sim.export_csv() {
        Ok(bytes) => bytes,
        Err(e) => return HttpResponse::BadRequest().json(ErrorDto { error: e }),
    };

    let filename = format!("sim-{}-{}.csv", sim.id, sim.sim_type.as_str());
    HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, "text/csv; charset=utf-8"))
        .insert_header((
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", filename),
        ))
        .body(csv)
}

pub fn run_web_server(base_seed: u64, speed: Speed, listen_open: bool) -> io::Result<()> {
    let shared = SharedState {
        inner: Arc::new(Mutex::new(WebState::new(base_seed, speed))),
    };

    let ticker = shared.clone();
    let bind_host = if listen_open { "0.0.0.0" } else { "127.0.0.1" };
    let display_host = if listen_open {
        "<machine-ip>"
    } else {
        "127.0.0.1"
    };

    println!(
        "Starting SoccerCloud web UI at http://{display_host}:{WEB_PORT} (bound {bind_host}:{WEB_PORT}, seed={base_seed}, speed={})",
        speed.label(),
    );
    if listen_open {
        println!(
            "Open listen enabled: accessible from other machines on your network at http://<machine-ip>:{WEB_PORT}"
        );
    }

    actix_web::rt::System::new().block_on(async move {
        actix_web::rt::spawn(async move {
            let mut interval = actix_web::rt::time::interval(Duration::from_millis(60));
            loop {
                interval.tick().await;
                if let Ok(mut guard) = ticker.inner.lock() {
                    guard.tick();
                }
            }
        });

        HttpServer::new(move || {
            ActixApp::new()
                .app_data(web::Data::new(shared.clone()))
                .route("/", web::get().to(index_html))
                .route("/index.html", web::get().to(index_html))
                .route("/data.js", web::get().to(data_js))
                .route("/sc-logo.jpg", web::get().to(sc_logo_jpg))
                .service(
                    web::scope("/api")
                        .route("/teams", web::get().to(api_teams))
                        .route("/simulations", web::get().to(api_list_simulations))
                        .route("/simulations", web::post().to(api_create_simulation))
                        .route("/simulations/{id}", web::get().to(api_get_simulation))
                        .route("/simulations/{id}", web::delete().to(api_delete_simulation))
                        .route(
                            "/simulations/{id}/start",
                            web::post().to(api_start_simulation),
                        )
                        .route(
                            "/simulations/{id}/clone",
                            web::post().to(api_clone_simulation),
                        )
                        .route(
                            "/simulations/{id}/export.csv",
                            web::get().to(api_export_csv),
                        ),
                )
        })
        .bind((bind_host, WEB_PORT))?
        .run()
        .await
    })
}
