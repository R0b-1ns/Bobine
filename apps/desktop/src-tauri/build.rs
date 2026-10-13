fn main() {
    let dist = std::path::Path::new("../dist");
    if let Err(err) = std::fs::create_dir_all(dist) {
        println!("cargo:warning=could not create frontend dist: {err}");
    }
    let index = dist.join("index.html");
    if !index.exists() {
        if let Err(err) = std::fs::write(
            &index,
            "<!doctype html><html><body>Bobine</body></html>",
        ) {
            println!("cargo:warning=could not write frontend dist index: {err}");
        }
    }
    tauri_build::build()
}
