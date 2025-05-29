use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};
use crossterm::event::Event;
use anyhow::Result;

use crate::config::Settings;
use crate::system::SystemMonitor;
use super::events::{handle_key_event, should_quit, AppAction};
use super::widgets::{CpuWidget, MemoryWidget, SystemInfoWidget, DiskWidget, ProcessWidget, NetworkWidget};

#[derive(Debug, Clone, PartialEq)]
pub enum TabIndex {
    Overview = 0,
    Processes = 1,
    Network = 2,
    Help = 3,
}

impl From<usize> for TabIndex {
    fn from(index: usize) -> Self {
        match index {
            0 => TabIndex::Overview,
            1 => TabIndex::Processes,
            2 => TabIndex::Network,
            3 => TabIndex::Help,
            _ => TabIndex::Overview,
        }
    }
}

pub struct Dashboard {
    settings: Settings,
    current_tab: TabIndex,
    process_scroll_offset: usize,
}

impl Dashboard {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            current_tab: TabIndex::Overview,
            process_scroll_offset: 0,
        }
    }

    pub fn render(&mut self, f: &mut Frame, monitor: &SystemMonitor) {
        let size = f.size();

        // Create main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title and tabs
                Constraint::Min(0),    // Main content
                Constraint::Length(1), // Status bar
            ])
            .split(size);

        // Render title and tabs
        self.render_tabs(f, chunks[0]);

        // Render main content based on current tab
        match self.current_tab {
            TabIndex::Overview => self.render_overview(f, chunks[1], monitor),
            TabIndex::Processes => self.render_processes(f, chunks[1], monitor),
            TabIndex::Network => self.render_network(f, chunks[1], monitor),
            TabIndex::Help => self.render_help(f, chunks[1]),
        }

        // Render status bar
        self.render_status_bar(f, chunks[2]);
    }

    fn render_tabs(&self, f: &mut Frame, area: Rect) {
        let tab_titles = vec![
            "Overview",
            "Processes", 
            "Network",
            "Help"
        ];

        let tabs = Tabs::new(tab_titles)
            .block(
                Block::default()
                    .title(format!(" {} ", self.settings.dashboard.title))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::White)),
            )
            .style(Style::default().fg(Color::Gray))
            .highlight_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .select(self.current_tab.clone() as usize);

        f.render_widget(tabs, area);
    }

    fn render_overview(&self, f: &mut Frame, area: Rect, monitor: &SystemMonitor) {
        // Create layout for overview
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6),  // CPU and Memory gauges
                Constraint::Min(10),    // Charts
                Constraint::Length(8),  // System info and disk
            ])
            .split(area);

        // Top row: CPU and Memory gauges
        let gauge_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_chunks[0]);

        CpuWidget::render(monitor, gauge_chunks[0], f.buffer_mut());
        MemoryWidget::render(monitor, gauge_chunks[1], f.buffer_mut());

        // Middle row: CPU and Memory history charts
        let chart_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_chunks[1]);

        CpuWidget::render_history_chart(monitor, chart_chunks[0], f.buffer_mut());
        MemoryWidget::render_history_chart(monitor, chart_chunks[1], f.buffer_mut());

        // Bottom row: System info and disk usage
        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(main_chunks[2]);

        SystemInfoWidget::render(monitor, bottom_chunks[0], f.buffer_mut());
        DiskWidget::render(monitor, bottom_chunks[1], f.buffer_mut());
    }

    fn render_processes(&self, f: &mut Frame, area: Rect, monitor: &SystemMonitor) {
        // Create layout for processes
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5),  // CPU and Memory summary
                Constraint::Min(0),     // Process list
            ])
            .split(area);

        // Top: Quick system summary
        let summary_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[0]);

        CpuWidget::render(monitor, summary_chunks[0], f.buffer_mut());
        MemoryWidget::render(monitor, summary_chunks[1], f.buffer_mut());

        // Bottom: Process list
        ProcessWidget::render(monitor, chunks[1], f.buffer_mut(), self.process_scroll_offset);
    }

    fn render_network(&self, f: &mut Frame, area: Rect, monitor: &SystemMonitor) {
        // Create layout for network
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5),  // System summary
                Constraint::Min(0),     // Network info
            ])
            .split(area);

        // Top: System summary
        let summary_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33), 
                Constraint::Percentage(33), 
                Constraint::Percentage(34)
            ])
            .split(chunks[0]);

        CpuWidget::render(monitor, summary_chunks[0], f.buffer_mut());
        MemoryWidget::render(monitor, summary_chunks[1], f.buffer_mut());
        SystemInfoWidget::render(monitor, summary_chunks[2], f.buffer_mut());

        // Bottom: Network information
        NetworkWidget::render(monitor, chunks[1], f.buffer_mut());
    }

    fn render_help(&self, f: &mut Frame, area: Rect) {
        let help_text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("System Monitor Dashboard", 
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Navigation:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("  Tab / Shift+Tab", Style::default().fg(Color::Green)),
                Span::raw("  - Switch between tabs"),
            ]),
            Line::from(vec![
                Span::styled("  ↑ / ↓", Style::default().fg(Color::Green)),
                Span::raw("           - Scroll process list"),
            ]),
            Line::from(vec![
                Span::styled("  r", Style::default().fg(Color::Green)),
                Span::raw("               - Force refresh"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Tabs:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("  Overview", Style::default().fg(Color::Green)),
                Span::raw("        - CPU, Memory, Disk usage with charts"),
            ]),
            Line::from(vec![
                Span::styled("  Processes", Style::default().fg(Color::Green)),
                Span::raw("       - Running processes sorted by CPU"),
            ]),
            Line::from(vec![
                Span::styled("  Network", Style::default().fg(Color::Green)),
                Span::raw("         - Network interface statistics"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Exit:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("  q / Esc / Ctrl+C", Style::default().fg(Color::Red)),
                Span::raw("  - Quit application"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Built with ❤️  in Rust", 
                    Style::default().fg(Color::Magenta)),
            ]),
        ];

        let help_paragraph = Paragraph::new(help_text)
            .block(
                Block::default()
                    .title(" Help ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::White)),
            );

        f.render_widget(help_paragraph, area);
    }

    fn render_status_bar(&self, f: &mut Frame, area: Rect) {
        let status_text = match self.current_tab {
            TabIndex::Overview => "Tab: Switch tabs | r: Refresh | q: Quit",
            TabIndex::Processes => "↑↓: Scroll | Tab: Switch tabs | r: Refresh | q: Quit",
            TabIndex::Network => "Tab: Switch tabs | r: Refresh | q: Quit",
            TabIndex::Help => "Tab: Switch tabs | q: Quit",
        };

        let status = Paragraph::new(status_text)
            .style(Style::default().fg(Color::Gray));

        f.render_widget(status, area);
    }

    pub fn handle_event(&mut self, event: Event) -> Result<bool> {
        if should_quit(&event) {
            return Ok(true); // Signal to quit
        }

        if let Event::Key(key_event) = event {
            if let Some(action) = handle_key_event(key_event) {
                match action {
                    AppAction::Quit => return Ok(true),
                    AppAction::NextTab => self.next_tab(),
                    AppAction::PrevTab => self.prev_tab(),
                    AppAction::ScrollUp => self.scroll_up(),
                    AppAction::ScrollDown => self.scroll_down(),
                    AppAction::Refresh => {
                        // Refresh will be handled by the main loop
                    }
                    AppAction::Help => {
                        self.current_tab = TabIndex::Help;
                    }
                }
            }
        }

        Ok(false) // Continue running
    }

    fn next_tab(&mut self) {
        let current = self.current_tab.clone() as usize;
        let next = (current + 1) % 4; // We have 4 tabs
        self.current_tab = TabIndex::from(next);
        self.process_scroll_offset = 0; // Reset scroll when switching tabs
    }

    fn prev_tab(&mut self) {
        let current = self.current_tab.clone() as usize;
        let prev = if current == 0 { 3 } else { current - 1 };
        self.current_tab = TabIndex::from(prev);
        self.process_scroll_offset = 0; // Reset scroll when switching tabs
    }

    fn scroll_up(&mut self) {
        if self.current_tab == TabIndex::Processes {
            self.process_scroll_offset = self.process_scroll_offset.saturating_sub(1);
        }
    }

    fn scroll_down(&mut self) {
        if self.current_tab == TabIndex::Processes {
            self.process_scroll_offset += 1;
        }
    }
}