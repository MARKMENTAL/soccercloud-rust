const state = {
  teams: [],
  simulations: [],
  selectedDetailId: null,
  pollHandle: null,
  demo: false,
};

const $ = (id) => document.getElementById(id);
const THEME_STORAGE_KEY = "soccercloud.web.theme";
const THEMES = {
  "default-light": "Default (Light)",
  dark: "Dark",
  matcha: "Matcha",
  "black-mesa": "Black Mesa",
};

function isThemeAllowed(theme) {
  return Object.prototype.hasOwnProperty.call(THEMES, theme);
}

function getSavedTheme() {
  try {
    const saved = localStorage.getItem(THEME_STORAGE_KEY);
    if (saved && isThemeAllowed(saved)) {
      return saved;
    }
  } catch (_) {}
  return "default-light";
}

function applyTheme(theme) {
  const normalized = isThemeAllowed(theme) ? theme : "default-light";
  document.documentElement.dataset.theme = normalized;
  const select = $("themeSelect");
  if (select && select.value !== normalized) {
    select.value = normalized;
  }
  try {
    localStorage.setItem(THEME_STORAGE_KEY, normalized);
  } catch (_) {}
}

function initThemeControls() {
  const select = $("themeSelect");
  if (!select) return;
  const initial = getSavedTheme();
  applyTheme(initial);
  select.addEventListener("change", () => {
    applyTheme(select.value);
    setStatus(`Theme: ${THEMES[select.value] || "Default (Light)"}`);
  });
}

function setStatus(message) {
  $("statusText").textContent = message;
}

async function request(path, options = {}) {
  const response = await fetch(path, options);
  if (!response.ok) {
    let msg = `${response.status} ${response.statusText}`;
    try {
      const body = await response.json();
      if (body && body.error) {
        msg = body.error;
      }
    } catch (_) {}
    throw new Error(msg);
  }

  if (response.status === 204) {
    return null;
  }

  const contentType = response.headers.get("content-type") || "";
  if (contentType.includes("application/json")) {
    return response.json();
  }
  return response.text();
}

function openModal(modalId) {
  const modal = $(modalId);
  if (!modal) return;
  modal.classList.add("open");
  modal.setAttribute("aria-hidden", "false");
}

function closeModal(modalId) {
  const modal = $(modalId);
  if (!modal) return;
  modal.classList.remove("open");
  modal.setAttribute("aria-hidden", "true");
}

function getModeTeamCount(mode) {
  return mode === "single" ? 2 : 4;
}

function renderTeamSelectors() {
  const wrap = $("teamSelectWrap");
  const mode = $("modeSelect").value;
  const required = getModeTeamCount(mode);
  const autoFill = $("autoFill").checked;

  $("teamCount").innerHTML = `<option value="${required}">${required}</option>`;

  if (state.teams.length === 0) {
    wrap.innerHTML = "<p>Loading teams...</p>";
    return;
  }

  const options = state.teams
    .map((team) => `<option value="${team.name}">${team.display_name}</option>`)
    .join("");

  const fields = [];
  for (let i = 0; i < required; i++) {
    fields.push(`
      <label>
        Team ${i + 1}
        <select data-team-index="${i}" ${autoFill ? "disabled" : ""}>
          ${options}
        </select>
      </label>
    `);
  }

  wrap.innerHTML = fields.join("");
}

function getCreatePayload() {
  const mode = $("modeSelect").value;
  const autoFill = $("autoFill").checked;
  const required = getModeTeamCount(mode);

  if (autoFill) {
    return { mode, auto_fill: true };
  }

  const picks = [...$("teamSelectWrap").querySelectorAll("select")].map((s) => s.value);
  const uniqueCount = new Set(picks).size;
  if (picks.length !== required || uniqueCount !== picks.length) {
    throw new Error("Please select unique teams for this mode");
  }

  return { mode, auto_fill: false, teams: picks };
}

function cardActions(sim) {
  const deleteButton = state.demo ? "" : `<button class="btn warn" data-action="delete" data-id="${sim.id}">Delete</button>`;
  const common = `
    <button class="btn secondary" data-action="view" data-id="${sim.id}">View</button>
    <button class="btn secondary" data-action="clone" data-id="${sim.id}">Clone</button>
    ${deleteButton}
  `;

  if (sim.status === "pending") {
    return `
      <button class="btn" data-action="start" data-id="${sim.id}">Start</button>
      ${common}
    `;
  }

  if (sim.status === "completed") {
    return `
      <button class="btn secondary" data-action="export" data-id="${sim.id}">Export CSV</button>
      ${common}
    `;
  }

  return common;
}

