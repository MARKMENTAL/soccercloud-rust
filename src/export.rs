use std::io::{self, Write};

use crate::sim::{PreparedSimulation, SimOutcome};
use crate::utils::csv_escape;

fn write_row<W: Write>(mut w: W, cols: &[String]) -> io::Result<()> {
    let mut first = true;
    for col in cols {
        if !first {
            w.write_all(b",")?;
        }
        first = false;
        w.write_all(csv_escape(col).as_bytes())?;
    }
    w.write_all(b"\n")
}

pub fn simulation_to_csv_bytes(sim: &PreparedSimulation) -> io::Result<Vec<u8>> {
    let mut out: Vec<u8> = Vec::new();

    match &sim.outcome {
        SimOutcome::Single(m) => {
            write_row(
                &mut out,
                &[
                    "Category".to_string(),
                    "Home Team".to_string(),
                    "Away Team".to_string(),
                ],
            )?;
            write_row(
                &mut out,
                &["Team".to_string(), m.home.clone(), m.away.clone()],
            )?;
            write_row(
                &mut out,
                &[
                    "Goals".to_string(),
                    m.home_goals.to_string(),
                    m.away_goals.to_string(),
                ],
            )?;
            write_row(
                &mut out,
                &[
                    "Shots".to_string(),
                    m.stats.home.shots.to_string(),
                    m.stats.away.shots.to_string(),
                ],
            )?;
            write_row(
                &mut out,
                &[
                    "Shots on Target".to_string(),
                    m.stats.home.sot.to_string(),
                    m.stats.away.sot.to_string(),
                ],
            )?;
            write_row(
                &mut out,
                &[
                    "xG".to_string(),
                    format!("{:.2}", m.stats.home.xg),
                    format!("{:.2}", m.stats.away.xg),
                ],
            )?;
            write_row(
                &mut out,
                &[
                    "Possession".to_string(),
                    format!("{}%", m.home_possession),
                    format!("{}%", m.away_possession),
                ],
            )?;
            write_row(
                &mut out,
                &[
                    "Corners".to_string(),
                    m.stats.home.corners.to_string(),
                    m.stats.away.corners.to_string(),
                ],
            )?;
            write_row(
                &mut out,
                &[
                    "Fouls".to_string(),
                    m.stats.home.fouls.to_string(),
                    m.stats.away.fouls.to_string(),
                ],
            )?;
            write_row(
                &mut out,
                &[
                    "Yellow Cards".to_string(),
                    m.stats.home.yellows.to_string(),
                    m.stats.away.yellows.to_string(),
                ],
            )?;
            write_row(
                &mut out,
                &[
                    "Saves".to_string(),
                    m.stats.home.saves.to_string(),
                    m.stats.away.saves.to_string(),
                ],
            )?;
        }
        SimOutcome::League { final_table, .. } => {
            write_row(
                &mut out,
                &[
                    "Team".to_string(),
                    "P".to_string(),
                    "W".to_string(),
                    "D".to_string(),
                    "L".to_string(),
                    "GF".to_string(),
                    "GA".to_string(),
                    "GD".to_string(),
                    "Pts".to_string(),
                ],
            )?;

            for row in final_table {
                write_row(
                    &mut out,
                    &[
                        row.team.clone(),
                        row.p.to_string(),
                        row.w.to_string(),
                        row.d.to_string(),
                        row.l.to_string(),
                        row.gf.to_string(),
                        row.ga.to_string(),
                        row.gd.to_string(),
                        row.pts.to_string(),
                    ],
                )?;
            }
        }
        SimOutcome::Knockout { .. } => {
            write_row(&mut out, &["Stage".to_string(), "Match Result".to_string()])?;
            for line in &sim.history_lines {
                let stage = if line.starts_with("Semi 1") {
                    "Semi 1"
                } else if line.starts_with("Semi 2") {
                    "Semi 2"
                } else if line.starts_with("Final") {
                    "Final"
                } else if line.starts_with("Champion") {
                    "Champion"
                } else {
                    "Info"
                };
                write_row(&mut out, &[stage.to_string(), line.clone()])?;
            }
        }
    }

    Ok(out)
}
