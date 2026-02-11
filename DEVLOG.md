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

## 2026-02-10 - Team selection controls (manual + CPU auto-fill)

### Scope completed
- Added interactive create modal in TUI for team slot configuration.
- Added per-slot selection mode:
  - `Manual` team selection
  - `CPU auto-fill`
- Kept deterministic behavior by resolving CPU team picks with the instance seed.
- Added keyboard controls for modal flow:
  - `m` set manual, `p` set CPU
  - `[` / `]` or left/right to cycle manual teams
  - up/down to switch slot
  - Enter to create, Esc to cancel
- Updated `quick` command to support optional `--home` / `--away`:
  - If missing, CPU auto-fills from remaining teams.
  - Same seed yields the same auto-filled teams and outcomes.

## 2026-02-10 - Live standings/bracket updates + possession surfacing

### Scope completed
- Added explicit possession fields to match results and surfaced them in quick mode output.
- Added possession row to single-match CSV export output.
- Implemented live detail updates driven by simulation frames:
  - per-frame stats updates
  - per-frame competition panel updates
  - incremental history updates
- Added live league standings snapshots after each fixture.
- Added ASCII knockout tree bracket snapshots and updates after each semifinal/final.
- Tuned detail view layout to allocate more space to stats/competition/history panels.

## 2026-02-10 - Fullscreen info modals for readability

### Scope completed
- Added dedicated fullscreen overlay modals for:
  - Stats (`t`)
  - Standings/Bracket (`g`)
  - History (`h`)
- Added modal scrolling controls (`j/k` or up/down) and close controls (`Esc` or `q`).
- Simplified detail view to focus on scoreboard, logs, and instance info.
- Added detail-panel hint bar to direct users to the new dedicated modals.

## 2026-02-10 - National team expansion

### Scope completed
- Expanded team database to include 50+ national teams in addition to existing clubs.
- Added national-team flag mappings, including `PRC China`.
- Added tactic/formation profile mappings for the new national teams.
- Verified with `list` and deterministic `quick` simulation using national teams.