function createCardElement(sim) {
  const article = document.createElement("article");
  article.className = "card";
  article.dataset.id = String(sim.id);
  article.innerHTML = `
    <div class="card-top">
      <div>
        <h3 class="card-title" data-role="title"></h3>
        <p class="card-id" data-role="idline"></p>
      </div>
      <span class="pill" data-role="pill"></span>
    </div>
    <p class="card-line"><strong>Progress:</strong> <span data-role="progress"></span></p>
    <p class="card-score" data-role="scoreboard"></p>
    <p class="card-line"><strong>Outcome:</strong> <span data-role="outcome"></span></p>
    <div class="card-actions" data-role="actions"></div>
  `;
  return article;
}

function setTextIfChanged(element, nextValue) {
  if (!element) return;
  if (element.textContent !== nextValue) {
    element.textContent = nextValue;
  }
}

function updateCardElement(card, sim) {
  setTextIfChanged(card.querySelector('[data-role="title"]'), sim.title);
  setTextIfChanged(card.querySelector('[data-role="idline"]'), `sim-${sim.id} | ${sim.mode} | seed=${sim.seed}`);
  setTextIfChanged(card.querySelector('[data-role="progress"]'), sim.progress);
  setTextIfChanged(card.querySelector('[data-role="scoreboard"]'), sim.scoreboard);
  setTextIfChanged(card.querySelector('[data-role="outcome"]'), sim.outcome);

  const pill = card.querySelector('[data-role="pill"]');
  const nextPillClass = `pill ${sim.status}`;
  if (pill.className !== nextPillClass) {
    pill.className = nextPillClass;
  }
  setTextIfChanged(pill, sim.status);

  if (card.dataset.status !== sim.status) {
    card.dataset.status = sim.status;
    card.querySelector('[data-role="actions"]').innerHTML = cardActions(sim);
  }
}

function renderDashboard() {
  const root = $("dashboard");
  if (!root) return;

  if (state.simulations.length === 0) {
    for (const card of root.querySelectorAll("article.card")) {
      card.remove();
    }
    if (!$("dashboardEmpty")) {
      root.innerHTML = `<div class="empty" id="dashboardEmpty">No simulation instances yet. Create one to start the web simulation flow.</div>`;
    }
    return;
  }

  const emptyNode = $("dashboardEmpty");
  if (emptyNode) {
    emptyNode.remove();
  }

  const existingCards = new Map(
    [...root.querySelectorAll("article.card")].map((card) => [Number(card.dataset.id), card]),
  );

  const activeIds = new Set(state.simulations.map((sim) => sim.id));

  for (let index = 0; index < state.simulations.length; index += 1) {
    const sim = state.simulations[index];
    let card = existingCards.get(sim.id);
    if (!card) {
      card = createCardElement(sim);
      root.appendChild(card);
    }

    updateCardElement(card, sim);

    if (root.children[index] !== card) {
      root.insertBefore(card, root.children[index] || null);
    }
  }

  for (const [id, card] of existingCards.entries()) {
    if (!activeIds.has(id)) {
      card.remove();
    }
  }
}

function renderStats() {
  const total = state.simulations.length;
  const running = state.simulations.filter((s) => s.status === "running").length;
  const completed = state.simulations.filter((s) => s.status === "completed").length;
  $("statInstances").textContent = String(total);
  $("statRunning").textContent = String(running);
  $("statCompleted").textContent = String(completed);
  $("statTeams").textContent = String(state.teams.length);
}

async function refreshSimulations() {
  state.simulations = await request("/api/simulations");
  renderStats();
  renderDashboard();

  if (state.selectedDetailId !== null) {
    const exists = state.simulations.some((s) => s.id === state.selectedDetailId);
    if (!exists) {
      state.selectedDetailId = null;
      closeModal("detailModal");
      return;
    }
    await loadDetail(state.selectedDetailId);
  }
}

function textOrPlaceholder(lines, fallback) {
  if (!Array.isArray(lines) || lines.length === 0) {
    return fallback;
  }
  return lines.join("\n");
}

