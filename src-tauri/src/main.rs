#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::time::{Duration, Instant}; 
use std::thread;

use discord_rpc_client::{Client, Event};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(serde::Deserialize)]
struct RpcData {
    application_id: String,
    details: String,
    state: String,
    large_image: String,
    large_text: String,
    small_image: String,
    small_text: String,
}

#[tauri::command]
fn update_discord_rpc(rpc_data: RpcData) {
    let mut drpc = Client::new(rpc_data.application_id.parse::<u64>().unwrap());

    drpc.on_ready(|_ctx| {
        println!("ready?");
    });


    drpc.on_event(Event::Ready, |_| {
        println!("READY!");
    });


    drpc.start();
    drpc.set_activity(|act| act
            .state(&rpc_data.state)
            .details(&rpc_data.details)
            // .timestamps(|t| t
            //     .start(Instant::now().elapsed().as_millis() as u64) 
            //     .end((Instant::now().elapsed() + Duration::from_secs(10)).as_millis() as u64)) 
            .assets(|ass| ass       
                .large_image(&rpc_data.large_image)
                .large_text(&rpc_data.large_text)
                .small_image(&rpc_data.small_image)
                .small_text(rpc_data.small_text.clone())
            )).expect("RPC 설정 에러");
       
            
    thread::sleep(Duration::from_secs(10));
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![update_discord_rpc])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
