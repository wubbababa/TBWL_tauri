use tauri::{WebviewWindowBuilder, WebviewUrl};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External("https://tbwl.vercel.app/".parse().unwrap()),
            )
            .title("TBWL")
            .inner_size(1280.0, 800.0)
            .resizable(true)
            // 传入 Chromium/Edge 命令行参数，忽略证书错误
            .additional_browser_args("--ignore-certificate-errors --ignore-ssl-errors")
            .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
