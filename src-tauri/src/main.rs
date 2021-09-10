#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
extern crate hyper;
extern crate hyperlocal;
extern crate tokio;

mod podman_api;

use podman_api::{Client, Container};

#[tauri::command]
async fn podman_api<'a>(
  state: tauri::State<'a, Client>,
  command: String,
) -> Result<Vec<Container>, String> {
  if command == "fetch_containers" {
    state
      .list_containers()
      .await
      .or_else(|_| Err("oops".into()))
  } else {
    Err("unknown command".into())
  }
}

#[tokio::main]
async fn main() {
  let client = Client::new();

  tauri::Builder::default()
    .manage(client)
    .invoke_handler(tauri::generate_handler![podman_api])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
