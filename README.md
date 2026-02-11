# SoccerCloud CLI (Rust)

Terminal-native rebuild of MentalNet SoccerCloud with a cloud-dashboard feel.

## Overview

This project is a Rust TUI/CLI soccer simulator with:

- Single Match, 4-Team League, and 4-Team Knockout modes
- Live match logs, scoreboard, and instance lifecycle controls
- Seeded deterministic runs (`--seed`) for reproducible results
- CSV export for single, league, and knockout outputs
- Expanded team pool (clubs + 50+ national teams, including `PRC China`)

## Requirements

- Rust toolchain (stable)
- Cargo
- A terminal that supports UTF-8 and colors

Install Rust (if needed):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Setup

Clone and build:

```bash
git clone <your-repo-url>
cd soccercloud-cli
cargo check
```

Run in debug mode:

```bash
cargo run
```

Build optimized binary:

```bash
cargo build --release
./target/release/soccercloud
```

## CLI Usage

Default (interactive TUI):

```bash
soccercloud
```

or with Cargo:

```bash
cargo run --
```

Use a global seed for reproducibility:

```bash
cargo run -- --seed 42
```

### Quick match (headless)

```bash
cargo run -- quick --home "Arsenal" --away "Real Madrid" --seed 42
```

CPU auto-fill for missing team(s):

```bash
cargo run -- quick --home "England" --seed 42
cargo run -- quick --seed 42
```

### List teams

```bash
cargo run -- list
```

### Export CSV

Single:

```bash
cargo run -- export --mode single --team "Arsenal" --team "PRC China" --out match.csv --seed 42
```

League:

```bash
cargo run -- export --mode league4 --team "England" --team "Brazil" --team "Japan" --team "Germany" --out league.csv --seed 42
```

Knockout:

```bash
cargo run -- export --mode knockout4 --team "France" --team "Argentina" --team "Morocco" --team "PRC China" --out knockout.csv --seed 42
```

## TUI Controls

Global:

- `n` create Single instance
- `l` create League4 instance
- `o` create Knockout4 instance
- `s` start selected instance
- `c` clone selected instance
- `d` delete selected instance
- `e` export selected instance CSV
- `v` or `Enter` toggle dashboard/detail
- `j/k` or `Up/Down` navigate instances
- `1/2/4/0` speed control (1x/2x/4x/instant)
- `q` quit

Create modal:

- `m` set selected slot to manual team
- `p` set selected slot to CPU auto-fill
- `[` / `]` or `Left/Right` cycle manual team
- `Enter` create
- `Esc` cancel

Readable fullscreen data panels:

- `t` stats modal
- `g` standings/bracket modal
- `h` history modal
- `j/k` or `Up/Down` scroll inside modal
- `Esc` or `q` close modal

## Project Structure

```text
src/
├── main.rs        # CLI entrypoint and commands
├── app.rs         # App state and event loop
├── data.rs        # Teams, flags, tactics, profiles
├── sim.rs         # Match/league/knockout simulation engine
├── instance.rs    # Simulation instance lifecycle and state
├── export.rs      # CSV export
├── utils.rs       # RNG + helper utilities
└── ui/
    ├── mod.rs
    ├── dashboard.rs
    ├── detail.rs
    ├── modal.rs
    └── widgets.rs
```

## Notes

- Dependency policy is intentionally strict (minimal crates).
- Team data is embedded in the binary (no external runtime data files).
- Use `--seed` for deterministic comparisons and debugging.

## License

MIT
