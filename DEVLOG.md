# DEVLOG

## 2026-02-10 - Rust CLI/TUI rebuild (strict dependencies)

### Scope completed
- Initialized a Rust project with strict dependency policy: `ratatui`, `crossterm`, `clap` only.
- Added global `--seed` support for reproducible runs across TUI and headless commands.
- Ported team/tactic/profile data from JavaScript into `src/data.rs`.
- Implemented a std-only deterministic RNG in `src/utils.rs`.
- Ported core simulation logic to Rust in `src/sim.rs`:
  - Single match minute loop with xG, tactics, fouls/cards, corners, offsides, saves.
  - 4-team league mode with fixtures and standings tie-breakers.
  - 4-team knockout mode with penalties and edge-case loop guard.
- Implemented lifecycle state machine and bounded logs in `src/instance.rs`.
- Built a ratatui dashboard/detail UI with keyboard controls in `src/app.rs` + `src/ui/*`.
- Implemented CSV export in `src/export.rs` with manual escaping and CSV injection sanitization.
- Added CLI commands:
  - `quick` (single match)
  - `list` (team listing)
  - `export` (CSV generation)

### Key controls in TUI
- `n`: create Single Match instance
- `l`: create 4-Team League instance
- `o`: create 4-Team Knockout instance
- `s`: start selected instance
- `c`: clone selected instance
- `d`: delete selected instance
- `e`: export selected instance to CSV
- `v` or `Enter`: toggle dashboard/detail
- `1`, `2`, `4`, `0`: speed control (1x/2x/4x/instant)
- `j`/`k` or arrow keys: select instance
- `q`: quit

### Notes
- No async runtime is used.
- No external data files are required at runtime.
- Team data is fully embedded in the binary.
- CSV export format is mode-specific and generated with std I/O.
