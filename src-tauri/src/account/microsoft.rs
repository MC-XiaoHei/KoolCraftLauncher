use tauri::{Manager, WindowBuilder, Window, AppHandle, Emitter, WebviewUrl, LogicalPosition, WindowEvent};
use tauri::webview::{PageLoadEvent, WebviewBuilder};
use serde::Serialize;
use tauri_plugin_http::reqwest;
use std::collections::HashMap;

const MS_LOGIN_URL: &str = "https://login.live.com/oauth20_authorize.srf
        ?client_id=00000000402b5328
        &response_type=code
        &scope=service%3A%3Auser.auth.xboxlive.com%3A%3AMBI_SSL
        &redirect_uri=https%3A%2F%2Flogin.live.com%2Foauth20_desktop.srf
        &prompt=select_account";
const MS_LOGIN_WINDOW_ID: &str = "microsoft_login";
const MS_LOGIN_STATUS_EVENT: &str = "microsoft-login-status";
const MS_LOGIN_WINDOW_CLOSED_EVENT: &str = "microsoft-login-window-closed";

#[derive(Clone, Serialize)]
enum MicrosoftLoginStatus {
    WaitingForOAuth,
    Authenticating,
    Success,
    Error,
}

pub async fn open_microsoft_login_webview(app: AppHandle) {
    let window = match WindowBuilder::new(&app, MS_LOGIN_WINDOW_ID)
        .visible(false)
        .title("登录 | Login")
        .build() {
            Ok(window) => window,
            Err(_) => {
                app.emit(MS_LOGIN_STATUS_EVENT, MicrosoftLoginStatus::Error).unwrap();
                return;
            },
        };
    let app_clone = app.clone();
    window.on_window_event(move |event| {
        match event {
            WindowEvent::CloseRequested { api: _, .. } => {
                app_clone.emit(MS_LOGIN_WINDOW_CLOSED_EVENT, "").unwrap();
            }
            _ => {}
        }
    });
    let app_clone = app.clone();
    let webview_builder = WebviewBuilder::new(
        MS_LOGIN_WINDOW_ID,
         WebviewUrl::External(MS_LOGIN_URL.parse().unwrap())
    ).on_page_load(move |webview, payload| {
        match payload.event() {
            PageLoadEvent::Started => {}
            PageLoadEvent::Finished => {
                app_clone.emit(MS_LOGIN_STATUS_EVENT, MicrosoftLoginStatus::WaitingForOAuth).unwrap();
                let window: Window = webview.window();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
        }
    });
    let app_clone = app.clone();
    let webview_builder = webview_builder.on_navigation(move |url| {
        if url.host_str() == Some("login.live.com")
         && url.path() == "/oauth20_desktop.srf"
         && url.scheme() == "https" {
            app_clone.get_window(MS_LOGIN_WINDOW_ID).unwrap().close().unwrap();
            match url.query_pairs().find(|(key, _)| key == "code") {
                Some((_, code)) => {
                    app_clone.emit(MS_LOGIN_STATUS_EVENT, MicrosoftLoginStatus::Authenticating).unwrap();
                    let app_clone = app_clone.clone();
                    let code_str = code.to_string();
                    tauri::async_runtime::spawn(async move {
                        do_auth(app_clone, code_str).await;
                    });
                }
                None => {
                    app_clone.emit(MS_LOGIN_STATUS_EVENT, MicrosoftLoginStatus::Error).unwrap();
                }
            }
            return false;
        }
        true
    });
    let _ = window.add_child(webview_builder, LogicalPosition::new(0, 0), window.inner_size().unwrap());
}

pub async fn close_microsoft_login_webview(app: AppHandle) {
    match app.get_window(MS_LOGIN_WINDOW_ID) {
        Some(window) => {
            window.close().unwrap();
        }
        None => {}
    }
}

async fn do_auth(app: AppHandle, auth_code: String) {
    match do_auth_internal(auth_code).await {
        Ok(_) => {
            app.emit(MS_LOGIN_STATUS_EVENT, MicrosoftLoginStatus::Success).unwrap();
        }
        Err(_) => {
            app.emit(MS_LOGIN_STATUS_EVENT, MicrosoftLoginStatus::Error).unwrap();
        }
    }
}

async fn do_auth_internal(auth_code: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let (access_token, refresh_token) = get_access_token(client, auth_code).await?;
    println!("{} {}", access_token, refresh_token);
    Ok(())
}

async fn get_access_token(client: reqwest::Client, auth_code: String) -> Result<(String, String), Box<dyn std::error::Error>> {
     let mut data = HashMap::new();
     data.insert("client_id", "00000000402b5328");
     data.insert("code", &auth_code);
     data.insert("grant_type", "authorization_code");
     data.insert("redirect_uri", "https://login.live.com/oauth20_desktop.srf");
     data.insert("scope", "service::user.auth.xboxlive.com::MBI_SSL");

     let res = client.post("https://login.live.com/oauth20_token.srf")
         .form(&data)
         .header("Content-Type", "application/x-www-form-urlencoded")
         .send()
         .await?;

     if res.status().is_success() {
         let body: HashMap<String, String> = res.json().await?;
         let access_token = body.get("access_token").unwrap_or(&"".to_string()).clone();
         let refresh_token = body.get("refresh_token").unwrap_or(&"".to_string()).clone();
         Ok((access_token, refresh_token))
     } else {
         Err(format!("Request failed with status: {}", res.status()).into())
     }
 }