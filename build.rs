const COMMANDS: &[&str] = &["makeToast"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .build();
}
