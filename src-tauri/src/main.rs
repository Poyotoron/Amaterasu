// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;
use std::time::Duration;
use std::{process::exit, sync::mpsc};

use tauri::Manager;
use windows::Gaming::Input::{GameControllerSwitchPosition, RawGameController};

// 接続中のコントローラーを取得して0番目を返す
fn connect() -> Result<RawGameController, &'static str> {
    let controller = RawGameController::RawGameControllers().unwrap();
    if controller.Size().unwrap() > 0 {
        Ok(controller.GetAt(0).unwrap())
    } else {
        Err("No controller found")
    }
}

// 動き始めはコントローラーを認識しないのでループで待つ
fn connect_wait() -> Result<RawGameController, &'static str> {
    loop {
        match connect() {
            Ok(controller) => return Ok(controller),
            Err(_) => std::thread::sleep(std::time::Duration::from_secs(1)),
        }
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.app_handle();

            // コントローラー接続
            let controller: RawGameController = connect_wait().unwrap();

            // 100マイクロ秒ごとにコントローラーのボタンの状態を取ってViewに送る
            std::thread::spawn(move || {
                let button_state = &mut [false; 16];
                let switch_state = &mut [GameControllerSwitchPosition::Center];
                let axis_state = &mut [0.0 as f64; 2];

                loop {
                    let _buttons = RawGameController::GetCurrentReading(
                        &controller,
                        button_state,
                        switch_state,
                        axis_state,
                    )
                    .unwrap();

                    app_handle.emit_all("buttonState", &button_state).unwrap();
                    app_handle.emit_all("scratchState", &axis_state[0]).unwrap();

                    std::thread::sleep(std::time::Duration::from_micros(100));
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
