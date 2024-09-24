// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rdev::{listen, EventType};
use tauri::{self, Manager};


#[derive(Clone, serde::Serialize)]
struct Payload {
    key: String,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();

            std::thread::spawn(move || {
                listen(move |event| match event.event_type {
                    EventType::KeyPress(key) => {
                        let key_name = format!("{:?}", key);
                        handle
                            .emit_all("key-down", Payload { key: key_name })
                            .unwrap()
                    }
                    EventType::KeyRelease(key) => {
                        let key_name = format!("{:?}", key);
                        handle
                            .emit_all("key-up", Payload { key: key_name })
                            .unwrap()
                    }
                    _ => {}
                })
                .unwrap();
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
