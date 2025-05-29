use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::info;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;
use tokio::time::{interval, Duration};

mod config;
mod system;
mod ui;
mod utils;

use config::Settings;
use system::SystemMonitor;
use ui::{Dashboard, EventHandler};

#[derive(Parser)]
#[command(name = "system-monitor")]
#[command(about = "Real-time System Monitor Dashboard")]
struct Cli {
    /// Configuration file path
    #[arg(short, long, default_value = "config.toml")]
    config: String,
    
    /// Refresh interval in seconds
    #[arg(short, long, default_value = "1")]
    refresh: u64,
    
    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    if cli.debug {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();
    }
    
    info!("Starting System Monitor Dashboard");
    
    // Load configuration
    let settings = Settings::load(&cli.config)?;
    info!("Configuration loaded from: {}", cli.config);
    
    // Initialize system monitor
    let mut system_monitor = SystemMonitor::new();
    system_monitor.refresh_all();
    
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    // Initialize dashboard and event handler
    let mut dashboard = Dashboard::new(settings.clone());
    let mut event_handler = EventHandler::new();
    
    // Create refresh interval
    let mut refresh_interval = interval(Duration::from_secs(cli.refresh));
    
    // Main application loop
    let result = run_app(
        &mut terminal,
        &mut dashboard,
        &mut event_handler,
        &mut system_monitor,
        &mut refresh_interval,
    ).await;
    
    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    
    if let Err(err) = result {
        eprintln!("Error: {}", err);
    }
    
    info!("System Monitor Dashboard shutdown complete");
    Ok(())
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    dashboard: &mut Dashboard,
    event_handler: &mut EventHandler,
    system_monitor: &mut SystemMonitor,
    refresh_interval: &mut tokio::time::Interval,
) -> Result<()> {
    loop {
        // Draw the UI
        terminal.draw(|f| dashboard.render(f, system_monitor))?;
        
        tokio::select! {
            // Handle user input events
            event = event_handler.next_event() => {
                if let Some(event) = event {
                    if dashboard.handle_event(event)? {
                        break; // Exit requested
                    }
                }
            }
            
            // Refresh system data
            _ = refresh_interval.tick() => {
                system_monitor.refresh_all();
            }
        }
    }
    
    Ok(())
}