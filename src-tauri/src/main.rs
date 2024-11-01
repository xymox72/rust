// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod service;
mod utils_base;
mod logging;
use chrono::{DateTime, Utc};
use collections::HashMap;
use flexi_logger::{FileSpec, Logger, WriteMode};
use log::{info, log, error};
use service::message_service::Service;
use service::models::message::Message;
use utils_base::utils_base::MessageServiceError;
use dotenv::dotenv;
use std::*;

use tauri::{command, State, Window};


struct AppState {
    service: Service,
}

#[command]
async fn remove_files(state: State<'_, AppState>, window: Window,  selected_date: String,) -> Result<(), String> {
    let service = &state.service;
    let datetime: DateTime<Utc> =
    utils_base::utils_base::date_format(&selected_date).map_err(|err| err.to_string())?;
    
    service.remove_messages(datetime, |message| {
        window.emit("file", message).unwrap();
    }).await.map_err(|err| err.to_string())?;
    
   Ok(())
  
}

#[command]
 fn get_envs() -> Result<HashMap<String, String>, String> {
    Ok(env::vars().collect())
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
        .get_meesages(datetime, None)
        .await
        .map_err(|err| err.to_string());

    match messages_result {
        Ok(messages) => Ok(messages),
        Err(err) => Err(err),
    }
}

#[command]
async fn count(state: State<'_, AppState>,  selected_date: String) -> Result<i64, String> {
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
async fn main() -> Result<(), MessageServiceError> {
  logging::init_logging();
    info!("Приложение запускается...");
    match dotenv() {
        Ok(_) => info!("Файл .env успешно загружен"),
        Err(e) => error!("Не удалось загрузить .env файл: {}", e),
    }
    let service = Service::new().await?;

       ctrlc::set_handler(move || {
        info!("Program is shutting down");
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");
        tauri::Builder::default()

        .manage(AppState { service })
        .invoke_handler(tauri::generate_handler![get_meesages, remove_files, count, get_envs])
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
            info!("Program is shutting down");
            info!("##########GOODBYE##########");
              
            }
            _ => {}
          })
        .run(tauri::generate_context!())
        
        .expect("error while running tauri application");

    Ok(())
}
