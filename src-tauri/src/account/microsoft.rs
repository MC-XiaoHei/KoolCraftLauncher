use chrono::{DateTime, Duration, Utc};
use serde::Serialize;
use serde_json::{json, to_value, Value};
use std::collections::HashMap;
use sys_locale::get_locale;
use tauri::webview::{PageLoadEvent, WebviewBuilder};
use tauri::{
	AppHandle, Emitter, LogicalPosition, Manager, WebviewUrl, Window, WindowBuilder, WindowEvent,
};
use tauri_plugin_http::reqwest;
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
struct UpdateStatusEvent {
	status: MicrosoftLoginStatus,
	auth_progress: Option<AuthProgress>,
	profile: Option<ProfileData>,
	error: Option<MicrosoftLoginError>,
}

#[derive(Clone, Serialize)]
enum AuthProgress {
	GetAccessToken,
	GetXBLToken,
	GetXSTSToken,
	GetMinecraftToken,
	GetMinecraftProfile,
}

#[derive(Clone, Serialize)]
enum MicrosoftLoginStatus {
	WaitingForOAuth,
	Authenticating,
	Success,
	Error,
}

#[derive(Clone, Serialize)]
struct MicrosoftLoginError {
	error_type: MicrosoftLoginErrorType,
	message: String,
}

#[derive(Clone, Serialize)]
enum MicrosoftLoginErrorType {
	CreateWindowError,
	OAuthError,
	NetworkError,
	ProfileNotFoundError,
	APIError,
	FileSystemError,
}

#[derive(Clone, Serialize)]
struct MicrosoftAccountData {
	uuid: String,
	name: String,
	access_token: String,
	refresh_token: String,
	expires_at: DateTime<Utc>,
}

#[derive(Clone, Serialize)]
struct ProfileData {
	uuid: String,
	name: String,
}

fn update_login_status(
	app: &AppHandle,
	status: MicrosoftLoginStatus,
	auth_progress: Option<AuthProgress>,
	profile: Option<ProfileData>,
	error: Option<MicrosoftLoginError>,
) {
	let _ = &app
		.emit(
			MS_LOGIN_STATUS_EVENT,
			UpdateStatusEvent {
				status,
				auth_progress,
				profile,
				error,
			},
		)
		.unwrap();
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
		Err(error) => {
			update_login_status(
				&app,
				MicrosoftLoginStatus::Error,
				None,
				None,
				Some(MicrosoftLoginError {
					error_type: MicrosoftLoginErrorType::CreateWindowError,
					message: error.to_string(),
				}),
			);
			return;
		}
	};
	let app_clone = app.clone();
	window.on_window_event(move |event| {
		if let WindowEvent::CloseRequested { api: _, .. } = event {
			app_clone.emit(MS_LOGIN_WINDOW_CLOSED_EVENT, "").unwrap();
		}
	});
	let app_clone = app.clone();
	let webview_builder = WebviewBuilder::new(
		MS_LOGIN_WINDOW_ID,
		WebviewUrl::External(MS_LOGIN_URL.parse().unwrap()),
	)
	.on_page_load(move |webview, payload| match payload.event() {
		PageLoadEvent::Started => {}
		PageLoadEvent::Finished => {
			update_login_status(
				&app_clone,
				MicrosoftLoginStatus::WaitingForOAuth,
				None,
				None,
				None,
			);
			let window: Window = webview.window();
			window.show().unwrap();
			window.set_focus().unwrap();
		}
	});
	let app_clone = app.clone();
	let webview_builder = webview_builder.on_navigation(move |url| {
		if url.host_str() != Some("login.live.com")
			|| url.path() != "/oauth20_desktop.srf"
			|| url.scheme() != "https"
		{
			return true;
		}
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
				update_login_status(
					&app_clone,
					MicrosoftLoginStatus::Authenticating,
					None,
					None,
					None,
				);
				let app_clone = app_clone.clone();
				tauri::async_runtime::spawn(async move {
					do_auth(app_clone, &code_clone).await;
				});
			}
			None => update_login_status(
				&app_clone,
				MicrosoftLoginStatus::Error,
				None,
				None,
				Some(MicrosoftLoginError {
					error_type: MicrosoftLoginErrorType::OAuthError,
					message: "oauth code do not exist".to_string(),
				}),
			),
		}
		false
	});
	let _ = window.add_child(
		webview_builder,
		LogicalPosition::new(0, 0),
		window.inner_size().unwrap(),
	);
}

pub async fn close_microsoft_login_webview(app: AppHandle) {
	if let Some(window) = app.get_window(MS_LOGIN_WINDOW_ID) {
		window.close().unwrap();
	}
}

