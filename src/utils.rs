use std::time::{SystemTime, UNIX_EPOCH};

pub const CSV_INJECTION_PREFIX: char = '\'';

#[derive(Debug, Clone)]
pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        let seeded = if seed == 0 { 0x9E3779B97F4A7C15 } else { seed };
        Self {
            state: splitmix64(seeded),
        }
    }

    pub fn from_time() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0xA5A5_A5A5_A5A5_A5A5);
        Self::new(now)
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }

    pub fn next_f64(&mut self) -> f64 {
        let v = self.next_u64() >> 11;
        (v as f64) * (1.0 / ((1u64 << 53) as f64))
    }

    pub fn chance(&mut self, p: f64) -> bool {
        self.next_f64() < p.clamp(0.0, 1.0)
    }

    pub fn range_usize(&mut self, upper_exclusive: usize) -> usize {
        if upper_exclusive <= 1 {
            return 0;
        }
        (self.next_u64() % (upper_exclusive as u64)) as usize
    }
}

pub fn derive_seed(base_seed: u64, salt: u64) -> u64 {
    splitmix64(base_seed ^ salt.wrapping_mul(0x9E3779B97F4A7C15))
}

pub fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

pub fn pad2(minute: u8) -> String {
    format!("{:02}", minute)
}

pub fn sanitize_csv_cell(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.starts_with('=')
        || trimmed.starts_with('+')
        || trimmed.starts_with('-')
        || trimmed.starts_with('@')
    {
        format!("{}{}", CSV_INJECTION_PREFIX, raw)
    } else {
        raw.to_string()
    }
}

pub fn csv_escape(field: &str) -> String {
    let sanitized = sanitize_csv_cell(field);
    let needs_quotes =
        sanitized.contains(',') || sanitized.contains('\n') || sanitized.contains('"');
    if needs_quotes {
        format!("\"{}\"", sanitized.replace('"', "\"\""))
    } else {
        sanitized
    }
}
