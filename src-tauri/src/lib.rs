// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use chrono::Datelike;
use chrono::Local;

#[tauri::command]
fn calcular_edad(anio_nacimiento: u32) -> u32 {
    let fecha_actual = Local::now();
    let anio_actual = fecha_actual.year();

    let edad = anio_actual - anio_nacimiento as i32;

    edad as u32
}

#[tauri::command]
fn calcular_anio_nacimiento(edad: u32) -> u32 {
    let fecha_actual = Local::now();
    let anio_actual = fecha_actual.year();

    let anio_nacimiento = anio_actual - edad as i32;

    anio_nacimiento as u32
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            calcular_edad,
            calcular_anio_nacimiento
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
