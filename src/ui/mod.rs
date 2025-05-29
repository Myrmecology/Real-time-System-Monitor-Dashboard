// UI module placeholders - to be implemented
pub mod dashboard;
pub mod events;
pub mod widgets;

// Placeholder structs to prevent compilation errors
pub struct Dashboard;
pub struct EventHandler;

impl Dashboard {
    pub fn new(_settings: crate::config::Settings) -> Self {
        Self
    }
    
    pub fn render(&mut self, _f: &mut ratatui::Frame, _monitor: &crate::system::SystemMonitor) {
        // Placeholder
    }
    
    pub fn handle_event(&mut self, _event: crossterm::event::Event) -> anyhow::Result<bool> {
        Ok(false)
    }
}

impl EventHandler {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn next_event(&mut self) -> Option<crossterm::event::Event> {
        None
    }
}