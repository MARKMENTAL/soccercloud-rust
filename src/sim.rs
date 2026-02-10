use std::cmp::Ordering;
use std::collections::BTreeMap;

use crate::data::{display_name, profile_for, tactic_by_key, TeamProfile};
use crate::utils::{pad2, Rng};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationType {
    Single,
    League4,
    Knockout4,
}

impl SimulationType {
    pub fn as_str(self) -> &'static str {
        match self {
            SimulationType::Single => "single",
            SimulationType::League4 => "league4",
            SimulationType::Knockout4 => "knockout4",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TeamStats {
    pub shots: u16,
    pub sot: u16,
    pub xg: f64,
    pub corners: u16,
    pub fouls: u16,
    pub yellows: u16,
    pub offsides: u16,
    pub saves: u16,
    pub attacks: u16,
}

#[derive(Debug, Clone)]
pub struct MatchStats {
    pub home: TeamStats,
    pub away: TeamStats,
}

#[derive(Debug, Clone)]
pub struct MatchResult {
    pub home: String,
    pub away: String,
    pub home_goals: u8,
    pub away_goals: u8,
    pub home_profile: TeamProfile,
    pub away_profile: TeamProfile,
    pub stats: MatchStats,
    pub home_possession: u8,
    pub away_possession: u8,
}

#[derive(Debug, Clone)]
pub struct StandingsRow {
    pub team: String,
    pub p: u8,
    pub w: u8,
    pub d: u8,
    pub l: u8,
    pub gf: u16,
    pub ga: u16,
    pub gd: i16,
    pub pts: u8,
}

#[derive(Debug, Clone)]
pub struct SimFrame {
    pub scoreboard: String,
    pub logs: Vec<String>,
    pub stats_lines: Option<Vec<String>>,
    pub competition_lines: Option<Vec<String>>,
    pub history_append: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum SimOutcome {
    Single(MatchResult),
    League {
        champion: String,
        final_table: Vec<StandingsRow>,
    },
    Knockout {
        champion: String,
    },
}

#[derive(Debug, Clone)]
pub struct PreparedSimulation {
    pub frames: Vec<SimFrame>,
    pub outcome: SimOutcome,
    pub history_lines: Vec<String>,
}

fn chance(rng: &mut Rng, p: f64) -> bool {
    rng.chance(p)
}

fn possession_pct(result: &MatchResult) -> (u8, u8) {
    let home_poss_base = (result.stats.home.attacks as f64)
        * if result.home_profile.tactic == "possession" {
            1.15
        } else {
            1.0
        };
    let away_poss_base = (result.stats.away.attacks as f64)
        * if result.away_profile.tactic == "possession" {
            1.15
        } else {
            1.0
        };
    let home_poss = if (home_poss_base + away_poss_base) > 0.0 {
        ((home_poss_base / (home_poss_base + away_poss_base)) * 100.0).round() as u8
    } else {
        50
    };
    let away_poss = 100 - home_poss;
    (home_poss, away_poss)
}

fn empty_frame(scoreboard: String, logs: Vec<String>) -> SimFrame {
    SimFrame {
        scoreboard,
        logs,
        stats_lines: None,
        competition_lines: None,
        history_append: Vec::new(),
    }
}

pub fn penalties(rng: &mut Rng) -> (u8, u8, bool) {
    let mut h = 0u8;
    let mut a = 0u8;
    for _ in 0..5 {
        if chance(rng, 0.76) {
            h += 1;
        }
        if chance(rng, 0.76) {
            a += 1;
        }
    }
    let mut rounds = 0;
    while h == a && rounds < 20 {
        if chance(rng, 0.76) {
            h += 1;
        }
        if chance(rng, 0.76) {
            a += 1;
        }
        rounds += 1;
    }
    (h, a, h > a)
}

pub fn simulate_match(home: &str, away: &str, rng: &mut Rng) -> (MatchResult, Vec<SimFrame>) {
    let home_profile = profile_for(home);
    let away_profile = profile_for(away);

    let home_tactic = tactic_by_key(home_profile.tactic);
    let away_tactic = tactic_by_key(away_profile.tactic);

    let mut minute: u8 = 0;
    let mut home_goals: u8 = 0;
    let mut away_goals: u8 = 0;

    let mut stats = MatchStats {
        home: TeamStats::default(),
        away: TeamStats::default(),
    };

    let kickoff = format!(
        "Kickoff! {} ({}, {}) vs {} ({}, {})",
        display_name(home),
        home_profile.formation,
        home_tactic.label,
        display_name(away),
        away_profile.formation,
        away_tactic.label
    );

    let mut frames = vec![empty_frame(
        format!(
            "{} ({}) {} - {} {} ({}) | {}'",
            display_name(home),
            home_profile.formation,
            home_goals,
            away_goals,
            display_name(away),
            away_profile.formation,
            pad2(minute)
        ),
        vec![kickoff],
    )];

    while minute < 90 {
        minute += 1;
        let pressure_boost = if minute < 15 || minute > 75 { 1.2 } else { 1.0 };
        let mut logs: Vec<String> = Vec::new();

        let home_bias = home_tactic.attack_bias;
        let away_bias = away_tactic.attack_bias;
        let home_attacks = rng.next_f64() * (home_bias + away_bias) < home_bias;

        let (atk_team, def_team, atk_prof, def_prof, atk_stats, def_stats) = if home_attacks {
            (
                home,
                away,
                home_profile,
                away_profile,
                &mut stats.home,
                &mut stats.away,
            )
        } else {
            (
                away,
                home,
                away_profile,
                home_profile,
                &mut stats.away,
                &mut stats.home,
            )
        };

        let atk_tactic = tactic_by_key(atk_prof.tactic);
        let def_tactic = tactic_by_key(def_prof.tactic);

        if chance(rng, 0.24 * pressure_boost) {
            atk_stats.attacks += 1;
            let fast_break = chance(rng, atk_tactic.fast_break);
            if chance(rng, (if fast_break { 0.75 } else { 0.55 }) * pressure_boost) {
                atk_stats.shots += 1;
                let mut xg = if fast_break {
                    0.20 + rng.next_f64() * 0.25
                } else {
                    0.05 + rng.next_f64() * 0.22
                };
                xg *= atk_tactic.goal_mult;
                xg /= def_tactic.block_mult;

                let on_target = chance(rng, 0.52);
                if on_target {
                    atk_stats.sot += 1;
                }

                let is_goal = on_target && chance(rng, xg);
                if is_goal {
                    if home_attacks {
                        home_goals += 1;
                    } else {
                        away_goals += 1;
                    }
                    let finish = if fast_break {
                        "cut-back finish"
                    } else {
                        "drilled low"
                    };
                    logs.push(format!(
                        "{}' GOOOOAL - {} ({}, xG {:.2})",
                        pad2(minute),
                        display_name(atk_team),
                        finish,
                        xg
                    ));
                } else if on_target {
                    def_stats.saves += 1;
                    logs.push(format!(
                        "{}' Big save by {}'s keeper!",
                        pad2(minute),
                        display_name(def_team)
                    ));
                } else if chance(rng, 0.25) {
                    logs.push(format!(
                        "{}' {} fire it just wide.",
                        pad2(minute),
                        display_name(atk_team)
                    ));
                }
                atk_stats.xg += xg;
            }

            if chance(rng, 0.05 * atk_tactic.attack_bias) {
                atk_stats.corners += 1;
                logs.push(format!(
                    "{}' Corner to {}.",
                    pad2(minute),
                    display_name(atk_team)
                ));
            }

            if chance(rng, 0.035 + 0.02 * atk_tactic.fast_break) {
                atk_stats.offsides += 1;
                logs.push(format!(
                    "{}' Flag up - {} caught offside.",
                    pad2(minute),
                    display_name(atk_team)
                ));
            }
        }

        if chance(rng, 0.07 * atk_tactic.press_mult * atk_tactic.foul_mult) {
            def_stats.fouls += 1;
            if chance(rng, 0.22 * atk_tactic.press_mult) {
                def_stats.yellows += 1;
                logs.push(format!(
                    "{}' Yellow card to {}.",
                    pad2(minute),
                    display_name(def_team)
                ));
            }
        }

        if minute == 45 {
            logs.push(format!(
                "Halftime - {} {}-{} {}",
                display_name(home),
                home_goals,
                away_goals,
                display_name(away)
            ));
        }

        if minute == 90 {
            logs.push(format!(
                "Full time - {} {}-{} {}",
                display_name(home),
                home_goals,
                away_goals,
                display_name(away)
            ));
        }

        frames.push(empty_frame(
            format!(
                "{} ({}) {} - {} {} ({}) | {}'",
                display_name(home),
                home_profile.formation,
                home_goals,
                away_goals,
                display_name(away),
                away_profile.formation,
                pad2(minute)
            ),
            logs,
        ));
    }

    let preview_result = MatchResult {
        home: home.to_string(),
        away: away.to_string(),
        home_goals,
        away_goals,
        home_profile,
        away_profile,
        stats: stats.clone(),
        home_possession: 50,
        away_possession: 50,
    };
    let (home_poss, away_poss) = possession_pct(&preview_result);

    (
        MatchResult {
            home: home.to_string(),
            away: away.to_string(),
            home_goals,
            away_goals,
            home_profile,
            away_profile,
            stats,
            home_possession: home_poss,
            away_possession: away_poss,
        },
        frames,
    )
}

pub fn match_stats_lines(result: &MatchResult) -> Vec<String> {
    let home_tactic = tactic_by_key(result.home_profile.tactic);
    let away_tactic = tactic_by_key(result.away_profile.tactic);

    vec![
        format!(
            "Tactics: {} {} | {} {}",
            display_name(&result.home),
            home_tactic.label,
            display_name(&result.away),
            away_tactic.label
        ),
        format!(
            "Shots (On Target): {} ({}) vs {} ({})",
            result.stats.home.shots,
            result.stats.home.sot,
            result.stats.away.shots,
            result.stats.away.sot
        ),
        format!(
            "xG: {:.2} vs {:.2}",
            result.stats.home.xg, result.stats.away.xg
        ),
        format!(
            "Corners: {} vs {}",
            result.stats.home.corners, result.stats.away.corners
        ),
        format!(
            "Fouls (Yellows): {} ({}) vs {} ({})",
            result.stats.home.fouls,
            result.stats.home.yellows,
            result.stats.away.fouls,
            result.stats.away.yellows
        ),
        format!(
            "Offsides: {} vs {}",
            result.stats.home.offsides, result.stats.away.offsides
        ),
        format!(
            "Saves: {} vs {}",
            result.stats.home.saves, result.stats.away.saves
        ),
        format!(
            "Possession: {}% vs {}%",
            result.home_possession, result.away_possession
        ),
    ]
}

fn standings_cmp(a: &StandingsRow, b: &StandingsRow) -> Ordering {
    b.pts
        .cmp(&a.pts)
        .then(b.gd.cmp(&a.gd))
        .then(b.gf.cmp(&a.gf))
        .then(a.team.cmp(&b.team))
}

fn league_table_lines(rows: &[StandingsRow]) -> Vec<String> {
    let mut out = Vec::with_capacity(rows.len() + 2);
    out.push("TEAM                         P  W  D  L  GF GA GD PTS".to_string());
    out.push("--------------------------------------------------------".to_string());
    for r in rows {
        out.push(format!(
            "{:<28} {:>2} {:>2} {:>2} {:>2} {:>3} {:>2} {:>3} {:>3}",
            r.team, r.p, r.w, r.d, r.l, r.gf, r.ga, r.gd, r.pts
        ));
    }
    out
}

fn knockout_bracket_lines(
    semi1: Option<&str>,
    semi2: Option<&str>,
    final_line: Option<&str>,
    champion: Option<&str>,
) -> Vec<String> {
    vec![
        "Knockout Bracket".to_string(),
        format!("Semi 1: {}", semi1.unwrap_or("TBD")),
        "            \\".to_string(),
        format!("            +-- Final: {}", final_line.unwrap_or("TBD")),
        "            /".to_string(),
        format!("Semi 2: {}", semi2.unwrap_or("TBD")),
        format!("Champion: {}", champion.unwrap_or("TBD")),
    ]
}

fn init_table(teams: &[String]) -> BTreeMap<String, StandingsRow> {
    let mut map = BTreeMap::new();
    for team in teams {
        map.insert(
            team.clone(),
            StandingsRow {
                team: team.clone(),
                p: 0,
                w: 0,
                d: 0,
                l: 0,
                gf: 0,
                ga: 0,
                gd: 0,
                pts: 0,
            },
        );
    }
    map
}

pub fn run_single(teams: &[String], rng: &mut Rng) -> PreparedSimulation {
    let home = teams[0].clone();
    let away = teams[1].clone();
    let (result, frames) = simulate_match(&home, &away, rng);
    let stats_lines = match_stats_lines(&result);
    let mut frames = frames;
    frames.push(SimFrame {
        scoreboard: format!(
            "{} {}-{} {} | FT",
            display_name(&result.home),
            result.home_goals,
            result.away_goals,
            display_name(&result.away)
        ),
        logs: Vec::new(),
        stats_lines: Some(stats_lines.clone()),
        competition_lines: None,
        history_append: Vec::new(),
    });
    PreparedSimulation {
        frames,
        outcome: SimOutcome::Single(result),
        history_lines: vec![],
    }
}

pub fn run_league4(teams: &[String], rng: &mut Rng) -> PreparedSimulation {
    let fixtures = vec![
        (teams[0].clone(), teams[1].clone()),
        (teams[2].clone(), teams[3].clone()),
        (teams[0].clone(), teams[2].clone()),
        (teams[1].clone(), teams[3].clone()),
        (teams[0].clone(), teams[3].clone()),
        (teams[1].clone(), teams[2].clone()),
    ];

    let mut table = init_table(teams);
    let mut frames = Vec::new();
    let mut history = Vec::new();
    let mut last_stats = Vec::new();

    let mut initial_table: Vec<StandingsRow> = table.values().cloned().collect();
    initial_table.sort_by(standings_cmp);
    frames.push(SimFrame {
        scoreboard: "League created - waiting for Matchday 1".to_string(),
        logs: vec!["League table initialized".to_string()],
        stats_lines: None,
        competition_lines: Some(league_table_lines(&initial_table)),
        history_append: Vec::new(),
    });

    for (idx, (home, away)) in fixtures.iter().enumerate() {
        frames.push(empty_frame(
            format!("Running League Match {}/{}", idx + 1, fixtures.len()),
            vec![format!(
                "League fixture {}/{}: {} vs {}",
                idx + 1,
                fixtures.len(),
                display_name(home),
                display_name(away)
            )],
        ));

        let (res, mut match_frames) = simulate_match(home, away, rng);
        frames.append(&mut match_frames);
        last_stats = match_stats_lines(&res);

        {
            let home_row = table.get_mut(home).expect("home in table");
            home_row.p += 1;
            home_row.gf += res.home_goals as u16;
            home_row.ga += res.away_goals as u16;
            home_row.gd = home_row.gf as i16 - home_row.ga as i16;
            if res.home_goals > res.away_goals {
                home_row.w += 1;
                home_row.pts += 3;
            } else if res.home_goals == res.away_goals {
                home_row.d += 1;
                home_row.pts += 1;
            } else {
                home_row.l += 1;
            }
        }

        {
            let away_row = table.get_mut(away).expect("away in table");
            away_row.p += 1;
            away_row.gf += res.away_goals as u16;
            away_row.ga += res.home_goals as u16;
            away_row.gd = away_row.gf as i16 - away_row.ga as i16;
            if res.away_goals > res.home_goals {
                away_row.w += 1;
                away_row.pts += 3;
            } else if res.away_goals == res.home_goals {
                away_row.d += 1;
                away_row.pts += 1;
            } else {
                away_row.l += 1;
            }
        }

        history.push(format!(
            "{} {}-{} {}",
            display_name(home),
            res.home_goals,
            res.away_goals,
            display_name(away)
        ));

        let mut snapshot: Vec<StandingsRow> = table.values().cloned().collect();
        snapshot.sort_by(standings_cmp);
        frames.push(SimFrame {
            scoreboard: format!("League table updated after Match {}", idx + 1),
            logs: vec!["Standings updated".to_string()],
            stats_lines: Some(last_stats.clone()),
            competition_lines: Some(league_table_lines(&snapshot)),
            history_append: vec![history.last().cloned().unwrap_or_default()],
        });
    }

    let mut final_table: Vec<StandingsRow> = table.into_values().collect();
    final_table.sort_by(standings_cmp);
    let champion = final_table[0].team.clone();
    history.push(format!(
        "Champion: {} with {} pts",
        display_name(&champion),
        final_table[0].pts
    ));

    let competition = league_table_lines(&final_table);

    frames.push(SimFrame {
        scoreboard: format!("League complete - Champion {}", display_name(&champion)),
        logs: vec!["League finished".to_string()],
        stats_lines: Some(last_stats.clone()),
        competition_lines: Some(competition.clone()),
        history_append: vec![history.last().cloned().unwrap_or_default()],
    });

    PreparedSimulation {
        frames,
        outcome: SimOutcome::League {
            champion,
            final_table,
        },
        history_lines: history,
    }
}

pub fn run_knockout4(teams: &[String], rng: &mut Rng) -> PreparedSimulation {
    let semis = vec![
        (teams[0].clone(), teams[3].clone()),
        (teams[1].clone(), teams[2].clone()),
    ];
    let mut winners = Vec::new();
    let mut history = Vec::new();
    let mut frames = Vec::new();
    let mut semi1_line: Option<String> = None;
    let mut semi2_line: Option<String> = None;

    frames.push(SimFrame {
        scoreboard: "Knockout bracket initialized".to_string(),
        logs: vec!["Semi-finals ready".to_string()],
        stats_lines: None,
        competition_lines: Some(knockout_bracket_lines(None, None, None, None)),
        history_append: Vec::new(),
    });

    for (idx, (home, away)) in semis.iter().enumerate() {
        frames.push(empty_frame(
            format!("Running Semi-final {}/2", idx + 1),
            vec![format!(
                "Semi {}: {} vs {}",
                idx + 1,
                display_name(home),
                display_name(away)
            )],
        ));

        let (res, mut semi_frames) = simulate_match(home, away, rng);
        frames.append(&mut semi_frames);

        let line_text;

        let winner = if res.home_goals == res.away_goals {
            let (ph, pa, home_wins) = penalties(rng);
            line_text = format!(
                "Semi {}: {} {}-{} {} (pens {}-{})",
                idx + 1,
                display_name(home),
                res.home_goals,
                res.away_goals,
                display_name(away),
                ph,
                pa
            );
            history.push(line_text.clone());
            if home_wins {
                home.clone()
            } else {
                away.clone()
            }
        } else if res.home_goals > res.away_goals {
            line_text = format!(
                "Semi {}: {} {}-{} {}",
                idx + 1,
                display_name(home),
                res.home_goals,
                res.away_goals,
                display_name(away)
            );
            history.push(line_text.clone());
            home.clone()
        } else {
            line_text = format!(
                "Semi {}: {} {}-{} {}",
                idx + 1,
                display_name(home),
                res.home_goals,
                res.away_goals,
                display_name(away)
            );
            history.push(line_text.clone());
            away.clone()
        };

        if idx == 0 {
            semi1_line = Some(line_text.clone());
        } else {
            semi2_line = Some(line_text.clone());
        }

        frames.push(SimFrame {
            scoreboard: format!("Semi-final {} complete", idx + 1),
            logs: vec!["Bracket updated".to_string()],
            stats_lines: Some(match_stats_lines(&res)),
            competition_lines: Some(knockout_bracket_lines(
                semi1_line.as_deref(),
                semi2_line.as_deref(),
                None,
                None,
            )),
            history_append: vec![line_text],
        });
        winners.push(winner);
    }

    frames.push(empty_frame(
        "Running Final".to_string(),
        vec![format!(
            "Final: {} vs {}",
            display_name(&winners[0]),
            display_name(&winners[1])
        )],
    ));

    let (final_res, mut final_frames) = simulate_match(&winners[0], &winners[1], rng);
    frames.append(&mut final_frames);
    let last_stats = match_stats_lines(&final_res);

    let final_line;
    let champion = if final_res.home_goals == final_res.away_goals {
        let (ph, pa, home_wins) = penalties(rng);
        final_line = format!(
            "Final: {} {}-{} {} (pens {}-{})",
            display_name(&winners[0]),
            final_res.home_goals,
            final_res.away_goals,
            display_name(&winners[1]),
            ph,
            pa
        );
        history.push(final_line.clone());
        if home_wins {
            winners[0].clone()
        } else {
            winners[1].clone()
        }
    } else if final_res.home_goals > final_res.away_goals {
        final_line = format!(
            "Final: {} {}-{} {}",
            display_name(&winners[0]),
            final_res.home_goals,
            final_res.away_goals,
            display_name(&winners[1])
        );
        history.push(final_line.clone());
        winners[0].clone()
    } else {
        final_line = format!(
            "Final: {} {}-{} {}",
            display_name(&winners[0]),
            final_res.home_goals,
            final_res.away_goals,
            display_name(&winners[1])
        );
        history.push(final_line.clone());
        winners[1].clone()
    };

    let champion_line = format!("Champion: {} 🏆", display_name(&champion));
    history.push(champion_line.clone());

    frames.push(SimFrame {
        scoreboard: format!("Knockout complete - {}", display_name(&champion)),
        logs: vec!["Final complete".to_string()],
        stats_lines: Some(last_stats.clone()),
        competition_lines: Some(knockout_bracket_lines(
            semi1_line.as_deref(),
            semi2_line.as_deref(),
            Some(&final_line),
            Some(&champion_line),
        )),
        history_append: vec![final_line.clone(), champion_line.clone()],
    });

    PreparedSimulation {
        frames,
        outcome: SimOutcome::Knockout {
            champion: champion.clone(),
        },
        history_lines: history,
    }
}

pub fn run_simulation(
    sim_type: SimulationType,
    teams: &[String],
    rng: &mut Rng,
) -> PreparedSimulation {
    match sim_type {
        SimulationType::Single => run_single(teams, rng),
        SimulationType::League4 => run_league4(teams, rng),
        SimulationType::Knockout4 => run_knockout4(teams, rng),
    }
}
