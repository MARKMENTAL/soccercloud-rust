// =======================================================
// SoccerCloud Simulator Data File
// Contains all team, flag, and profile information.
// =======================================================

const teams = [
  // J-League
  "Kashima Antlers","Urawa Red Diamonds","Gamba Osaka","Cerezo Osaka","Kawasaki Frontale",
  "Yokohama F. Marinos","Nagoya Grampus","Shimizu S-Pulse","Sanfrecce Hiroshima","Consadole Sapporo",
  "Ventforet Kofu","Tokyo Verdy","JEF United Chiba",
  // Euro clubs
  "Arsenal","FC Barcelona","Real Madrid","Manchester City","Manchester United","Liverpool",
  "Bayern Munich","Borussia Dortmund","Paris Saint-Germain","Juventus","Inter","AC Milan",
  "Ajax","Benfica","Porto","Celtic"
];

const TEAM_FLAGS = {
  // Japan
  "Kashima Antlers":"🇯🇵","Urawa Red Diamonds":"🇯🇵","Gamba Osaka":"🇯🇵","Cerezo Osaka":"🇯🇵","Kawasaki Frontale":"🇯🇵",
  "Yokohama F. Marinos":"🇯🇵","Nagoya Grampus":"🇯🇵","Shimizu S-Pulse":"🇯🇵","Sanfrecce Hiroshima":"🇯🇵","Consadole Sapporo":"🇯🇵",
  "Ventforet Kofu":"🇯🇵","Tokyo Verdy":"🇯🇵", "JEF United Chiba":"🇯🇵",
  // UK
  "Arsenal":"🇬🇧","Manchester City":"🇬🇧","Manchester United":"🇬🇧","Liverpool":"🇬🇧","Celtic":"🇬🇧",
  // Spain
  "FC Barcelona":"🇪🇸","Real Madrid":"🇪🇸",
  // Germany
  "Bayern Munich":"🇩🇪","Borussia Dortmund":"🇩🇪",
  // France
  "Paris Saint-Germain":"🇫🇷",
  // Italy
  "Juventus":"🇮🇹","Inter":"🇮🇹","AC Milan":"🇮🇹",
  // Netherlands
  "Ajax":"🇳🇱",
  // Portugal
  "Benfica":"🇵🇹","Porto":"🇵🇹"
};

const FORMATIONS = ["4-4-2","4-3-3","4-2-3-1","3-5-2","5-4-1"];

const TACTICS = {
  counter:     { label:"Counter",      attackBias:1.10, goalMult:1.08, fastBreak:0.25, foulMult:1.00, blockMult:1.00, pressMult:0.95 },
  possession:  { label:"Possession",   attackBias:1.00, goalMult:0.95, fastBreak:0.10, foulMult:0.90, blockMult:1.00, pressMult:0.90 },
  high_press:  { label:"High Press",   attackBias:1.15, goalMult:1.00, fastBreak:0.20, foulMult:1.20, blockMult:0.95, pressMult:1.20 },
  low_block:   { label:"Low Block",    attackBias:0.92, goalMult:0.92, fastBreak:0.12, foulMult:0.95, blockMult:1.15, pressMult:0.85 },
};

const TEAM_PROFILES = {
  // Europe
  "Arsenal":              { formation:"4-3-3",   tactic:"possession" },
  "FC Barcelona":         { formation:"4-3-3",   tactic:"possession" },
  "Real Madrid":          { formation:"4-3-3",   tactic:"counter" },
  "Manchester City":      { formation:"4-3-3",   tactic:"possession" },
  "Manchester United":    { formation:"4-2-3-1", tactic:"high_press" },
  "Liverpool":            { formation:"4-3-3",   tactic:"high_press" },
  "Bayern Munich":        { formation:"4-2-3-1", tactic:"high_press" },
  "Borussia Dortmund":    { formation:"4-2-3-1", tactic:"high_press" },
  "Paris Saint-Germain":  { formation:"4-3-3",   tactic:"possession" },
  "Juventus":             { formation:"3-5-2",   tactic:"low_block" },
  "Inter":                { formation:"3-5-2",   tactic:"low_block" },
  "AC Milan":             { formation:"4-2-3-1", tactic:"possession" },
  "Ajax":                 { formation:"4-3-3",   tactic:"possession" },
  "Benfica":              { formation:"4-2-3-1", tactic:"possession" },
  "Porto":                { formation:"4-4-2",   tactic:"counter" },
  "Celtic":               { formation:"4-3-3",   tactic:"possession" },
  // J-League (generic lean)
  "Kawasaki Frontale":    { formation:"4-3-3",   tactic:"possession" },
  "Yokohama F. Marinos":  { formation:"4-3-3",   tactic:"high_press" },
  "Kashima Antlers":      { formation:"4-4-2",   tactic:"counter" },
  "Urawa Red Diamonds":   { formation:"4-2-3-1", tactic:"possession" },
  "Gamba Osaka":          { formation:"4-4-2",   tactic:"counter" },
  "Cerezo Osaka":         { formation:"4-4-2",   tactic:"counter" },
  "Nagoya Grampus":       { formation:"4-2-3-1", tactic:"low_block" },
  "Sanfrecce Hiroshima":  { formation:"3-5-2",   tactic:"possession" },
  "Consadole Sapporo":    { formation:"3-5-2",   tactic:"high_press" },
  "Shimizu S-Pulse":      { formation:"4-4-2",   tactic:"counter" },
  "Ventforet Kofu":       { formation:"4-4-2",   tactic:"counter" },
  "Tokyo Verdy":          { formation:"4-3-3",   tactic:"possession" },
  "JEF United Chiba":          { formation:"4-3-3",   tactic:"counter" }
};

