use serde::{Deserialize, Serialize};

pub const DEFAULT_360: &str = "https://360-search.com/search?q-%s";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchEngine {
    pub name: String,
    pub template: String,
}

impl Default for SearchEngine {
    fn default() -> Self {
        Self { name: "360 Search".into(), template: DEFAULT_360.into() }
    }
}

pub fn resolve_input(input: &str, engine: &SearchEngine) -> String {
    if input.starts_with("http://") || input.starts_with("https://") || input.starts_with("file://") || input.starts_with("orbit://") {
        return input.to_string();
    }
    if input.contains('.') && !input.contains(' ') {
        return format!("https://{input}");
    }
    engine.template.replace("%s", &urlencoding::encode(input))
}
