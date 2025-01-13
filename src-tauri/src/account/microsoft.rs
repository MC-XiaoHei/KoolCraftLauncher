use serde::{Serialize, Deserialize};
use serde_json::{json, Value, to_value};
use std::collections::HashMap;
use tauri::webview::{PageLoadEvent, WebviewBuilder};
use tauri::{
	AppHandle, Emitter, LogicalPosition, Manager, WebviewUrl, Window, WindowBuilder, WindowEvent,
};
use tauri_plugin_http::reqwest;
use sys_locale::get_locale;
use chrono::{Duration, Utc, DateTime};
use std::fmt;
use tauri_plugin_store::StoreExt;

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
	Error(MicrosoftLoginError),
}

#[derive(Clone, Debug, Serialize)]
enum MicrosoftLoginError{
    CreateWindowError,
    OAuthError,
    NetworkError(u16),
    ProfileNotFoundError,
    UnknownError,
}

impl fmt::Display for MicrosoftLoginError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MicrosoftLoginError::CreateWindowError => write!(f, "Create window error"),
            MicrosoftLoginError::OAuthError => write!(f, "OAuth error"),
            MicrosoftLoginError::NetworkError(code) => write!(f, "Network error: {}", code),
            MicrosoftLoginError::ProfileNotFoundError => write!(f, "Profile not found"),
            MicrosoftLoginError::UnknownError => write!(f, "Unknown error"),
        }
    }
}

impl std::error::Error for MicrosoftLoginError {}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MicrosoftAccountData {
    uuid: String,
    name: String,
    access_token: String,
    refresh_token: String,
    expires_at: DateTime<Utc>,
}

pub async fn open_microsoft_login_webview(app: AppHandle) {
    let is_zh = || -> bool {
        if let Some(locale) = get_locale() {
            return locale.starts_with("zh");
        }
        false
    };

	let window = match WindowBuilder::new(&app, MS_LOGIN_WINDOW_ID)
		.visible(false)
		.title(if is_zh() { "登录" } else { "Login" })
		.build()
	{
		Ok(window) => window,
		Err(_) => {
			app.emit(
                MS_LOGIN_STATUS_EVENT,
                MicrosoftLoginStatus::Error(MicrosoftLoginError::CreateWindowError)
            ).unwrap();
			return;
		}
	};
	let app_clone = app.clone();
	window.on_window_event(move |event| match event {
		WindowEvent::CloseRequested { api: _, .. } => {
			app_clone.emit(MS_LOGIN_WINDOW_CLOSED_EVENT, "").unwrap();
		}
		_ => {}
	});
	let app_clone = app.clone();
	let webview_builder = WebviewBuilder::new(
		MS_LOGIN_WINDOW_ID,
		WebviewUrl::External(MS_LOGIN_URL.parse().unwrap()),
	)
	.on_page_load(move |webview, payload| match payload.event() {
		PageLoadEvent::Started => {}
		PageLoadEvent::Finished => {
			app_clone
				.emit(MS_LOGIN_STATUS_EVENT, MicrosoftLoginStatus::WaitingForOAuth)
				.unwrap();
			let window: Window = webview.window();
			window.show().unwrap();
			window.set_focus().unwrap();
		}
	});
	let app_clone = app.clone();
	let webview_builder = webview_builder.on_navigation(move |url| {
		if url.host_str() == Some("login.live.com")
			&& url.path() == "/oauth20_desktop.srf"
			&& url.scheme() == "https"
		{
			app_clone
				.get_window(MS_LOGIN_WINDOW_ID)
				.unwrap()
				.close()
				.unwrap();
			let query_pairs: Vec<(String, String)> = url
				.query_pairs()
				.map(|(k, v)| (k.to_string(), v.to_string()))
				.collect();
			match query_pairs.iter().find(|(key, _)| key == "code") {
				Some((_, code)) => {
					let code_clone = code.clone();
					app_clone
						.emit(MS_LOGIN_STATUS_EVENT, MicrosoftLoginStatus::Authenticating)
						.unwrap();
					let app_clone = app_clone.clone();
					tauri::async_runtime::spawn(async move {
						do_auth(app_clone, &code_clone).await;
					});
				}
				None => {
					app_clone.emit(
                        MS_LOGIN_STATUS_EVENT,
                        MicrosoftLoginStatus::Error(MicrosoftLoginError::OAuthError)
                    ).unwrap();
				}
			}
			return false;
		}
		true
	});
	let _ = window.add_child(
		webview_builder,
		LogicalPosition::new(0, 0),
		window.inner_size().unwrap(),
	);
}

