fn main() {
    if let Err(err) = bobine_app::run() {
        eprintln!("error while running tauri application: {err}");
        std::process::exit(1);
    }
}
