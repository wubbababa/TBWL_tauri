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
            .additional_browser_args("--ignore-certificate-errors --ignore-ssl-errors --overscroll-history-navigation=0 --disable-features=ElasticOverscroll")
            .initialization_script(r#"
                (function() {
                    var style = document.createElement('style');
                    style.textContent = 'html, body { overscroll-behavior: none !important; overflow: auto; }';
                    (document.head || document.documentElement).appendChild(style);
                    document.addEventListener('DOMContentLoaded', function() {
                        document.documentElement.style.setProperty('overscroll-behavior', 'none', 'important');
                        document.body.style.setProperty('overscroll-behavior', 'none', 'important');
                    });
                })();
            "#)
            .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
