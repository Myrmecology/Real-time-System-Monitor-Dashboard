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
        // Poll for events with a timeout to prevent blocking
        if poll(Duration::from_millis(100)).unwrap_or(false) {
            event::read().ok()
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