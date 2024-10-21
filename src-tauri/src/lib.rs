use std::sync::Mutex;

pub static TIME: Mutex<i64> = Mutex::new(0);

#[tauri::command]
fn reset_time(invoke_time: i64) {
    set_time(invoke_time);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![reset_time])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[allow(dead_code)]
pub fn get_time() -> Option<i64> {
    match TIME.try_lock() {
        Ok(time) => Some(*time),
        Err(_) => None
    }
}

#[allow(dead_code)]
pub fn set_time(invoke_time: i64) {
    let mut time = TIME.try_lock().unwrap();
    *time = invoke_time;
}