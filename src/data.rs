#[derive(Debug, Clone, Copy)]
pub struct Tactic {
    pub key: &'static str,
    pub label: &'static str,
    pub attack_bias: f64,
    pub goal_mult: f64,
    pub fast_break: f64,
    pub foul_mult: f64,
    pub block_mult: f64,
    pub press_mult: f64,
}

pub const TACTICS: [Tactic; 4] = [
    Tactic {
        key: "counter",
        label: "Counter",
        attack_bias: 1.10,
        goal_mult: 1.08,
        fast_break: 0.25,
        foul_mult: 1.00,
        block_mult: 1.00,
        press_mult: 0.95,
    },
    Tactic {
        key: "possession",
        label: "Possession",
        attack_bias: 1.00,
        goal_mult: 0.95,
        fast_break: 0.10,
        foul_mult: 0.90,
        block_mult: 1.00,
        press_mult: 0.90,
    },
    Tactic {
        key: "high_press",
        label: "High Press",
        attack_bias: 1.15,
        goal_mult: 1.00,
        fast_break: 0.20,
        foul_mult: 1.20,
        block_mult: 0.95,
        press_mult: 1.20,
    },
    Tactic {
        key: "low_block",
        label: "Low Block",
        attack_bias: 0.92,
        goal_mult: 0.92,
        fast_break: 0.12,
        foul_mult: 0.95,
        block_mult: 1.15,
        press_mult: 0.85,
    },
];

#[derive(Debug, Clone, Copy)]
pub struct Team {
    pub name: &'static str,
    pub flag: &'static str,
    pub formation: &'static str,
    pub tactic: &'static str,
}

