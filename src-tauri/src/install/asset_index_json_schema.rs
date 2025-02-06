#![allow(unused_variables)] // TODO: remove this

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct AssetIndexJson {
	pub objects: HashMap<String, AssetObject>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AssetObject {
	pub hash: String,
	pub size: u64,
}
