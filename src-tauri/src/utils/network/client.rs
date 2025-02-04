use lazy_static::lazy_static;
use tauri_plugin_http::reqwest::Client;
use tauri_plugin_http::reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};

lazy_static!{
	pub static ref HTTP_CLIENT: Client = {
		let mut headers = HeaderMap::new();
		let version = env!("CARGO_PKG_VERSION").to_string();
		let ua_text = format!("KCl/{}", version);
		let user_agent = HeaderValue::from_str(ua_text.as_str()).unwrap();
		headers.insert(USER_AGENT, user_agent);

		Client::builder()
			.default_headers(headers)
			.build()
			.expect("Failed to create HTTP client")
	};
}