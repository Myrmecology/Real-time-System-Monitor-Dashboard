pub mod config;
pub mod system;
pub mod ui;
pub mod utils;

pub use config::Settings;
pub use system::SystemMonitor;
pub use ui::{Dashboard, EventHandler};