use std::collections::VecDeque;
use sysinfo::{System, SystemExt, CpuExt, DiskExt, NetworkExt, NetworksExt, ProcessExt};
use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub struct CpuData {
    pub timestamp: DateTime<Local>,
    pub usage: f32,
    pub frequency: u64,
}

#[derive(Debug, Clone)]
pub struct MemoryData {
    pub timestamp: DateTime<Local>,
    pub used: u64,
    pub total: u64,
    pub usage_percent: f32,
}

#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total_space: u64,
    pub available_space: u64,
    pub used_space: u64,
    pub usage_percent: f32,
    pub file_system: String,
}

#[derive(Debug, Clone)]
pub struct NetworkInfo {
    pub interface: String,
    pub bytes_received: u64,
    pub bytes_transmitted: u64,
    pub packets_received: u64,
    pub packets_transmitted: u64,
}

#[derive(Debug)]
pub struct SystemMonitor {
    system: System,
    cpu_history: VecDeque<CpuData>,
    memory_history: VecDeque<MemoryData>,
    max_history: usize,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        
        Self {
            system,
            cpu_history: VecDeque::new(),
            memory_history: VecDeque::new(),
            max_history: 60, // Keep 60 data points by default
        }
    }

    pub fn refresh_all(&mut self) {
        self.system.refresh_all();
        self.update_cpu_history();
        self.update_memory_history();
    }

    pub fn refresh_cpu(&mut self) {
        self.system.refresh_cpu();
        self.update_cpu_history();
    }

    pub fn refresh_memory(&mut self) {
        self.system.refresh_memory();
        self.update_memory_history();
    }

    fn update_cpu_history(&mut self) {
        let global_cpu = self.system.global_cpu_info();
        let cpu_data = CpuData {
            timestamp: Local::now(),
            usage: global_cpu.cpu_usage(),
            frequency: global_cpu.frequency(),
        };

        self.cpu_history.push_back(cpu_data);
        if self.cpu_history.len() > self.max_history {
            self.cpu_history.pop_front();
        }
    }

    fn update_memory_history(&mut self) {
        let used = self.system.used_memory();
        let total = self.system.total_memory();
        let usage_percent = if total > 0 {
            (used as f32 / total as f32) * 100.0
        } else {
            0.0
        };

        let memory_data = MemoryData {
            timestamp: Local::now(),
            used,
            total,
            usage_percent,
        };

        self.memory_history.push_back(memory_data);
        if self.memory_history.len() > self.max_history {
            self.memory_history.pop_front();
        }
    }

    // Getters for system information
    pub fn cpu_usage(&self) -> f32 {
        self.system.global_cpu_info().cpu_usage()
    }

    pub fn cpu_count(&self) -> usize {
        self.system.cpus().len()
    }

    pub fn cpu_history(&self) -> &VecDeque<CpuData> {
        &self.cpu_history
    }

    pub fn memory_used(&self) -> u64 {
        self.system.used_memory()
    }

    pub fn memory_total(&self) -> u64 {
        self.system.total_memory()
    }

    pub fn memory_usage_percent(&self) -> f32 {
        let used = self.system.used_memory();
        let total = self.system.total_memory();
        if total > 0 {
            (used as f32 / total as f32) * 100.0
        } else {
            0.0
        }
    }

    pub fn memory_history(&self) -> &VecDeque<MemoryData> {
        &self.memory_history
    }

    pub fn swap_used(&self) -> u64 {
        self.system.used_swap()
    }

    pub fn swap_total(&self) -> u64 {
        self.system.total_swap()
    }

    pub fn uptime(&self) -> u64 {
        self.system.uptime()
    }

    pub fn boot_time(&self) -> u64 {
        self.system.boot_time()
    }

    pub fn load_average(&self) -> sysinfo::LoadAvg {
        self.system.load_average()
    }

    pub fn disk_info(&self) -> Vec<DiskInfo> {
        self.system
            .disks()
            .iter()
            .map(|disk| {
                let total = disk.total_space();
                let available = disk.available_space();
                let used = total - available;
                let usage_percent = if total > 0 {
                    (used as f32 / total as f32) * 100.0
                } else {
                    0.0
                };

                DiskInfo {
                    name: disk.name().to_string_lossy().to_string(),
                    mount_point: disk.mount_point().to_string_lossy().to_string(),
                    total_space: total,
                    available_space: available,
                    used_space: used,
                    usage_percent,
                    file_system: String::from_utf8_lossy(disk.file_system()).to_string(),
                }
            })
            .collect()
    }

    pub fn network_info(&self) -> Vec<NetworkInfo> {
        self.system
            .networks()
            .iter()
            .map(|(interface, data)| NetworkInfo {
                interface: interface.clone(),
                bytes_received: data.received(),
                bytes_transmitted: data.transmitted(),
                packets_received: data.packets_received(),
                packets_transmitted: data.packets_transmitted(),
            })
            .collect()
    }

    pub fn process_count(&self) -> usize {
        self.system.processes().len()
    }

    pub fn system(&self) -> &System {
        &self.system
    }

    pub fn set_max_history(&mut self, max: usize) {
        self.max_history = max;
        // Trim existing history if needed
        while self.cpu_history.len() > max {
            self.cpu_history.pop_front();
        }
        while self.memory_history.len() > max {
            self.memory_history.pop_front();
        }
    }
}