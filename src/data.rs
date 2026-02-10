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

#[derive(Debug, Clone, Copy)]
pub struct TeamProfile {
    pub formation: &'static str,
    pub tactic: &'static str,
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

pub const TEAMS: [&str; 29] = [
    "Kashima Antlers",
    "Urawa Red Diamonds",
    "Gamba Osaka",
    "Cerezo Osaka",
    "Kawasaki Frontale",
    "Yokohama F. Marinos",
    "Nagoya Grampus",
    "Shimizu S-Pulse",
    "Sanfrecce Hiroshima",
    "Consadole Sapporo",
    "Ventforet Kofu",
    "Tokyo Verdy",
    "JEF United Chiba",
    "Arsenal",
    "FC Barcelona",
    "Real Madrid",
    "Manchester City",
    "Manchester United",
    "Liverpool",
    "Bayern Munich",
    "Borussia Dortmund",
    "Paris Saint-Germain",
    "Juventus",
    "Inter",
    "AC Milan",
    "Ajax",
    "Benfica",
    "Porto",
    "Celtic",
];

pub fn team_flag(team: &str) -> &'static str {
    match team {
        "Kashima Antlers"
        | "Urawa Red Diamonds"
        | "Gamba Osaka"
        | "Cerezo Osaka"
        | "Kawasaki Frontale"
        | "Yokohama F. Marinos"
        | "Nagoya Grampus"
        | "Shimizu S-Pulse"
        | "Sanfrecce Hiroshima"
        | "Consadole Sapporo"
        | "Ventforet Kofu"
        | "Tokyo Verdy"
        | "JEF United Chiba" => "🇯🇵",
        "Arsenal" | "Manchester City" | "Manchester United" | "Liverpool" | "Celtic" => "🇬🇧",
        "FC Barcelona" | "Real Madrid" => "🇪🇸",
        "Bayern Munich" | "Borussia Dortmund" => "🇩🇪",
        "Paris Saint-Germain" => "🇫🇷",
        "Juventus" | "Inter" | "AC Milan" => "🇮🇹",
        "Ajax" => "🇳🇱",
        "Benfica" | "Porto" => "🇵🇹",
        _ => "🏳️",
    }
}

pub fn display_name(team: &str) -> String {
    format!("{} {}", team_flag(team), team)
}

pub fn tactic_by_key(key: &str) -> Tactic {
    TACTICS
        .iter()
        .copied()
        .find(|t| t.key == key)
        .unwrap_or(TACTICS[0])
}

pub fn profile_for(team: &str) -> TeamProfile {
    match team {
        "Arsenal" => TeamProfile {
            formation: "4-3-3",
            tactic: "possession",
        },
        "FC Barcelona" => TeamProfile {
            formation: "4-3-3",
            tactic: "possession",
        },
        "Real Madrid" => TeamProfile {
            formation: "4-3-3",
            tactic: "counter",
        },
        "Manchester City" => TeamProfile {
            formation: "4-3-3",
            tactic: "possession",
        },
        "Manchester United" => TeamProfile {
            formation: "4-2-3-1",
            tactic: "high_press",
        },
        "Liverpool" => TeamProfile {
            formation: "4-3-3",
            tactic: "high_press",
        },
        "Bayern Munich" => TeamProfile {
            formation: "4-2-3-1",
            tactic: "high_press",
        },
        "Borussia Dortmund" => TeamProfile {
            formation: "4-2-3-1",
            tactic: "high_press",
        },
        "Paris Saint-Germain" => TeamProfile {
            formation: "4-3-3",
            tactic: "possession",
        },
        "Juventus" => TeamProfile {
            formation: "3-5-2",
            tactic: "low_block",
        },
        "Inter" => TeamProfile {
            formation: "3-5-2",
            tactic: "low_block",
        },
        "AC Milan" => TeamProfile {
            formation: "4-2-3-1",
            tactic: "possession",
        },
        "Ajax" => TeamProfile {
            formation: "4-3-3",
            tactic: "possession",
        },
        "Benfica" => TeamProfile {
            formation: "4-2-3-1",
            tactic: "possession",
        },
        "Porto" => TeamProfile {
            formation: "4-4-2",
            tactic: "counter",
        },
        "Celtic" => TeamProfile {
            formation: "4-3-3",
            tactic: "possession",
        },
        "Kawasaki Frontale" => TeamProfile {
            formation: "4-3-3",
            tactic: "possession",
        },
        "Yokohama F. Marinos" => TeamProfile {
            formation: "4-3-3",
            tactic: "high_press",
        },
        "Kashima Antlers" => TeamProfile {
            formation: "4-4-2",
            tactic: "counter",
        },
        "Urawa Red Diamonds" => TeamProfile {
            formation: "4-2-3-1",
            tactic: "possession",
        },
        "Gamba Osaka" => TeamProfile {
            formation: "4-4-2",
            tactic: "counter",
        },
        "Cerezo Osaka" => TeamProfile {
            formation: "4-4-2",
            tactic: "counter",
        },
        "Nagoya Grampus" => TeamProfile {
            formation: "4-2-3-1",
            tactic: "low_block",
        },
        "Sanfrecce Hiroshima" => TeamProfile {
            formation: "3-5-2",
            tactic: "possession",
        },
        "Consadole Sapporo" => TeamProfile {
            formation: "3-5-2",
            tactic: "high_press",
        },
        "Shimizu S-Pulse" => TeamProfile {
            formation: "4-4-2",
            tactic: "counter",
        },
        "Ventforet Kofu" => TeamProfile {
            formation: "4-4-2",
            tactic: "counter",
        },
        "Tokyo Verdy" => TeamProfile {
            formation: "4-3-3",
            tactic: "possession",
        },
        "JEF United Chiba" => TeamProfile {
            formation: "4-3-3",
            tactic: "counter",
        },
        _ => TeamProfile {
            formation: "4-4-2",
            tactic: "counter",
        },
    }
}