pub const TEAMS_DATA: [Team; 85] = [
    // J-League Clubs
    Team {
        name: "Kashima Antlers",
        flag: "🇯🇵",
        formation: "4-4-2",
        tactic: "counter",
    },
    Team {
        name: "Urawa Red Diamonds",
        flag: "🇯🇵",
        formation: "4-2-3-1",
        tactic: "possession",
    },
    Team {
        name: "Gamba Osaka",
        flag: "🇯🇵",
        formation: "4-4-2",
        tactic: "counter",
    },
    Team {
        name: "Cerezo Osaka",
        flag: "🇯🇵",
        formation: "4-4-2",
        tactic: "counter",
    },
    Team {
        name: "Kawasaki Frontale",
        flag: "🇯🇵",
        formation: "4-3-3",
        tactic: "possession",
    },
    Team {
        name: "Yokohama F. Marinos",
        flag: "🇯🇵",
        formation: "4-3-3",
        tactic: "high_press",
    },
    Team {
        name: "Nagoya Grampus",
        flag: "🇯🇵",
        formation: "4-2-3-1",
        tactic: "low_block",
    },
    Team {
        name: "Shimizu S-Pulse",
        flag: "🇯🇵",
        formation: "4-4-2",
        tactic: "counter",
    },
    Team {
        name: "Sanfrecce Hiroshima",
        flag: "🇯🇵",
        formation: "3-5-2",
        tactic: "possession",
    },
    Team {
        name: "Consadole Sapporo",
        flag: "🇯🇵",
        formation: "3-5-2",
        tactic: "high_press",
    },
    Team {
        name: "Ventforet Kofu",
        flag: "🇯🇵",
        formation: "4-4-2",
        tactic: "counter",
    },
    Team {
        name: "Tokyo Verdy",
        flag: "🇯🇵",
        formation: "4-3-3",
        tactic: "possession",
    },
    Team {
        name: "JEF United Chiba",
        flag: "🇯🇵",
        formation: "4-3-3",
        tactic: "counter",
    },
    // European Clubs
    Team {
        name: "Arsenal",
        flag: "🇬🇧",
        formation: "4-3-3",
        tactic: "possession",
    },
    Team {
        name: "FC Barcelona",
        flag: "🇪🇸",
        formation: "4-3-3",
        tactic: "possession",
    },
    Team {
        name: "Real Madrid",
        flag: "🇪🇸",
        formation: "4-3-3",
        tactic: "counter",
    },
    Team {
        name: "Manchester City",
        flag: "🇬🇧",
        formation: "4-3-3",
        tactic: "possession",
    },
    Team {
        name: "Manchester United",
        flag: "🇬🇧",
        formation: "4-2-3-1",
        tactic: "high_press",
    },
    Team {
        name: "Liverpool",
        flag: "🇬🇧",
        formation: "4-3-3",
        tactic: "high_press",
    },
    Team {
        name: "Bayern Munich",
        flag: "🇩🇪",
        formation: "4-2-3-1",
        tactic: "high_press",
    },
    Team {
        name: "Borussia Dortmund",
        flag: "🇩🇪",
        formation: "4-2-3-1",
        tactic: "high_press",
    },
    Team {
        name: "Paris Saint-Germain",
        flag: "🇫🇷",
        formation: "4-3-3",
        tactic: "possession",
    },
    Team {
        name: "Juventus",
        flag: "🇮🇹",
        formation: "3-5-2",
        tactic: "low_block",
    },
    Team {
        name: "Inter",
        flag: "🇮🇹",
        formation: "3-5-2",
        tactic: "low_block",
    },
    Team {
        name: "AC Milan",
        flag: "🇮🇹",
        formation: "4-2-3-1",
        tactic: "possession",
    },
    Team {
        name: "Ajax",
        flag: "🇳🇱",
        formation: "4-3-3",
        tactic: "possession",
    },
    Team {
        name: "Benfica",
        flag: "🇵🇹",
        formation: "4-2-3-1",
        tactic: "possession",
    },
    Team {
        name: "Porto",
        flag: "🇵🇹",
        formation: "4-4-2",
        tactic: "counter",
    },
    Team {
        name: "Celtic",
        flag: "🇬🇧",
        formation: "4-3-3",
        tactic: "possession",
    },
    // UEFA National Teams
    Team {
        name: "England",
        flag: "🇬🇧",
        formation: "4-2-3-1",
        tactic: "high_press",
    },
    Team {
        name: "France",
        flag: "🇫🇷",
        formation: "4-2-3-1",
        tactic: "high_press",
    },
    Team {
        name: "Spain",
        flag: "🇪🇸",
        formation: "4-3-3",
        tactic: "possession",
    },
    Team {
        name: "Germany",
        flag: "🇩🇪",
        formation: "4-2-3-1",
        tactic: "high_press",
    },
    Team {
        name: "Italy",
        flag: "🇮🇹",
        formation: "4-4-2",
        tactic: "counter",
    },
    Team {
        name: "Portugal",
        flag: "🇵🇹",
        formation: "4-3-3",
        tactic: "possession",
    },
    Team {
        name: "Netherlands",
        flag: "🇳🇱",
        formation: "4-3-3",
        tactic: "possession",
    },
    Team {
        name: "Belgium",
        flag: "🇧🇪",
        formation: "4-2-3-1",
        tactic: "high_press",
    },
    Team {
        name: "Croatia",
        flag: "🇭🇷",
        formation: "4-4-2",
        tactic: "counter",
    },
    Team {
        name: "Denmark",
        flag: "🇩🇰",
        formation: "4-4-2",
        tactic: "counter",
    },
    Team {
        name: "Switzerland",
        flag: "🇨🇭",
        formation: "4-4-2",
        tactic: "counter",
    },
    Team {
        name: "Austria",
        flag: "🇦🇹",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    Team {
        name: "Sweden",
        flag: "🇸🇪",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    Team {
        name: "Norway",
        flag: "🇳🇴",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    Team {
        name: "Poland",
        flag: "🇵🇱",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    Team {
        name: "Serbia",
        flag: "🇷🇸",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    Team {
        name: "Turkey",
        flag: "🇹🇷",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    Team {
        name: "Ukraine",
        flag: "🇺🇦",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    Team {
        name: "Czech Republic",
        flag: "🇨🇿",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    Team {
        name: "Scotland",
        flag: "🇬🇧",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    // CONMEBOL National Teams
    Team {
        name: "Argentina",
        flag: "🇦🇷",
        formation: "4-2-3-1",
        tactic: "high_press",
    },
    Team {
        name: "Brazil",
        flag: "🇧🇷",
        formation: "4-2-3-1",
        tactic: "high_press",
    },
    Team {
        name: "Uruguay",
        flag: "🇺🇾",
        formation: "4-4-2",
        tactic: "counter",
    },
    Team {
        name: "Colombia",
        flag: "🇨🇴",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    Team {
        name: "Chile",
        flag: "🇨🇱",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    Team {
        name: "Peru",
        flag: "🇵🇪",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    Team {
        name: "Ecuador",
        flag: "🇪🇨",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    Team {
        name: "Paraguay",
        flag: "🇵🇾",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    Team {
        name: "Bolivia",
        flag: "🇧🇴",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    Team {
        name: "Venezuela",
        flag: "🇻🇪",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    // CONCACAF National Teams
    Team {
        name: "United States",
        flag: "🇺🇸",
        formation: "4-2-3-1",
        tactic: "high_press",
    },
    Team {
        name: "Mexico",
        flag: "🇲🇽",
        formation: "4-4-2",
        tactic: "counter",
    },
    Team {
        name: "Canada",
        flag: "🇨🇦",
        formation: "4-4-2",
        tactic: "counter",
    },
    Team {
        name: "Costa Rica",
        flag: "🇨🇷",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    Team {
        name: "Panama",
        flag: "🇵🇦",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    Team {
        name: "Jamaica",
        flag: "🇯🇲",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    Team {
        name: "Honduras",
        flag: "🇭🇳",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    // AFC/OFC National Teams
    Team {
        name: "Japan",
        flag: "🇯🇵",
        formation: "4-3-3",
        tactic: "possession",
    },
    Team {
        name: "South Korea",
        flag: "🇰🇷",
        formation: "4-2-3-1",
        tactic: "high_press",
    },
    Team {
        name: "Australia",
        flag: "🇦🇺",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    Team {
        name: "Iran",
        flag: "🇮🇷",
        formation: "4-4-2",
        tactic: "counter",
    },
    Team {
        name: "Saudi Arabia",
        flag: "🇸🇦",
        formation: "4-4-2",
        tactic: "counter",
    },
    Team {
        name: "Qatar",
        flag: "🇶🇦",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    Team {
        name: "Iraq",
        flag: "🇮🇶",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    Team {
        name: "United Arab Emirates",
        flag: "🇦🇪",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    Team {
        name: "PRC China",
        flag: "🇨🇳",
        formation: "4-3-3",
        tactic: "possession",
    },
    // CAF National Teams
    Team {
        name: "Morocco",
        flag: "🇲🇦",
        formation: "4-2-3-1",
        tactic: "high_press",
    },
    Team {
        name: "Senegal",
        flag: "🇸🇳",
        formation: "4-4-2",
        tactic: "counter",
    },
    Team {
        name: "Nigeria",
        flag: "🇳🇬",
        formation: "4-2-3-1",
        tactic: "high_press",
    },
    Team {
        name: "Egypt",
        flag: "🇪🇬",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    Team {
        name: "Algeria",
        flag: "🇩🇿",
        formation: "4-4-2",
        tactic: "counter",
    },
    Team {
        name: "Tunisia",
        flag: "🇹🇳",
        formation: "4-4-2",
        tactic: "counter",
    },
    Team {
        name: "Ghana",
        flag: "🇬🇭",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    Team {
        name: "Cameroon",
        flag: "🇨🇲",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    Team {
        name: "Ivory Coast",
        flag: "🇨🇮",
        formation: "4-2-3-1",
        tactic: "counter",
    },
    Team {
        name: "South Africa",
        flag: "🇿🇦",
        formation: "4-2-3-1",
        tactic: "counter",
    },
];

/// Generate team names array dynamically from TEAMS_DATA at compile time
const fn extract_team_names<const N: usize>(data: &[Team; N]) -> [&str; N] {
    let mut result = [""; N];
    let mut i = 0;
    while i < N {
        result[i] = data[i].name;
        i += 1;
    }
    result
}

/// Team names array automatically derived from TEAMS_DATA
pub const TEAMS: [&str; TEAMS_DATA.len()] = extract_team_names(&TEAMS_DATA);

pub fn team_by_name(name: &str) -> Option<&'static Team> {
    TEAMS_DATA.iter().find(|t| t.name == name)
}

pub fn team_flag(team: &str) -> &'static str {
    team_by_name(team).map(|t| t.flag).unwrap_or("🏳️")
}

pub fn display_name(team: &str) -> String {
    format!("{} {}", team_flag(team), team)
}

#[derive(Debug, Clone, Copy)]
pub struct TeamProfile {
    pub formation: &'static str,
    pub tactic: &'static str,
}

pub fn tactic_by_key(key: &str) -> Tactic {
    TACTICS
        .iter()
        .copied()
        .find(|t| t.key == key)
        .unwrap_or(TACTICS[0])
}

pub fn profile_for(team: &str) -> TeamProfile {
    team_by_name(team)
        .map(|t| TeamProfile {
            formation: t.formation,
            tactic: t.tactic,
        })
        .unwrap_or(TeamProfile {
            formation: "4-4-2",
            tactic: "counter",
        })
}
