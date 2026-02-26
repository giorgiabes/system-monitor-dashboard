use serde::Serialize;
use std::sync::Mutex;
use sysinfo::System;
use tauri::State;

// --- Data Structures ---
// These get serialized to JSON and sent to JS automatically

#[derive(Serialize)]
pub struct CpuInfo {
    pub usage: f32,
    pub core_count: usize,
}

#[derive(Serialize)]
pub struct MemoryInfo {
    pub used_mb: u64,
    pub total_mb: u64,
    pub percent: f32,
}

#[derive(Serialize)]
pub struct ProcessInfo {
    pub name: String,
    pub cpu: f32,
    pub memory_mb: u64,
}

// --- Shared State ---
// System is expensive to create; we keep one instance alive for the whole app
pub struct AppState(pub Mutex<System>);

// --- Commands ---

#[tauri::command]
fn get_cpu_info(state: State<AppState>) -> CpuInfo {
    let mut sys = state.0.lock().unwrap();
    sys.refresh_cpu_usage();

    let usage = sys.global_cpu_usage();
    let core_count = sys.cpus().len();

    CpuInfo { usage, core_count }
}

#[tauri::command]
fn get_memory_info(state: State<AppState>) -> MemoryInfo {
    let mut sys = state.0.lock().unwrap();
    sys.refresh_memory();

    let total = sys.total_memory() / 1024 / 1024;
    let used = sys.used_memory() / 1024 / 1024;
    let percent = (used as f32 / total as f32) * 100.0;

    MemoryInfo {
        used_mb: used,
        total_mb: total,
        percent,
    }
}

#[tauri::command]
fn get_top_processes(state: State<AppState>) -> Vec<ProcessInfo> {
    let mut sys = state.0.lock().unwrap();
    sys.refresh_all();

    let mut processes: Vec<ProcessInfo> = sys
        .processes()
        .values()
        .map(|p| ProcessInfo {
            name: p.name().to_string_lossy().to_string(),
            cpu: p.cpu_usage(),
            memory_mb: p.memory() / 1024 / 1024,
        })
        .collect();

    // Sort by CPU usage, return top 10
    processes.sort_by(|a, b| b.cpu.partial_cmp(&a.cpu).unwrap());
    processes.truncate(10);
    processes
}

// --- App Entry Point ---

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState(Mutex::new(System::new_all())))
        .invoke_handler(tauri::generate_handler![
            get_cpu_info,
            get_memory_info,
            get_top_processes
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
