use std::{thread, time, fs::OpenOptions, io::Write};
use sysinfo::{System, SystemExt, CpuExt, DiskExt, NetworkExt, NetworksExt};  // Add NetworksExt
use chrono::Local;

fn main() {
    let refresh_interval = time::Duration::from_secs(1);
    let mut sys = System::new_all();

    // Set up optional logging
    let log_file_path = "system_monitor_log.txt";
    let mut log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_file_path)
        .expect("Could not open log file");

    loop {
        sys.refresh_all();

        // Retrieve metrics
        let cpu_usage = sys.global_cpu_info().cpu_usage();
        let used_memory = sys.used_memory();
        let available_memory = sys.available_memory();

        // Disk usage
        let disk_usage: Vec<String> = sys.disks().iter()
            .map(|disk| format!("{}: {} KB used", disk.name().to_string_lossy(), disk.available_space()))
            .collect();

        // Network usage
        let network_usage: Vec<String> = sys.networks().iter()
            .map(|(name, data)| format!("{} - Sent: {} bytes, Received: {} bytes", name, data.transmitted(), data.received()))
            .collect();

        // Display formatted output
        println!("================ System Performance Metrics ================");
        println!("CPU Usage        : {:.2}%", cpu_usage);
        println!("Used Memory      : {} KB", used_memory);
        println!("Available Memory : {} KB", available_memory);
        println!("Disk Usage       :");
        for disk in &disk_usage {
            println!("  {}", disk);
        }
        println!("Network Usage    :");
        for network in &network_usage {
            println!("  {}", network);
        }
        println!("============================================================");

        // Logging data to file
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        writeln!(
            log_file,
            "{}, CPU Usage: {:.2}%, Used Memory: {} KB, Available Memory: {} KB, Disk: {:?}, Network: {:?}",
            timestamp, cpu_usage, used_memory, available_memory, disk_usage, network_usage
        ).expect("Could not write to log file");

        // Sleep before refreshing
        thread::sleep(refresh_interval);
    }
}