pub async fn close_microsoft_login_webview(app: AppHandle) {
	match app.get_window(MS_LOGIN_WINDOW_ID) {
		Some(window) => {
			window.close().unwrap();
		}
		None => {}
	}
}

async fn do_auth(app: AppHandle, auth_code: &str) {
	match do_auth_internal(auth_code).await {
		Ok(account_data) => {
            match app.store("account-data.json") {
                Ok(store) => {
                    let value = to_value(&account_data).unwrap();
                    let uuid = account_data.uuid.clone();
                    println!("{}", value);
                    store.set(format!("microsoft@{}", uuid), value);
                    app.emit(MS_LOGIN_STATUS_EVENT, MicrosoftLoginStatus::Success)
                    	.unwrap();
                }
                Err(_) => {
                    app.emit(
                        MS_LOGIN_STATUS_EVENT,
                        MicrosoftLoginStatus::Error(MicrosoftLoginError::UnknownError)
                    ).unwrap();
                }
            }
		}
		Err(error) => {
			if let Some(microsoft_error) = error.downcast_ref::<MicrosoftLoginError>() {
                app.emit(
                    MS_LOGIN_STATUS_EVENT,
                    MicrosoftLoginStatus::Error(microsoft_error.clone())
                ).unwrap();
            } else {
                app.emit(
                    MS_LOGIN_STATUS_EVENT,
                    MicrosoftLoginStatus::Error(MicrosoftLoginError::UnknownError)
                ).unwrap();
            }
		}
	}
}

async fn do_auth_internal(auth_code: &str) -> Result<MicrosoftAccountData, Box<dyn std::error::Error>> {
	let client = reqwest::Client::new();
	let (access_token, refresh_token) = get_access_token(&client, auth_code).await?;
	let (xbl_token, uhs) = get_xbl_token(&client, &access_token).await?;
	let xsts_token = get_xsts_token(&client, &xbl_token).await?;
	let (minecraft_token, expires_at) = get_minecraft_token(&client, &xsts_token, &uhs).await?;
	let (uuid, name) = get_minecraft_profile(&client, &minecraft_token).await?;
	Ok(MicrosoftAccountData {
        uuid,
        name,
        access_token,
        refresh_token,
        expires_at,
    })
}

async fn get_access_token(
	client: &reqwest::Client,
	auth_code: &str,
) -> Result<(String, String), Box<dyn std::error::Error>> {
	let mut data = HashMap::new();
	data.insert("client_id", "00000000402b5328");
	data.insert("code", auth_code);
	data.insert("grant_type", "authorization_code");
	data.insert("redirect_uri", "https://login.live.com/oauth20_desktop.srf");
	data.insert("scope", "service::user.auth.xboxlive.com::MBI_SSL");

	let res = client
		.post("https://login.live.com/oauth20_token.srf")
		.form(&data)
		.header("Content-Type", "application/x-www-form-urlencoded")
		.send()
		.await?;

	if res.status().is_success() {
		let body: HashMap<String, Value> = res.json().await?;
		let access_token = body.get("access_token").unwrap().to_string();
		let refresh_token = body.get("refresh_token").unwrap().to_string();
		Ok((
			access_token[1..access_token.len() - 1].parse().unwrap(),
			refresh_token[1..refresh_token.len() - 1].parse().unwrap(),
		))
	} else {
        Err(MicrosoftLoginError::NetworkError(res.status().as_u16()).into())
	}
}

