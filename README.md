# ⚽ SoccerCloud — Cloudified Soccer Simulation Environment

**Live Demo:** [https://mentalnet.xyz/soccercloud/](https://mentalnet.xyz/soccercloud/)  
**Author:** [markmental / MentalNet.xyz](https://mentalnet.xyz)  
**License:** MIT  

---

## 🧠 Overview

**SoccerCloud** is a browser-based soccer simulator that reimagines match simulations through the aesthetic and structure of a **cloud orchestration dashboard** — think *OpenStack meets Football Manager*.

Each match, league, or knockout bracket behaves like a **“virtual instance”**, complete with lifecycle controls:
- **Create / Start / View / Delete / Clone / Export**  
- Real-time logs, xG data, formations, and tactical analytics  
- **Dynamic UI** styled like a cloud console with per-match telemetry

SoccerCloud is written entirely in **HTML, CSS, and vanilla JavaScript**, with no external backend dependencies. It runs fully client-side and is suitable for static hosting.

---

## 🌐 Live Deployment

> [https://mentalnet.xyz/soccercloud/](https://mentalnet.xyz/soccercloud/)

Hosted on **MentalNet.xyz**, the current deployment showcases all features including:
- Match instance dashboard  
- 4-team League and Knockout modes  
- CSV export of results and tables  
- Auto-team picker with J-League and European clubs  
- Cloud-inspired modal configuration UI  

---

## 🏗️ Features

| Category | Description |
|-----------|-------------|
| **Simulation Types** | Single Match, 4-Team League, 4-Team Knockout |
| **Team Database** | Includes J-League + top European clubs with realistic formations/tactics |
| **UI Design** | Styled like a lightweight OpenStack/Proxmox console |
| **Export Options** | Download match or league data as CSV |
| **Logging & Recaps** | Live xG updates, goal commentary, and tactical analysis |
| **Client-Only** | Runs directly in browser — no backend needed |

---

## 🗂️ Project Structure

```

soccercloud/
├── index.html      # Main web dashboard and simulation logic
├── data.js         # Team definitions, flags, formations, and tactics
└── assets/         # (Optional) icons, logos, or future expansion files

````

---

## 🚀 Getting Started (Local)

You can run SoccerCloud locally with **no build process** — just open it in a browser.

### Option 1: Double-click
```bash
open index.html
````

### Option 2: Local dev server

```bash
python3 -m http.server 8080
```

Then visit:
👉 `http://localhost:8080`

---

## 🧩 Technical Notes

* Written in **vanilla JavaScript** for speed and transparency.
* Each simulation instance is handled via a `SimulationInstance` class.
* Data persistence is session-based; future versions may support saving instance states.
* CSS uses retro **UnifrakturCook + Press Start 2P** fonts for a distinct MentalNet look.

---

## 🖥️ Upcoming: CLI Edition

> ⚡ **tuxsoccercloud** *(Coming soon!)*

A simplified **terminal version** of the simulator is in development — ideal for users who prefer a command-line workflow or want to integrate match simulations into scripts or data pipelines.

Planned features:

* Text-only match recaps and league tables
* Randomized or argument-based team selection
* Fully offline operation

---

## 🤝 Contributing

Pull requests are welcome (when I get signups up)!
To contribute:

1. Fork this repository
2. Make your edits in a feature branch
3. Submit a pull request with a clear description

---

## 💡 Credits

* Built and designed by **markmental**
* Hosted under **MentalNet.xyz**
* Inspired by *OpenStack Horizon* dashboards and *Football Manager*-style simulations
* Font assets via [Google Fonts](https://fonts.google.com)
* Icons via [Font Awesome](https://fontawesome.com)

---

### ⚽ *"Deploy your next match like a VM — welcome to SoccerCloud."*

