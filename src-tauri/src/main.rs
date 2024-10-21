#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use std::time::UNIX_EPOCH;
use tokio::time;
use sysinfo::{ProcessesToUpdate, Signal, System};
use tokio::runtime::Runtime;

pub static TIME: Mutex<i64> = Mutex::new(0);

fn main() {
    let rt = Runtime::new().unwrap();
    rt.spawn(check_tasks());
    lifeguard_lib::run();
}

async fn check_tasks() {
    let mut interval = time::interval(time::Duration::from_secs(5));
    loop {
        interval.tick().await;
        check_and_kill_process().await;
    }
}

async fn check_and_kill_process() {
    let mut sys = System::new_all();
    let timestamp = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();
    if let Some(time) = lifeguard_lib::get_time() {
        if time as u128 > timestamp {
            return;
        }
        sys.refresh_processes(ProcessesToUpdate::All, true);
        for (pid, process) in sys.processes() {
            if is_target_process(process) {
                sys.process(*pid).expect("Failed to kill process").kill_with(Signal::Kill);
            }
        }
    }
}

fn is_target_process(process: &sysinfo::Process) -> bool {
    let name = process.name();
    if let Some(s) = name.to_str() {
        return s.to_lowercase().eq("code.exe");
    }
    false
}