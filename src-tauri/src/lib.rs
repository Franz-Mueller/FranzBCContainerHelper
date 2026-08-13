use self::docker::container::{get_version, list_containers};
use tauri_plugin_sql::{Migration, MigrationKind};
mod docker;
mod utils;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![Migration {
        version: 1,
        description: "create_initial_tables",
        sql:
            "CREATE TABLE containers (id INTEGER PRIMARY KEY, name TEXT, image TEXT, status TEXT);",
        kind: MigrationKind::Up,
    }];
    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:mydatabase.db", migrations)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_version,
            list_containers
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
