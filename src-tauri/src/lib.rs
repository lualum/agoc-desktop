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
                const d = window.__TAURI__?.dialog;
                if (d) {
                    window.alert = m => d.message(String(m));
                    window.confirm = m => (d.confirm(String(m)), true);
                }

                const style = document.createElement('style');
                style.textContent = `
                    html, body {
                        overscroll-behavior: none;
                        position: fixed;
                        overflow: hidden;
                        width: 100%;
                        height: 100%;
                    }
                `;
                (document.head || document.documentElement).appendChild(style);
            "#)
            .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}