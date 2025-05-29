use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, poll};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct EventHandler {
    last_key_time: Option<Instant>,
    key_debounce_ms: u64,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            last_key_time: None,
            key_debounce_ms: 150, // 150ms debounce for tab switching
        }
    }

    pub async fn next_event(&mut self) -> Option<Event> {
        // Use a longer timeout to prevent rapid key repeats
        if poll(Duration::from_millis(100)).unwrap_or(false) {
            match event::read() {
                Ok(event) => {
                    // Apply debouncing for certain keys
                    if let Event::Key(key_event) = &event {
                        if self.should_debounce_key(key_event) {
                            let now = Instant::now();
                            if let Some(last_time) = self.last_key_time {
                                if now.duration_since(last_time).as_millis() < self.key_debounce_ms as u128 {
                                    return None; // Ignore this key press (too soon)
                                }
                            }
                            self.last_key_time = Some(now);
                        }
                    }
                    Some(event)
                }
                Err(_) => None,
            }
        } else {
            None
        }
    }

    fn should_debounce_key(&self, key_event: &KeyEvent) -> bool {
        // Apply debouncing to tab navigation keys to prevent rapid switching
        matches!(key_event.code, 
            KeyCode::Tab | 
            KeyCode::BackTab
        )
    }
}

// Helper functions for handling specific events
pub fn should_quit(event: &Event) -> bool {
    matches!(event,
        Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
            ..
        }) | Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) | Event::Key(KeyEvent {
            code: KeyCode::Esc,
            modifiers: KeyModifiers::NONE,
            ..
        })
    )
}

pub fn handle_key_event(event: KeyEvent) -> Option<AppAction> {
    match (event.code, event.modifiers) {
        // Quit commands
        (KeyCode::Char('q'), KeyModifiers::NONE) => Some(AppAction::Quit),
        (KeyCode::Char('c'), KeyModifiers::CONTROL) => Some(AppAction::Quit),
        (KeyCode::Esc, KeyModifiers::NONE) => Some(AppAction::Quit),
        
        // Tab navigation - only on key press, not release
        (KeyCode::Tab, KeyModifiers::NONE) => Some(AppAction::NextTab),
        (KeyCode::BackTab, KeyModifiers::SHIFT) => Some(AppAction::PrevTab),
        
        // Alternative navigation with numbers
        (KeyCode::Char('1'), KeyModifiers::NONE) => Some(AppAction::GoToTab(0)),
        (KeyCode::Char('2'), KeyModifiers::NONE) => Some(AppAction::GoToTab(1)),
        (KeyCode::Char('3'), KeyModifiers::NONE) => Some(AppAction::GoToTab(2)),
        (KeyCode::Char('4'), KeyModifiers::NONE) => Some(AppAction::GoToTab(3)),
        
        // Arrow key navigation (no debouncing for smoother scrolling)
        (KeyCode::Up, KeyModifiers::NONE) => Some(AppAction::ScrollUp),
        (KeyCode::Down, KeyModifiers::NONE) => Some(AppAction::ScrollDown),
        
        // Other commands
        (KeyCode::Char('r'), KeyModifiers::NONE) => Some(AppAction::Refresh),
        (KeyCode::Char('h'), KeyModifiers::NONE) => Some(AppAction::Help),
        
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppAction {
    Quit,
    NextTab,
    PrevTab,
    GoToTab(usize),
    ScrollUp,
    ScrollDown,
    Refresh,
    Help,
}