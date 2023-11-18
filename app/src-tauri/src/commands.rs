use std::collections::HashMap;
use reqwest::{Response, StatusCode};
use tauri::{Manager, Window};

#[tauri::command]
pub async fn close_splashscreen(window: Window) {
    window.get_window("splashscreen").expect("no window labeled 'splashscreen' found").close().unwrap();
    window.get_window("main").expect("no window labeled 'main' found").show().unwrap();
}

#[tauri::command]
pub async fn handle_sign_in(email: String, password: String) -> Result<serde_json::Value, serde_json::Value> {
    let client = reqwest::Client::new();

    let mut map = HashMap::new();
    map.insert("email".to_string(), email);
    map.insert("password".to_string(), password);

    let res = client.post("http://localhost:8080/auth/login")
        .json(&map)
        .send()
        .await.map_err(|_e| "Failed to connect to the API".to_string())?;

    let status = res.status();

    let json: serde_json::Value = res.json().await.map_err(|_e| "Failed to transform into json".to_string())?;

    let message: serde_json::Value = json["Message"].clone().into();

    if status != StatusCode::OK {
        return Err(message);
    }

    Ok(message)
}