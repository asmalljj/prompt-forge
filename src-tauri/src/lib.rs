mod commands;
mod config;
mod error;
mod services;
mod settings;
mod state;

use tauri_plugin_log::{Target, TargetKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 加载环境变量
    config::init();

    tauri::Builder::default()
        // 注册日志插件
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(Target::new(TargetKind::Stdout))
                .target(Target::new(TargetKind::Webview))
                .target(Target::new(TargetKind::Folder {
                    path: std::path::PathBuf::from("logs"),
                    file_name: Some("prompt-forge".into()),
                }))
                .level(log::LevelFilter::Info)
                .build(),
        )
        // 注册应用状态（数据库连接等）
        .manage(state::AppState::new())
        // 注册 Tauri 命令
        .invoke_handler(tauri::generate_handler![
            commands::health::check_health,
            commands::optimize::optimize_prompt,
            commands::hello::hello,
            commands::settings::get_settings,
            commands::settings::save_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running prompt-forge application");
}
