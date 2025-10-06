use std::sync::Mutex;
use std::path::Path;
use tauri::State;

struct AppState {
    is_downloaded: Mutex<bool>,
}

#[tauri::command]
fn get_downloaded(state: State<AppState>) -> bool {
    let downloaded = state.is_downloaded.lock().unwrap();
    *downloaded
}

#[tauri::command]
fn verify_downloaded(state: State<AppState>) {
    let path_expected = r"C:\Users\Usuário\Documents\legoRobot";
    let path = Path::new(path_expected);
    let mut downloaded = state.is_downloaded.lock().unwrap();
    *downloaded = path.exists();
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            is_downloaded: Mutex::new(false),
        })
        .invoke_handler(tauri::generate_handler![get_downloaded, verify_downloaded])
        .run(tauri::generate_context!())
        .expect("Erro ao rodar o app Tauri");
}