async fn do_auth(app: AppHandle, auth_code: &str) {
	let account_data = match get_account_data(&app, auth_code).await {
		Ok(account_data) => account_data,
		Err(error) => {
			update_login_status(&app, MicrosoftLoginStatus::Error, None, None, Some(error));
			return;
		}
	};
	let store = match app.store("account-data.json") {
		Ok(store) => store,
		Err(err) => {
			update_login_status(
				&app,
				MicrosoftLoginStatus::Error,
				None,
				None,
				Some(MicrosoftLoginError {
					error_type: MicrosoftLoginErrorType::FileSystemError,
					message: "cannot open store file, ".to_string() + &err.to_string(),
				}),
			);
			return;
		}
	};
	let value = to_value(&account_data).unwrap();
	let uuid = account_data.uuid.clone();
	let name = account_data.name.clone();
	store.set(format!("microsoft@{}", uuid), value);
	match store.save() {
		Ok(_) => update_login_status(
			&app,
			MicrosoftLoginStatus::Success,
			None,
			Some(ProfileData { uuid, name }),
			None,
		),
		Err(err) => update_login_status(
			&app,
			MicrosoftLoginStatus::Error,
			None,
			None,
			Some(MicrosoftLoginError {
				error_type: MicrosoftLoginErrorType::FileSystemError,
				message: "cannot save account data, ".to_string() + &err.to_string(),
			}),
		),
	}
}

async fn get_account_data(
	app: &AppHandle,
	auth_code: &str,
) -> Result<MicrosoftAccountData, MicrosoftLoginError> {
	let client = reqwest::Client::new();
	update_login_status(
		app,
		MicrosoftLoginStatus::Authenticating,
		Some(AuthProgress::GetAccessToken),
		None,
		None,
	);
	let (access_token, refresh_token) = get_access_token(&client, auth_code).await?;
	update_login_status(
		app,
		MicrosoftLoginStatus::Authenticating,
		Some(AuthProgress::GetXBLToken),
		None,
		None,
	);
	let (xbl_token, uhs) = get_xbl_token(&client, &access_token).await?;
	update_login_status(
		app,
		MicrosoftLoginStatus::Authenticating,
		Some(AuthProgress::GetXSTSToken),
		None,
		None,
	);
	let xsts_token = get_xsts_token(&client, &xbl_token).await?;
	update_login_status(
		app,
		MicrosoftLoginStatus::Authenticating,
		Some(AuthProgress::GetMinecraftToken),
		None,
		None,
	);
	let (minecraft_token, expires_at) = get_minecraft_token(&client, &xsts_token, &uhs).await?;
	update_login_status(
		app,
		MicrosoftLoginStatus::Authenticating,
		Some(AuthProgress::GetMinecraftProfile),
		None,
		None,
	);
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
) -> Result<(String, String), MicrosoftLoginError> {
	let mut data = HashMap::new();
	data.insert("client_id", "00000000402b5328");
	data.insert("code", auth_code);
	data.insert("grant_type", "authorization_code");
	data.insert("redirect_uri", "https://login.live.com/oauth20_desktop.srf");
	data.insert("scope", "service::user.auth.xboxlive.com::MBI_SSL");

	let res = match client
		.post("https://login.live.com/oauth20_token.srf")
		.form(&data)
		.header("Content-Type", "application/x-www-form-urlencoded")
		.send()
		.await
	{
		Ok(res) => res,
		Err(err) => {
			return Err(MicrosoftLoginError {
				error_type: MicrosoftLoginErrorType::NetworkError,
				message: err.to_string(),
			});
		}
	};

	if res.status().is_success() {
		let body: HashMap<String, Value> = match res.json().await {
			Ok(body) => body,
			Err(error) => {
				return Err(MicrosoftLoginError {
					error_type: MicrosoftLoginErrorType::APIError,
					message: error.to_string(),
				});
			}
		};
		let access_token = body.get("access_token").unwrap().to_string();
		let refresh_token = body.get("refresh_token").unwrap().to_string();
		Ok((
			access_token[1..access_token.len() - 1].parse().unwrap(),
			refresh_token[1..refresh_token.len() - 1].parse().unwrap(),
		))
	} else {
		Err(MicrosoftLoginError {
			error_type: MicrosoftLoginErrorType::APIError,
			message: res.text().await.unwrap(),
		})
	}
}

async fn get_xbl_token(
	client: &reqwest::Client,
	access_token: &str,
) -> Result<(String, String), MicrosoftLoginError> {
	let json_data = json!({
		"Properties": {
			"AuthMethod": "RPS",
			"SiteName": "user.auth.xboxlive.com",
			"RpsTicket": access_token,
		},
		"RelyingParty": "http://auth.xboxlive.com",
		"TokenType": "JWT",
	});

	let res = match client
		.post("https://user.auth.xboxlive.com/user/authenticate")
		.json(&json_data)
		.header("Content-Type", "application/json")
		.header("Accept", "application/json")
		.send()
		.await
	{
		Ok(res) => res,
		Err(err) => {
			return Err(MicrosoftLoginError {
				error_type: MicrosoftLoginErrorType::NetworkError,
				message: err.to_string(),
			});
		}
	};

	if res.status().is_success() {
		let body: HashMap<String, Value> = match res.json().await {
			Ok(body) => body,
			Err(err) => {
				return Err(MicrosoftLoginError {
					error_type: MicrosoftLoginErrorType::APIError,
					message: err.to_string(),
				});
			}
		};
		let token = body.get("Token").unwrap().to_string();
		let uhs = body.get("DisplayClaims").unwrap()["xui"][0]["uhs"].to_string();
		Ok((
			token[1..token.len() - 1].parse().unwrap(),
			uhs[1..uhs.len() - 1].parse().unwrap(),
		))
	} else {
		Err(MicrosoftLoginError {
			error_type: MicrosoftLoginErrorType::APIError,
			message: res.text().await.unwrap(),
		})
	}
}

