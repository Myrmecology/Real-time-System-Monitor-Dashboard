use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, poll};
use std::time::Duration;

#[derive(Debug)]
pub struct EventHandler {
    // We can add more sophisticated event handling later
}

impl EventHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn next_event(&mut self) -> Option<Event> {
        // Use a very short timeout to make it more responsive
        if poll(Duration::from_millis(50)).unwrap_or(false) {
            match event::read() {
                Ok(event) => Some(event),
                Err(_) => None,
            }
        } else {
            None
        }
    }
}

// Helper functions for handling specific events
pub fn should_quit(event: &Event) -> bool {
    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
            ..
        }) => true,
        Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => true,
        Event::Key(KeyEvent {
            code: KeyCode::Esc,
            modifiers: KeyModifiers::NONE,
            ..
        }) => true,
        _ => false,
    }
}

pub fn handle_key_event(event: KeyEvent) -> Option<AppAction> {
    match event {
        // Quit commands
        KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
            ..
        } => Some(AppAction::Quit),
        KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => Some(AppAction::Quit),
        KeyEvent {
            code: KeyCode::Esc,
            modifiers: KeyModifiers::NONE,
            ..
        } => Some(AppAction::Quit),
        
        // Tab navigation
        KeyEvent {
            code: KeyCode::Tab,
            modifiers: KeyModifiers::NONE,
            ..
        } => Some(AppAction::NextTab),
        KeyEvent {
            code: KeyCode::BackTab,
            modifiers: KeyModifiers::SHIFT,
            ..
        } => Some(AppAction::PrevTab),
        
        // Arrow key navigation - more explicit matching
        KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
            kind: crossterm::event::KeyEventKind::Press,
            ..
        } => Some(AppAction::ScrollUp),
        KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
            kind: crossterm::event::KeyEventKind::Press,
            ..
        } => Some(AppAction::ScrollDown),
        
        // Also handle arrow keys without explicit kind matching (fallback)
        KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
            ..
        } => Some(AppAction::ScrollUp),
        KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
            ..
        } => Some(AppAction::ScrollDown),
        
        // Other commands
        KeyEvent {
            code: KeyCode::Char('r'),
            modifiers: KeyModifiers::NONE,
            ..
        } => Some(AppAction::Refresh),
        KeyEvent {
            code: KeyCode::Char('h'),
            modifiers: KeyModifiers::NONE,
            ..
        } => Some(AppAction::Help),
        
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppAction {
    Quit,
    NextTab,
    PrevTab,
    ScrollUp,
    ScrollDown,
    Refresh,
    Help,
}