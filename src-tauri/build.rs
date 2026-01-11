fn main() {
    // Only run tauri_build when building the full Tauri app
    // Skip in headless mode to allow CI testing without GUI dependencies
    #[cfg(feature = "tauri-app")]
    tauri_build::build();
}