async fn get_xsts_token(
	client: &reqwest::Client,
	xbl_token: &str,
) -> Result<String, MicrosoftLoginError> {
	let json_data = json!({
		"Properties": {
			"SandboxId": "RETAIL",
			"UserTokens": [xbl_token],
		},
		"RelyingParty": "rp://api.minecraftservices.com/",
		"TokenType": "JWT",
	});

	let res = match client
		.post("https://xsts.auth.xboxlive.com/xsts/authorize")
		.json(&json_data)
		.header("Content-Type", "application/json")
		.header("Accept", "application/json")
		.send()
		.await
	{
		Ok(res) => res,
		Err(err) => {
			return Err(MicrosoftLoginError {
				error_type: MicrosoftLoginErrorType::NetworkError,
				message: err.to_string(),
			});
		}
	};

	if res.status().is_success() {
		let body: HashMap<String, Value> = match res.json().await {
			Ok(body) => body,
			Err(err) => {
				return Err(MicrosoftLoginError {
					error_type: MicrosoftLoginErrorType::APIError,
					message: err.to_string(),
				});
			}
		};
		let token = body.get("Token").unwrap().to_string();
		Ok(token[1..token.len() - 1].parse().unwrap())
	} else {
		Err(MicrosoftLoginError {
			error_type: MicrosoftLoginErrorType::APIError,
			message: res.text().await.unwrap(),
		})
	}
}

async fn get_minecraft_token(
	client: &reqwest::Client,
	xsts_token: &str,
	uhs: &str,
) -> Result<(String, DateTime<Utc>), MicrosoftLoginError> {
	let json_data = json!({
		"identityToken": format!("XBL3.0 x={};{}", uhs, xsts_token),
	});

	let res = match client
		.post("https://api.minecraftservices.com/authentication/login_with_xbox")
		.json(&json_data)
		.header("Content-Type", "application/json")
		.header("Accept", "application/json")
		.send()
		.await
	{
		Ok(res) => res,
		Err(err) => {
			return Err(MicrosoftLoginError {
				error_type: MicrosoftLoginErrorType::NetworkError,
				message: err.to_string(),
			});
		}
	};

	if res.status().is_success() {
		let body: HashMap<String, Value> = match res.json().await {
			Ok(body) => body,
			Err(err) => {
				return Err(MicrosoftLoginError {
					error_type: MicrosoftLoginErrorType::APIError,
					message: err.to_string(),
				});
			}
		};
		let token = body.get("access_token").unwrap().to_string();
		match body.get("expires_in") {
			Some(expires_in) => {
				let expires_in = expires_in.as_i64().unwrap();
				let expires_at = Utc::now() + Duration::seconds(expires_in);
				Ok((token[1..token.len() - 1].parse().unwrap(), expires_at))
			}
			None => Err(MicrosoftLoginError {
				error_type: MicrosoftLoginErrorType::APIError,
				message: "expires_in do not exist".to_string(),
			}),
		}
	} else {
		Err(MicrosoftLoginError {
			error_type: MicrosoftLoginErrorType::APIError,
			message: res.text().await.unwrap(),
		})
	}
}

async fn get_minecraft_profile(
	client: &reqwest::Client,
	minecraft_token: &str,
) -> Result<(String, String), MicrosoftLoginError> {
	let res = match client
		.get("https://api.minecraftservices.com/minecraft/profile")
		.header("Authorization", format!("Bearer {}", minecraft_token))
		.send()
		.await
	{
		Ok(res) => res,
		Err(err) => {
			return Err(MicrosoftLoginError {
				error_type: MicrosoftLoginErrorType::NetworkError,
				message: err.to_string(),
			});
		}
	};

	if res.status().is_success() {
		let body: HashMap<String, Value> = match res.json().await {
			Ok(body) => body,
			Err(err) => {
				return Err(MicrosoftLoginError {
					error_type: MicrosoftLoginErrorType::APIError,
					message: err.to_string(),
				});
			}
		};
		if body.contains_key("error") {
			return Err(MicrosoftLoginError {
				error_type: MicrosoftLoginErrorType::ProfileNotFoundError,
				message: "profile not found".to_string(),
			});
		}
		let uuid = body.get("id").unwrap().to_string();
		let name = body.get("name").unwrap().to_string();

		Ok((
			uuid[1..uuid.len() - 1].parse().unwrap(),
			name[1..name.len() - 1].parse().unwrap(),
		))
	} else {
		Err(MicrosoftLoginError {
			error_type: MicrosoftLoginErrorType::APIError,
			message: res.text().await.unwrap(),
		})
	}
}
