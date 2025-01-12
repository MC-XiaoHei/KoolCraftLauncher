use std::env;

fn main() {
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-env=VERSION={}", version);
    tauri_build::build()
}
