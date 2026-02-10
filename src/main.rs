mod app;
mod data;
mod export;
mod instance;
mod sim;
mod ui;
mod utils;

use std::fs::File;
use std::io::{self, Write};

use clap::{Parser, Subcommand, ValueEnum};

use app::{resolve_quick_single_teams, run_tui, App, Speed};
use data::{display_name, TEAMS};
use export::simulation_to_csv_bytes;
use sim::{run_simulation, SimulationType};
use utils::{derive_seed, Rng};

#[derive(Debug, Parser)]
#[command(name = "soccercloud")]
#[command(about = "MentalNet SoccerCloud - Rust CLI/TUI simulator")]
struct Cli {
    #[arg(long, global = true)]
    seed: Option<u64>,

    #[arg(long, global = true, value_enum, default_value_t = Speed::X1)]
    speed: Speed,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Quick {
        #[arg(long)]
        home: Option<String>,
        #[arg(long)]
        away: Option<String>,
    },
    List,
    Export {
        #[arg(long, value_enum)]
        mode: ModeArg,
        #[arg(long)]
        out: String,
        #[arg(long = "team", required = true)]
        teams: Vec<String>,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ModeArg {
    Single,
    League4,
    Knockout4,
}

impl From<ModeArg> for SimulationType {
    fn from(value: ModeArg) -> Self {
        match value {
            ModeArg::Single => SimulationType::Single,
            ModeArg::League4 => SimulationType::League4,
            ModeArg::Knockout4 => SimulationType::Knockout4,
        }
    }
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let base_seed = cli.seed.unwrap_or_else(|| Rng::from_time().next_u64());

    match cli.command {
        None => {
            let app = App::new(base_seed, cli.speed);
            run_tui(app)
        }
        Some(Commands::Quick { home, away }) => quick_mode(home, away, base_seed),
        Some(Commands::List) => {
            for team in TEAMS {
                println!("{}", display_name(team));
            }
            Ok(())
        }
        Some(Commands::Export { mode, out, teams }) => export_mode(mode, out, teams, base_seed),
    }
}

fn quick_mode(home: Option<String>, away: Option<String>, base_seed: u64) -> io::Result<()> {
    let teams =
        resolve_quick_single_teams(home.as_deref(), away.as_deref(), derive_seed(base_seed, 1))
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    let seed = derive_seed(base_seed, 1);
    let mut rng = Rng::new(seed);
    let prepared = run_simulation(SimulationType::Single, &teams, &mut rng);

    println!("seed={seed}");
    if let sim::SimOutcome::Single(m) = prepared.outcome {
        println!("{} {}-{} {}", m.home, m.home_goals, m.away_goals, m.away);
        println!("xG {:.2} - {:.2}", m.stats.home.xg, m.stats.away.xg);
    }

    println!("-- log --");
    for frame in prepared.frames {
        for line in frame.logs {
            println!("{}", line);
        }
    }

    Ok(())
}

fn export_mode(mode: ModeArg, out: String, teams: Vec<String>, base_seed: u64) -> io::Result<()> {
    let required = match mode {
        ModeArg::Single => 2,
        ModeArg::League4 | ModeArg::Knockout4 => 4,
    };
    if teams.len() != required {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "mode {:?} requires exactly {} --team values",
                mode, required
            ),
        ));
    }

    let mut rng = Rng::new(derive_seed(base_seed, 1));
    let prepared = run_simulation(mode.into(), &teams, &mut rng);
    let bytes = simulation_to_csv_bytes(&prepared)?;
    let mut f = File::create(&out)?;
    f.write_all(&bytes)?;
    println!("Wrote {}", out);
    Ok(())
}
