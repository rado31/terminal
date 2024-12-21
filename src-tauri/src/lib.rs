mod application;
mod cashcode;
mod config;
mod logger;

use application::{commands, App};
use std::process;
use tauri::{Builder, Manager};

pub fn run() {
    logger::init();

    let options = config::Options::default();
    let state = App::build(options).unwrap_or_else(|error| {
        log::error!("App build: {error}");
        process::exit(1);
    });

    if let Err(error) = Builder::default()
        .setup(|app| {
            app.manage(state);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![commands::start_payment])
        .run(tauri::generate_context!())
    {
        log::error!("Run tauri: {error}")
    }
}
