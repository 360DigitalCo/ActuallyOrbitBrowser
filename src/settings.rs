use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use crate::search::{SearchEngine, DEFAULT_360};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnimationMode { Full, Reduced, Off }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitSettings {
    pub animations: AnimationMode,
    pub search_engine: SearchEngine,
    pub engines: Vec<SearchEngine>,
}

impl Default for OrbitSettings {
    fn default() -> Self {
        Self {
            animations: AnimationMode::Full,
            search_engine: SearchEngine::default(),
            engines: vec![
                SearchEngine { name: "360 Search".into(), template: DEFAULT_360.into() },
                SearchEngine { name: "DuckDuckGo".into(), template: "https://duckduckgo.com/?q=%s".into() },
                SearchEngine { name: "Brave Search".into(), template: "https://search.brave.com/search?q=%s".into() },
                SearchEngine { name: "Bing".into(), template: "https://www.bing.com/search?q=%s".into() },
            ],
        }
    }
}

impl OrbitSettings {
    pub fn load() -> Self {
        let path = settings_path();
        fs::read_to_string(path).ok().and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default()
    }

    pub fn save(&self) {
        let path = settings_path();
        if let Some(parent) = path.parent() { let _ = fs::create_dir_all(parent); }
        if let Ok(data) = serde_json::to_string_pretty(self) { let _ = fs::write(path, data); }
    }
}

fn settings_path() -> PathBuf {
    dirs::config_dir().unwrap_or_else(|| PathBuf::from("."))
        .join("Orbit")
        .join("settings.json")
}

pub fn settings_url(settings: &OrbitSettings) -> String {
    let mode = match settings.animations { AnimationMode::Full => "full", AnimationMode::Reduced => "reduced", AnimationMode::Off => "off" };
    format!("data:text/html,{}", urlencoding::encode(&settings_page(mode)))
}

fn settings_page(mode: &str) -> String {
    format!(r#"<!doctype html><meta charset=utf-8><title>Orbit Settings</title>
<style>
body{{margin:0;background:#0b0f19;color:#f8fafc;font:15px system-ui;padding:40px;max-width:820px}}
h1{{font-size:30px;margin:0 0 8px}} p{{color:#94a3b8}} section{{border:1px solid #263041;border-radius:8px;padding:20px;margin-top:18px}}
a,button{{color:#f8fafc;background:transparent;border:1px solid #4a5568;border-radius:6px;padding:11px 15px;margin:4px;cursor:pointer}}
button:hover{{background:#f8fafc;color:#0b0f19}} .active{{background:#34d399;color:#064e3b;border-color:#34d399}}
</style><h1>Orbit Settings</h1><p>Browser controls and search configuration. Settings are stored locally.</p>
<section><h2>Animation control</h2><p>Current mode: <b>{mode}</b></p>
<button onclick="location.href='orbit://settings?animations=full'">Full</button>
<button onclick="location.href='orbit://settings?animations=reduced'">Reduced</button>
<button onclick="location.href='orbit://settings?animations=off'">Off</button></section>
<section><h2>Search</h2><p>Default: 360 Search. Custom engines are configured in Orbit's local settings file.</p></section>
<section><h2>Browser</h2><p>Orbit uses Servo for HTML, CSS and JavaScript. No Chromium, Electron, or WebView2 dependency.</p></section>"#)
}
