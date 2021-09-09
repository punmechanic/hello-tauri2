#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod podman_api;

use podman_api::Client;

fn main() {
  let client = Client::new();
  let containers = client.list_containers().expect("oops");
  println!("{:?}", containers);

  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