async function loadDetail(id) {
  const detail = await request(`/api/simulations/${id}`);
  state.selectedDetailId = id;

  $("detailTitle").textContent = `sim-${detail.id} - ${detail.title}`;
  $("detailScoreboard").textContent = `${detail.scoreboard} | ${detail.outcome}`;
  $("detailLogs").textContent = textOrPlaceholder(detail.logs, "No events yet.");
  $("detailStats").textContent = textOrPlaceholder(detail.stats_lines, "No stats available yet.");
  $("detailCompetition").textContent = textOrPlaceholder(detail.competition_lines, "No standings/bracket available yet.");
  $("detailHistory").textContent = textOrPlaceholder(detail.history_lines, "No history recorded yet.");
}

async function createSimulation() {
  try {
    const payload = getCreatePayload();
    const created = await request("/api/simulations", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(payload),
    });
    closeModal("createModal");
    setStatus(`Created sim-${created.id}`);
    await refreshSimulations();
  } catch (error) {
    setStatus(`Create failed: ${error.message}`);
  }
}

async function startSimulation(id) {
  try {
    await request(`/api/simulations/${id}/start`, { method: "POST" });
    setStatus(`Started sim-${id}`);
    await refreshSimulations();
  } catch (error) {
    setStatus(`Start failed: ${error.message}`);
  }
}

async function cloneSimulation(id) {
  try {
    const created = await request(`/api/simulations/${id}/clone`, { method: "POST" });
    setStatus(`Cloned sim-${id} to sim-${created.id}`);
    await refreshSimulations();
  } catch (error) {
    setStatus(`Clone failed: ${error.message}`);
  }
}

async function deleteSimulation(id) {
  try {
    await request(`/api/simulations/${id}`, { method: "DELETE" });
    if (state.selectedDetailId === id) {
      state.selectedDetailId = null;
      closeModal("detailModal");
    }
    setStatus(`Deleted sim-${id}`);
    await refreshSimulations();
  } catch (error) {
    setStatus(`Delete failed: ${error.message}`);
  }
}

function exportSimulation(id) {
  window.location.href = `/api/simulations/${id}/export.csv`;
  setStatus(`Exporting sim-${id} CSV...`);
}

function bindEvents() {
  $("openCreateBtn").addEventListener("click", () => openModal("createModal"));
  $("createBtn").addEventListener("click", createSimulation);
  $("modeSelect").addEventListener("change", renderTeamSelectors);
  $("autoFill").addEventListener("change", renderTeamSelectors);

  document.querySelectorAll("[data-close]").forEach((button) => {
    button.addEventListener("click", () => {
      closeModal(button.dataset.close);
      if (button.dataset.close === "detailModal") {
        state.selectedDetailId = null;
      }
    });
  });

  ["createModal", "detailModal"].forEach((id) => {
    const modal = $(id);
    modal.addEventListener("click", (event) => {
      if (event.target === modal) {
        closeModal(id);
        if (id === "detailModal") {
          state.selectedDetailId = null;
        }
      }
    });
  });

  $("dashboard").addEventListener("click", async (event) => {
    const button = event.target.closest("button[data-action]");
    if (!button) return;

    const id = Number(button.dataset.id);
    const action = button.dataset.action;

    if (action === "start") return startSimulation(id);
    if (action === "clone") return cloneSimulation(id);
    if (action === "delete") return deleteSimulation(id);
    if (action === "export") return exportSimulation(id);
    if (action === "view") {
      try {
        await loadDetail(id);
        openModal("detailModal");
      } catch (error) {
        setStatus(`Failed to load detail: ${error.message}`);
      }
    }
  });
}

async function boot() {
  initThemeControls();
  bindEvents();
  try {
    setStatus("Loading configuration, teams and simulations...");
    const config = await request("/api/config");
    state.demo = config.demo;
    
    // Show/hide demo banner
    const demoBanner = $("demoBanner");
    if (demoBanner) {
      if (state.demo) {
        demoBanner.classList.add("visible");
      } else {
        demoBanner.classList.remove("visible");
      }
    }
    
    state.teams = await request("/api/teams");
    renderTeamSelectors();
    await refreshSimulations();
    setStatus(`Connected to SoccerCloud backend on port 9009.${state.demo ? " (Demo mode)" : ""}`);
  } catch (error) {
    setStatus(`Startup failed: ${error.message}`);
    return;
  }

  if (state.pollHandle) {
    clearInterval(state.pollHandle);
  }

  state.pollHandle = setInterval(async () => {
    try {
      await refreshSimulations();
    } catch (error) {
      setStatus(`Polling issue: ${error.message}`);
    }
  }, 500);
}

boot();
