# Real-time System Monitor Dashboard

A high-performance, real-time system monitoring dashboard built entirely in Rust. This terminal-based (TUI) application provides comprehensive system insights with beautiful visualizations, live charts, and interactive navigation.

For a video DEMO of this project, please visit: https://www.youtube.com/watch?v=hS2n7LYf040

## 🚀 Features

### **Real-time Monitoring**
- **CPU Usage**: Live monitoring with historical charts and multi-core detection
- **Memory Usage**: Real-time RAM and swap monitoring with trend visualization
- **Process Management**: Live process list sorted by CPU usage with memory details
- **Disk Usage**: Comprehensive disk space monitoring for all mounted drives
- **Network Statistics**: Interface-level network traffic and packet monitoring
- **System Information**: Uptime, load averages, and process counts

### **Interactive TUI Interface**
- **Multiple Tabs**: Overview, Processes, Network, and Help sections
- **Live Charts**: Historical CPU and memory usage graphs using Braille patterns
- **Color-coded Widgets**: Visual indicators with red/yellow/green status colors
- **Keyboard Navigation**: Full keyboard control with smooth tab switching
- **Responsive Design**: Adapts to different terminal sizes
- **Professional Styling**: Clean, modern terminal interface

### **Performance & Architecture**
- **100% Rust**: Memory-safe, high-performance system programming
- **Async Architecture**: Non-blocking event handling with Tokio
- **Cross-platform**: Works on Windows, macOS, and Linux
- **Low Resource Usage**: Minimal CPU and memory footprint
- **Configurable**: TOML-based configuration system

## 📋 Prerequisites

