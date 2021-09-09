#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod podman_api;

fn main() {
  podman_api::hello_world();

  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
