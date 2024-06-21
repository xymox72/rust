// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod db;
mod models;
mod utils_base;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use db::Database;
use models::message::Message;
use std::{error::Error, fmt::format};
use tauri::command;
use tauri::State;

struct AppState {
    db: Database,
}

#[command]
async fn remove_files(state: State<'_, AppState>) -> Result<(), String> {
    let db = &state.db;
    let date_str = "20.04.2024";

    Ok(())
}

#[command]
async fn get_meesages(
    state: State<'_, AppState>,
    selected_date: String,
) -> Result<Vec<Message>, String> {
    let db = &state.db;

    let date_str = selected_date;

    let datetime: DateTime<Utc> =
        utils_base::utils_base::date_format(&date_str).map_err(|err| err.to_string())?;
    let result = db
        .get_messages(&datetime)
        .await
        .map_err(|err| err.to_string());

    match result {
        Ok(messages) => Ok(messages),
        Err(err) => Err(err),
    }
}


#[command]
async fn count(
    state: State<'_, AppState>,
    selected_date: String,
) -> Result<i64, String> {
    let db = &state.db;

    let date_str = selected_date;

    let datetime: DateTime<Utc> =
        utils_base::utils_base::date_format(&date_str).map_err(|err| err.to_string())?;
    let result = db
        .count(&datetime)
        .await
        .map_err(|err| err.to_string());

    match result {
        Ok(messages) => Ok(messages),
        Err(err) => Err(err),
    }
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db = Database::new("mysql://root:12345@localhost/hmaildb").await?;

    tauri::Builder::default()
        .manage(AppState { db })
        .invoke_handler(tauri::generate_handler![get_meesages, remove_files, count])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
