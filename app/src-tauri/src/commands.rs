use std::collections::HashMap;
use reqwest::Response;
use tauri::{Manager, Window};

#[tauri::command]
pub async fn close_splashscreen(window: Window) {
    window.get_window("splashscreen").expect("no window labeled 'splashscreen' found").close().unwrap();
    window.get_window("main").expect("no window labeled 'main' found").show().unwrap();
}
#[tauri::command]
pub async fn handle_sign_in(email: String, password: String) -> Result<Response, String> {
    let client = reqwest::Client::new();

    let mut map = HashMap::new();
    map.insert("email".to_string(), email);
    map.insert("password".to_string(), password);

    let res = client.post("http://localhost:8080/auth/login")
        .json(&map)
        .send()
        .await.map_err(|e| "Request failed".to_string())?;

    res.into()
}