//! The app's name and id, `[package.metadata.app]` in `Cargo.toml`, as the compile-time
//! variables `APP_NAME` and `APP_ID`. Gradle and the `justfile` read the same two lines.

fn main() {
    println!("cargo::rerun-if-changed=Cargo.toml");
    let manifest = std::fs::read_to_string("Cargo.toml").expect("Cargo.toml");
    let app = manifest
        .split_once("[package.metadata.app]")
        .expect("[package.metadata.app] in Cargo.toml")
        .1;
    for (key, variable) in [("name", "APP_NAME"), ("id", "APP_ID")] {
        let value = app
            .lines()
            .find_map(|line| line.strip_prefix(&format!("{key} = \""))?.strip_suffix('"'))
            .unwrap_or_else(|| panic!("`{key} = \"..\"` under [package.metadata.app]"));
        println!("cargo::rustc-env={variable}={value}");
    }
}
