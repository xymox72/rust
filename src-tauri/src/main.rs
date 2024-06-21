// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod service;
mod utils_base;

use chrono::{DateTime, Utc};
use dotenv_vault::dotenv;
use service::message_service::Service;
use service::models::message::Message;
use std::*;

use tauri::command;
use tauri::State;

struct AppState {
    service: Service,
}

#[command]
async fn remove_files(state: State<'_, AppState>,  selected_date: String,) -> Result<u64, String> {
    let service = &state.service;
    let datetime: DateTime<Utc> =
    utils_base::utils_base::date_format(&selected_date).map_err(|err| err.to_string())?;

    let row_affected = service.remove_messages(datetime).await.map_err(|err| err.to_string());
    
    match row_affected {
        Ok(num) => Ok(num),
        Err(err) => Err(err),
    }
  
}

#[command]
async fn get_meesages(
    state: State<'_, AppState>,
    selected_date: String,
) -> Result<Vec<Message>, String> {
    let service = &state.service;

    let date_str = selected_date;

    let datetime: DateTime<Utc> =
        utils_base::utils_base::date_format(&date_str).map_err(|err| err.to_string())?;

    let messages_result = service
        .get_meesages(datetime)
        .await
        .map_err(|err| err.to_string());

    match messages_result {
        Ok(messages) => Ok(messages),
        Err(err) => Err(err),
    }
}

#[command]
async fn count(state: State<'_, AppState>, selected_date: String) -> Result<i64, String> {
    let service = &state.service;

    let date_str = selected_date;

    let datetime: DateTime<Utc> =
        utils_base::utils_base::date_format(&date_str).map_err(|err| err.to_string())?;
    let count_result = service
        .get_count_of_messages(datetime)
        .await
        .map_err(|err| err.to_string());

    match count_result {
        Ok(count) => Ok(count),
        Err(err) => Err(err),
    }
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let service = Service::new().await?;

    tauri::Builder::default()
        .manage(AppState { service })
        .invoke_handler(tauri::generate_handler![get_meesages, remove_files, count])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