- **Rust 1.70+**: Install from [rustup.rs](https://rustup.rs/)
- **Terminal**: Any modern terminal with Unicode support
- **Operating System**: Windows 10+, macOS 10.12+, or Linux

## 🛠️ Installation & Setup

### **1. Clone the Repository**
```bash
git clone https://github.com/Myrmecology/Real-time-System-Monitor-Dashboard.git
cd Real-time-System-Monitor-Dashboard
```

### **2. Build the Project**
```bash
# Check that everything compiles
cargo check

# Build optimized release version
cargo build --release

# Or run directly in development mode
cargo run
```

### **3. Install Dependencies**
All dependencies are automatically handled by Cargo. Key dependencies include:
- `ratatui` - Terminal UI framework
- `sysinfo` - System information collection
- `tokio` - Async runtime
- `crossterm` - Cross-platform terminal manipulation
- `clap` - Command-line argument parsing

## 🎮 Usage

### **Running the Application**
```bash
# Run with default settings
cargo run

# Run with custom configuration
cargo run -- --config custom-config.toml

# Run with custom refresh rate (seconds)
cargo run -- --refresh 2

# Enable debug logging
cargo run -- --debug

# Show help
cargo run -- --help
```

### **Command Line Options**
```
USAGE:
    system-monitor [OPTIONS]

OPTIONS:
    -c, --config <CONFIG>    Configuration file path [default: config.toml]
    -r, --refresh <REFRESH>  Refresh interval in seconds [default: 1]
    -d, --debug             Enable debug logging
    -h, --help              Print help information
```

## ⌨️ Controls & Navigation

### **Tab Navigation**
- **Tab / Shift+Tab**: Cycle through tabs (with smooth 150ms delay)
- **1, 2, 3, 4**: Jump directly to Overview, Processes, Network, Help
- **Current tab**: Displayed in status bar

### **Process List (Processes Tab)**
- **↑ / ↓ Arrow Keys**: Scroll through process list
- **Processes**: Automatically sorted by CPU usage (highest first)

### **General Controls**
- **r**: Force refresh system data
- **h**: Jump to help screen
- **q / Esc / Ctrl+C**: Quit application

### **Tab Descriptions**
1. **Overview**: CPU/Memory gauges, historical charts, system info, disk usage
2. **Processes**: Live process list with CPU/memory usage, scrollable
3. **Network**: Network interface statistics and traffic data
4. **Help**: Comprehensive help and keyboard shortcuts

## 📊 Dashboard Sections

### **Overview Tab**
- **Top Row**: Real-time CPU and Memory usage gauges
- **Middle Row**: Historical charts showing CPU and Memory trends over time
- **Bottom Row**: System information (uptime, processes, load) and disk usage table

### **Processes Tab**
- **Summary Bar**: Quick CPU and Memory overview
- **Process List**: Scrollable list showing:
  - Process ID (PID)
  - CPU usage percentage
  - Memory usage in MB
  - Process name
  - Sorted by CPU usage (highest first)

### **Network Tab**
- **Summary Bar**: System overview widgets
- **Network Table**: Interface statistics showing:
  - Interface name
  - Bytes received/transmitted
  - Packets received/transmitted

## ⚙️ Configuration

### **Default Configuration File (config.toml)**
```toml
[dashboard]
title = "System Monitor Dashboard"
refresh_rate_ms = 1000
max_history_entries = 100

[system]
enable_process_monitoring = true
max_processes_displayed = 20
cpu_history_length = 60
memory_history_length = 60

[display]
show_cpu_graph = true
show_memory_graph = true
show_process_list = true
show_network_info = true
show_disk_info = true
```

### **Configuration Options**
- **refresh_rate_ms**: How often to update data (milliseconds)
- **max_history_entries**: Maximum data points for charts
- **cpu_history_length**: CPU chart history length
- **memory_history_length**: Memory chart history length
- **max_processes_displayed**: Processes to show per page

## 🏗️ Project Structure

```
Real-time-System-Monitor-Dashboard/
├── Cargo.toml                 # Project dependencies and metadata
├── config.toml               # Default configuration file
├── README.md                 # This file
├── .gitignore               # Git ignore rules
└── src/
    ├── main.rs              # Application entry point
    ├── lib.rs               # Library exports
    ├── config/
    │   ├── mod.rs           # Configuration module
    │   └── settings.rs      # Settings management
    ├── system/
    │   ├── mod.rs           # System monitoring module
    │   ├── monitor.rs       # Core system monitoring logic
    │   └── processes.rs     # Process management
    ├── ui/
    │   ├── mod.rs           # UI module exports
    │   ├── dashboard.rs     # Main dashboard and layouts
    │   ├── events.rs        # Event handling and key processing
    │   └── widgets.rs       # Custom TUI widgets
    └── utils/
        ├── mod.rs           # Utility module
        └── helpers.rs       # Helper functions
```

## 🔧 Technical Details

### **Architecture**
- **Async Event Loop**: Non-blocking UI updates with system monitoring
- **Modular Design**: Clean separation of concerns across modules  
- **Memory Management**: Efficient data structures with bounded history
- **Error Handling**: Comprehensive error handling with `anyhow`

### **Performance**
- **Minimal CPU Usage**: Efficient system data collection
- **Memory Efficient**: Bounded data structures prevent memory leaks
- **Responsive UI**: 60+ FPS rendering capability
- **Cross-platform**: Native performance on all supported platforms

### **Dependencies**
```toml
[dependencies]
ratatui = "0.26"              # Terminal UI framework
crossterm = "0.27"            # Cross-platform terminal control
sysinfo = "0.30"              # System information collection
tokio = "1.0"                 # Async runtime
clap = "4.4"                  # Command-line parsing
serde = "1.0"                 # Serialization framework
toml = "0.8"                  # Configuration file parsing
chrono = "0.4"                # Date and time utilities
anyhow = "1.0"                # Error handling
log = "0.4"                   # Logging framework
env_logger = "0.10"           # Environment-based logging
```

## 📈 Data Collection

### **System Metrics**
- **CPU**: Usage percentage, frequency, core count
- **Memory**: Used/total RAM, swap usage, usage percentages  
- **Processes**: PID, name, CPU usage, memory usage
- **Disks**: Mount points, file systems, space usage
- **Network**: Interface names, bytes/packets transmitted/received
- **System**: Uptime, boot time, load averages, process count

### **Chart Data**
- **Historical Data**: Last 60 data points by default
- **Update Frequency**: 1-second intervals (configurable)
- **Chart Types**: Line charts with Braille character rendering
- **Data Retention**: Automatic cleanup of old data points

## 🚨 Important Notes

### **Chart Initialization**
- **Memory charts need ~30 seconds** to build meaningful historical data
- **CPU charts start immediately** but become more useful over time
- **First run**: Charts may appear empty initially - this is normal

### **Process List**
- **Sorted by CPU usage** (highest first)
- **Real-time updates** every second
- **Memory shown in MB** for readability
- **Scrolling available** with arrow keys

### **Platform Differences**
- **Windows**: Shows many `.exe` processes (normal system behavior)
- **macOS/Linux**: Different process naming conventions
- **Network interfaces**: Names vary by platform

## 🐛 Troubleshooting

### **Common Issues**
1. **Charts not showing data**: Wait 30+ seconds for history to build
2. **High CPU usage**: Increase refresh interval with `--refresh 2`
3. **Terminal rendering issues**: Ensure Unicode support in terminal
4. **Permission errors**: Some system info may require elevated privileges

### **Performance Tips**
- **Increase refresh interval** for lower resource usage
- **Resize terminal** if widgets appear cramped
- **Use release build** (`cargo build --release`) for better performance

## 🤝 Contributing

This project demonstrates modern Rust development practices:
- **Safe Systems Programming**: Memory-safe system monitoring
- **Async Programming**: Non-blocking concurrent operations
- **TUI Development**: Advanced terminal user interface design
- **Error Handling**: Robust error management strategies
- **Project Organization**: Clean, modular architecture

## 📝 License

I used a MIT License for this project 

## 🎯 Learning Outcomes

This project showcases:
- **Advanced Rust Programming**: Ownership, lifetimes, async/await
- **Systems Programming**: Direct OS interaction and resource monitoring
- **User Interface Design**: Professional terminal-based interfaces
- **Software Architecture**: Modular, maintainable code organization
- **Performance Optimization**: Efficient real-time data processing
- **Cross-platform Development**: Consistent behavior across operating systems

---

**Built with ❤️ in Rust** 🦀

Happy coding everyone and best of luck with future projects 