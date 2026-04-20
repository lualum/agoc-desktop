// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            tauri::WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::External("https://gameofchance.xyz/".parse().unwrap()),
            )
            .title("GameOfChance")
            .inner_size(1000.0, 800.0)
            .initialization_script(r#"
                window.alert = function(message) {
                    if (window.__TAURI__?.dialog) {
                        window.__TAURI__.dialog.message(String(message));
                    }
                };
                window.confirm = function(message) {
                    if (window.__TAURI__?.dialog) {
                        window.__TAURI__.dialog.confirm(String(message));
                    }
                    return true;
                };
            "#)
            .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}