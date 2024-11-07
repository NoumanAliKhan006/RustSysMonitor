# RustSysMonitor
**RustSysMonitor** is a real-time system performance monitoring tool written in Rust. It provides an overview of system metrics, including CPU usage, memory usage, disk space, and network traffic. Designed for efficiency, RustSysMonitor outputs formatted data to both the console and a log file, making it ideal for continuous performance tracking.

# Features

   **CPU Usage Monitoring:** Provides real-time CPU utilization.
   
   **Memory Tracking:** Displays used and available memory.
   
   **Disk Usage:** Monitors disk space usage for each partition.
   
   **Network Traffic Analysis:** Tracks sent and received bytes across network interfaces.
   
   **Logging:** Logs all monitored data with timestamps to system_monitor_log.txt for historical analysis.
   

# Table of Contents

   - Getting Started
   - Usage
   - Example Output
   - Configuration
   - Dependencies

# Getting Started

To get started with RustSysMonitor, follow the instructions below to set up and run the tool.

## Prerequisites

   - Rust (version 1.53+)
   - Cargo

## Installation

1. Clone the repository:

    git clone https://github.com/yourusername/RustSysMonitor.git

    cd RustSysMonitor

1. Build the project:

    cargo build --release

1. Run the executable:

    ./target/release/RustSysMonitor

# Usage

The tool will automatically start monitoring and displaying system metrics every second. All data is logged to system_monitor_log.txt in the project directory.
Example Output
```
================ System Performance Metrics ================
CPU Usage        : 15.2%
Used Memory      : 123456 KB
Available Memory : 654321 KB
Disk Usage       :
  /dev/sda1: 204800 KB used
Network Usage    :
  eth0 - Sent: 20480 bytes, Received: 40960 bytes
============================================================
```

# Configuration

You can modify the refresh interval by adjusting the refresh_interval variable in main.rs:

let refresh_interval = time::Duration::from_secs(1); // Set to preferred interval

# Dependencies

RustSysMonitor relies on the following crates:

    sysinfo: Provides system information (CPU, memory, disk, and network data).
    chrono: For timestamping log entries.

Install dependencies with Cargo:

cargo build
