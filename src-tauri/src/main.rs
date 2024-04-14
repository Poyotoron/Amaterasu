// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod controller;

use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Manager};

fn controller_loop(app_handle: &AppHandle) {
    let mut controller = controller::Controller::new();
    controller.connect_wait(0);

    // ポーズフラグ
    let mut is_pause = false;
    let mut is_pause_toggled = false;

    // メインループ
    loop {
        // コントローラーの状態を更新
        controller.update_state();

        // コントローラーの状態をViewに送信
        app_handle
            .emit_all("buttonState", controller.get_button_state())
            .expect("failed to send controller button state");
        app_handle
            .emit_all("scratchState", controller.get_scratch_state())
            .expect("failed to send controller scratch state");

        // 非ポーズ中のみカウントの更新と送信を行う
        if !is_pause {
            controller.update_count();

            app_handle
                .emit_all("buttonCounter", controller.get_button_count())
                .unwrap();
            app_handle
                .emit_all("scratchCount", controller.get_scratch_count())
                .unwrap();
        }

        // E1 + E4でカウントのリセット
        if controller.button_pressed_all(vec![
            controller::Button::KEYE1 as i32,
            controller::Button::KEYE4 as i32,
        ]) {
            controller.reset_count();
        }

        // E3 + E4でポーズのトグル
        if controller.button_pressed_all(vec![
            controller::Button::KEYE3 as i32,
            controller::Button::KEYE4 as i32,
        ]) {
            if !is_pause_toggled {
                is_pause = !is_pause;
                is_pause_toggled = true;
                app_handle.emit_all("togglePause", is_pause).unwrap();
            }
        } else {
            is_pause_toggled = false;
        }

        // 500us待機
        thread::sleep(Duration::from_micros(500));
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Rust→Viewの通信用ハンドル
            let app_handle = app.app_handle();

            // コントローラー周りのループ
            thread::spawn(move || controller_loop(&app_handle));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
