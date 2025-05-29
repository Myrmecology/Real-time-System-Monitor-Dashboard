use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span, Text},
    widgets::{
        Axis, Block, Borders, Chart, Dataset, Gauge, List, ListItem, Paragraph, Row, 
        Sparkline, Table, Widget, Wrap,
    },
};
use std::collections::VecDeque;
use crate::system::{SystemMonitor, CpuData, MemoryData, DiskInfo, NetworkInfo};

pub struct CpuWidget;

impl CpuWidget {
    pub fn render(monitor: &SystemMonitor, area: Rect, buf: &mut Buffer) {
        let cpu_usage = monitor.cpu_usage();
        let cpu_count = monitor.cpu_count();
        
        // Create CPU usage gauge
        let gauge = Gauge::default()
            .block(
                Block::default()
                    .title(format!(" CPU Usage ({} cores) ", cpu_count))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .gauge_style(
                Style::default()
                    .fg(if cpu_usage > 80.0 {
                        Color::Red
                    } else if cpu_usage > 60.0 {
                        Color::Yellow
                    } else {
                        Color::Green
                    })
                    .add_modifier(Modifier::BOLD),
            )
            .percent(cpu_usage as u16)
            .label(format!("{:.1}%", cpu_usage));

        gauge.render(area, buf);
    }

    pub fn render_history_chart(monitor: &SystemMonitor, area: Rect, buf: &mut Buffer) {
        let history = monitor.cpu_history();
        
        if history.is_empty() {
            return;
        }

        // Convert history to chart data points
        let data: Vec<(f64, f64)> = history
            .iter()
            .enumerate()
            .map(|(i, cpu_data)| (i as f64, cpu_data.usage as f64))
            .collect();

        let dataset = Dataset::default()
            .name("CPU %")
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Cyan))
            .data(&data);

        let chart = Chart::new(vec![dataset])
            .block(
                Block::default()
                    .title(" CPU History ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .x_axis(
                Axis::default()
                    .title("Time")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, data.len().max(1) as f64]),
            )
            .y_axis(
                Axis::default()
                    .title("Usage %")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 100.0]),
            );

        chart.render(area, buf);
    }
}

pub struct MemoryWidget;

impl MemoryWidget {
    pub fn render(monitor: &SystemMonitor, area: Rect, buf: &mut Buffer) {
        let used = monitor.memory_used();
        let total = monitor.memory_total();
        let usage_percent = monitor.memory_usage_percent();
        
        let used_gb = used as f64 / 1_073_741_824.0; // Convert bytes to GB
        let total_gb = total as f64 / 1_073_741_824.0;

        let gauge = Gauge::default()
            .block(
                Block::default()
                    .title(" Memory Usage ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Magenta)),
            )
            .gauge_style(
                Style::default()
                    .fg(if usage_percent > 90.0 {
                        Color::Red
                    } else if usage_percent > 75.0 {
                        Color::Yellow
                    } else {
                        Color::Green
                    })
                    .add_modifier(Modifier::BOLD),
            )
            .percent(usage_percent as u16)
            .label(format!("{:.1}% ({:.1}/{:.1} GB)", usage_percent, used_gb, total_gb));

        gauge.render(area, buf);
    }

    pub fn render_history_chart(monitor: &SystemMonitor, area: Rect, buf: &mut Buffer) {
        let history = monitor.memory_history();
        
        if history.is_empty() {
            return;
        }

        let data: Vec<(f64, f64)> = history
            .iter()
            .enumerate()
            .map(|(i, mem_data)| (i as f64, mem_data.usage_percent as f64))
            .collect();

        let dataset = Dataset::default()
            .name("Memory %")
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Magenta))
            .data(&data);

        let chart = Chart::new(vec![dataset])
            .block(
                Block::default()
                    .title(" Memory History ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Magenta)),
            )
            .x_axis(
                Axis::default()
                    .title("Time")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, data.len().max(1) as f64]),
            )
            .y_axis(
                Axis::default()
                    .title("Usage %")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 100.0]),
            );

        chart.render(area, buf);
    }
}

pub struct SystemInfoWidget;