async fn get_xbl_token(
	client: &reqwest::Client,
	access_token: &str,
) -> Result<(String, String), Box<dyn std::error::Error>> {
	let json_data = json!({
		"Properties": {
			"AuthMethod": "RPS",
			"SiteName": "user.auth.xboxlive.com",
			"RpsTicket": access_token,
		},
		"RelyingParty": "http://auth.xboxlive.com",
		"TokenType": "JWT",
	});

	let res = client
		.post("https://user.auth.xboxlive.com/user/authenticate")
		.json(&json_data)
		.header("Content-Type", "application/json")
		.header("Accept", "application/json")
		.send()
		.await?;

	if res.status().is_success() {
		let body: HashMap<String, Value> = res.json().await?;
		let token = body.get("Token").unwrap().to_string();
		let uhs = body.get("DisplayClaims").unwrap()["xui"][0]["uhs"].to_string();
		Ok((
			token[1..token.len() - 1].parse().unwrap(),
			uhs[1..uhs.len() - 1].parse().unwrap(),
		))
	} else {
        Err(MicrosoftLoginError::NetworkError(res.status().as_u16()).into())
	}
}

async fn get_xsts_token(
	client: &reqwest::Client,
	xbl_token: &str,
) -> Result<String, Box<dyn std::error::Error>> {
	let json_data = json!({
		"Properties": {
			"SandboxId": "RETAIL",
			"UserTokens": [xbl_token],
		},
		"RelyingParty": "rp://api.minecraftservices.com/",
		"TokenType": "JWT",
	});

	let res = client
		.post("https://xsts.auth.xboxlive.com/xsts/authorize")
		.json(&json_data)
		.header("Content-Type", "application/json")
		.header("Accept", "application/json")
		.send()
		.await?;

	if res.status().is_success() {
		let body: HashMap<String, Value> = res.json().await?;
		let token = body.get("Token").unwrap().to_string();
		Ok(token[1..token.len() - 1].parse().unwrap())
	} else {
        Err(MicrosoftLoginError::NetworkError(res.status().as_u16()).into())
	}
}

async fn get_minecraft_token(
	client: &reqwest::Client,
	xsts_token: &str,
	uhs: &str,
) -> Result<(String, DateTime<Utc>), Box<dyn std::error::Error>> {
	let json_data = json!({
		"identityToken": format!("XBL3.0 x={};{}", uhs, xsts_token),
	});

	let res = client
		.post("https://api.minecraftservices.com/authentication/login_with_xbox")
		.json(&json_data)
		.header("Content-Type", "application/json")
		.header("Accept", "application/json")
		.send()
		.await?;

	if res.status().is_success() {
		let body: HashMap<String, Value> = res.json().await?;
		let token = body.get("access_token").unwrap().to_string();
		return match body.get("expires_in") {
            Some(expires_in) => {
                let expires_in = expires_in.as_i64().unwrap();
                let expires_at = Utc::now() + Duration::seconds(expires_in);
                 Ok((
                    token[1..token.len() - 1].parse().unwrap(),
                    expires_at,
                ))
            }
            None => Err(MicrosoftLoginError::UnknownError.into()),
        }
	} else {
        Err(MicrosoftLoginError::NetworkError(res.status().as_u16()).into())
	}
}

async fn get_minecraft_profile(
    client: &reqwest::Client,
    minecraft_token: &str,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let res = client
        .get("https://api.minecraftservices.com/minecraft/profile")
        .header("Authorization", format!("Bearer {}", minecraft_token))
        .send()
        .await?;

    if res.status().is_success() {
        let body: HashMap<String, Value> = res.json().await?;
        if body.contains_key("error") {
            return Err(MicrosoftLoginError::ProfileNotFoundError.into());
        }
        let uuid = body.get("id").unwrap().to_string();
        let name = body.get("name").unwrap().to_string();

        Ok((
            uuid[1..uuid.len() - 1].parse().unwrap(),
            name[1..name.len() - 1].parse().unwrap(),
        ))
    } else {
        Err(MicrosoftLoginError::NetworkError(res.status().as_u16()).into())
    }
}