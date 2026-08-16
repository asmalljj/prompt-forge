mod commands;
mod config;
mod error;
mod services;
mod settings;
mod state;

use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 加载环境变量
    config::init();

    tauri::Builder::default()
        // 单实例插件：重复启动时聚焦已有窗口，防止多实例残留
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            log::info!("检测到重复启动，聚焦已有窗口");
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_focus();
            }
        }))
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