impl SystemInfoWidget {
    pub fn render(monitor: &SystemMonitor, area: Rect, buf: &mut Buffer) {
        let uptime = monitor.uptime();
        let process_count = monitor.process_count();
        let load_avg = monitor.load_average();
        
        // Convert uptime to human readable format
        let uptime_days = uptime / 86400;
        let uptime_hours = (uptime % 86400) / 3600;
        let uptime_minutes = (uptime % 3600) / 60;
        
        let uptime_str = if uptime_days > 0 {
            format!("{}d {}h {}m", uptime_days, uptime_hours, uptime_minutes)
        } else if uptime_hours > 0 {
            format!("{}h {}m", uptime_hours, uptime_minutes)
        } else {
            format!("{}m", uptime_minutes)
        };

        let info_text = vec![
            Line::from(vec![
                Span::styled("Uptime: ", Style::default().fg(Color::Cyan)),
                Span::raw(uptime_str),
            ]),
            Line::from(vec![
                Span::styled("Processes: ", Style::default().fg(Color::Green)),
                Span::raw(process_count.to_string()),
            ]),
            Line::from(vec![
                Span::styled("Load Avg: ", Style::default().fg(Color::Yellow)),
                Span::raw(format!("{:.2} {:.2} {:.2}", 
                    load_avg.one, load_avg.five, load_avg.fifteen)),
            ]),
        ];

        let paragraph = Paragraph::new(info_text)
            .block(
                Block::default()
                    .title(" System Info ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::White)),
            )
            .wrap(Wrap { trim: true });

        paragraph.render(area, buf);
    }
}

pub struct DiskWidget;

impl DiskWidget {
    pub fn render(monitor: &SystemMonitor, area: Rect, buf: &mut Buffer) {
        let disks = monitor.disk_info();
        
        if disks.is_empty() {
            let empty_text = Paragraph::new("No disk information available")
                .block(
                    Block::default()
                        .title(" Disk Usage ")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Yellow)),
                );
            empty_text.render(area, buf);
            return;
        }

        let rows: Vec<Row> = disks
            .iter()
            .map(|disk| {
                let used_gb = disk.used_space as f64 / 1_073_741_824.0;
                let total_gb = disk.total_space as f64 / 1_073_741_824.0;
                
                Row::new(vec![
                    disk.mount_point.clone(),
                    disk.file_system.clone(),
                    format!("{:.1} GB", total_gb),
                    format!("{:.1} GB", used_gb),
                    format!("{:.1}%", disk.usage_percent),
                ])
            })
            .collect();

        let table = Table::new(
            rows,
            &[
                Constraint::Length(15), // Mount point
                Constraint::Length(10), // File system
                Constraint::Length(10), // Total
                Constraint::Length(10), // Used
                Constraint::Length(8),  // Usage %
            ],
        )
        .header(
            Row::new(vec!["Mount", "FS", "Total", "Used", "Usage"])
                .style(Style::default().add_modifier(Modifier::BOLD))
                .bottom_margin(1),
        )
        .block(
            Block::default()
                .title(" Disk Usage ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        );

        table.render(area, buf);
    }
}

pub struct ProcessWidget;

impl ProcessWidget {
    pub fn render(monitor: &SystemMonitor, area: Rect, buf: &mut Buffer, scroll_offset: usize) {
        let mut processes: Vec<_> = monitor.system().processes().iter().collect();
        
        // Sort by CPU usage (descending)
        processes.sort_by(|a, b| b.1.cpu_usage().partial_cmp(&a.1.cpu_usage()).unwrap_or(std::cmp::Ordering::Equal));
        
        let items: Vec<ListItem> = processes
            .iter()
            .skip(scroll_offset)
            .take(area.height.saturating_sub(2) as usize) // Account for border
            .map(|(pid, process)| {
                let memory_mb = process.memory() as f64 / 1_048_576.0; // Convert to MB
                
                ListItem::new(Line::from(vec![
                    Span::styled(format!("{:>8}", pid), Style::default().fg(Color::Cyan)),
                    Span::raw("  "),
                    Span::styled(format!("{:>6.1}%", process.cpu_usage()), 
                        Style::default().fg(Color::Green)),
                    Span::raw("  "),
                    Span::styled(format!("{:>8.1}M", memory_mb), 
                        Style::default().fg(Color::Yellow)),
                    Span::raw("  "),
                    Span::raw(process.name()),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title(format!(" Processes ({}) ", processes.len()))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Green)),
            );

        list.render(area, buf);
    }
}

pub struct NetworkWidget;

impl NetworkWidget {
    pub fn render(monitor: &SystemMonitor, area: Rect, buf: &mut Buffer) {
        let networks = monitor.network_info();
        
        if networks.is_empty() {
            let empty_text = Paragraph::new("No network information available")
                .block(
                    Block::default()
                        .title(" Network Info ")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Blue)),
                );
            empty_text.render(area, buf);
            return;
        }

        let rows: Vec<Row> = networks
            .iter()
            .map(|net| {
                let rx_mb = net.bytes_received as f64 / 1_048_576.0;
                let tx_mb = net.bytes_transmitted as f64 / 1_048_576.0;
                
                Row::new(vec![
                    net.interface.clone(),
                    format!("{:.1} MB", rx_mb),
                    format!("{:.1} MB", tx_mb),
                    net.packets_received.to_string(),
                    net.packets_transmitted.to_string(),
                ])
            })
            .collect();

        let table = Table::new(
            rows,
            &[
                Constraint::Length(12), // Interface
                Constraint::Length(12), // RX bytes
                Constraint::Length(12), // TX bytes
                Constraint::Length(10), // RX packets
                Constraint::Length(10), // TX packets
            ],
        )
        .header(
            Row::new(vec!["Interface", "RX Bytes", "TX Bytes", "RX Pkts", "TX Pkts"])
                .style(Style::default().add_modifier(Modifier::BOLD))
                .bottom_margin(1),
        )
        .block(
            Block::default()
                .title(" Network Info ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue)),
        );

        table.render(area, buf);
    }
}