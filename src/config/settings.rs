use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub dashboard: DashboardSettings,
    pub system: SystemSettings,
    pub display: DisplaySettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardSettings {
    pub title: String,
    pub refresh_rate_ms: u64,
    pub max_history_entries: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSettings {
    pub enable_process_monitoring: bool,
    pub max_processes_displayed: usize,
    pub cpu_history_length: usize,
    pub memory_history_length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplaySettings {
    pub show_cpu_graph: bool,
    pub show_memory_graph: bool,
    pub show_process_list: bool,
    pub show_network_info: bool,
    pub show_disk_info: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            dashboard: DashboardSettings {
                title: "System Monitor Dashboard".to_string(),
                refresh_rate_ms: 1000,
                max_history_entries: 100,
            },
            system: SystemSettings {
                enable_process_monitoring: true,
                max_processes_displayed: 20,
                cpu_history_length: 60,
                memory_history_length: 60,
            },
            display: DisplaySettings {
                show_cpu_graph: true,
                show_memory_graph: true,
                show_process_list: true,
                show_network_info: true,
                show_disk_info: true,
            },
        }
    }
}

impl Settings {
    pub fn load(config_path: &str) -> Result<Self> {
        match fs::read_to_string(config_path) {
            Ok(content) => {
                let settings: Settings = toml::from_str(&content)
                    .with_context(|| format!("Failed to parse config file: {}", config_path))?;
                Ok(settings)
            }
            Err(_) => {
                // Create default config file if it doesn't exist
                let default_settings = Settings::default();
                default_settings.save(config_path)?;
                Ok(default_settings)
            }
        }
    }

    pub fn save(&self, config_path: &str) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize settings to TOML")?;
        fs::write(config_path, content)
            .with_context(|| format!("Failed to write config file: {}", config_path))?;
        Ok(())
    }
}