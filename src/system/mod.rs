pub mod monitor;
pub mod processes;

pub use monitor::{SystemMonitor, CpuData, MemoryData, DiskInfo, NetworkInfo};
// pub use processes::{ProcessInfo, ProcessManager}; // Will uncomment when we create these