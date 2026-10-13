use bobine_daemon::{BobineService, ClipSummary, Volume};
use tauri::Manager;

#[tauri::command]
fn list_volumes(service: tauri::State<BobineService>) -> Result<Vec<Volume>, String> {
    service.list_volumes().map_err(|err| err.to_string())
}

#[tauri::command]
fn list_clips(service: tauri::State<BobineService>) -> Result<Vec<ClipSummary>, String> {
    service.list_clips().map_err(|err| err.to_string())
}

pub fn run() -> Result<(), tauri::Error> {
    tauri::Builder::default()
        .setup(|app| {
            let dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&dir)?;
            let service = BobineService::open(&dir.join("bobine.sqlite")).map_err(|err| {
                Box::<dyn std::error::Error>::from(err.to_string())
            })?;
            app.manage(service);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![list_volumes, list_clips])
        .run(tauri::generate_context!())
}
