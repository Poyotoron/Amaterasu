// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod controller;

use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Manager};

fn controller_loop(app_handle: &AppHandle) {
    let mut controller = controller::Controller::new();
    controller.connect_wait(0);
    controller.init();

    // リセットフラグ
    let mut is_reset_pressed = false;

    // ポーズフラグ
    let mut is_pause = false;
    let mut is_pause_toggled = false;

    // 2Pフラグ
    let mut is_2p = false;
    let mut is_2p_toggled = false;

    // セーブ
    let mut is_save_pressed = false;

    // スクラッチ状態の送信間隔
    let scratch_send_interval = 40;
    let mut scratch_send_counter = 0;

    // 初期カウントの送信
    app_handle
        .emit_all("buttonCounter", controller.get_button_count())
        .unwrap();
    app_handle
        .emit_all("scratchCount", controller.get_scratch_count())
        .unwrap();

    // メインループ
    loop {
        // コントローラーの状態を更新
        controller.update_state();

        scratch_send_counter += 1;

        // コントローラーの状態をViewに送信
        if controller.get_button_diff() {
            app_handle
                .emit_all("buttonState", controller.get_button_state())
                .expect("failed to send controller button state");
        }
        if scratch_send_counter >= scratch_send_interval {
            app_handle
                .emit_all("scratchState", controller.get_scratch_state())
                .expect("failed to send controller scratch state");
            scratch_send_counter = 0;
        }

        // 非ポーズ中のみカウントの更新と送信を行う
        if !is_pause {
            controller.update_count();

            if controller.get_button_count_diff() {
                app_handle
                    .emit_all("buttonCounter", controller.get_button_count())
                    .unwrap();
            }
            if controller.get_scratch_count_diff() {
                app_handle
                    .emit_all("scratchCount", controller.get_scratch_count())
                    .unwrap();
            }
        }

        // E1 + E4でカウントのリセット
        if controller.button_pressed_all(vec![
            controller::Button::KEYE1 as i32,
            controller::Button::KEYE4 as i32,
        ]) {
            if !is_reset_pressed {
                controller.reset_count();
                app_handle
                    .emit_all("buttonCounter", controller.get_button_count())
                    .unwrap();
                app_handle
                    .emit_all("scratchCount", controller.get_scratch_count())
                    .unwrap();
                is_reset_pressed = true;
            }
        } else {
            is_reset_pressed = false;
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

        // E2 + 2 + 6でプレイサイドのトグル
        if controller.button_pressed_all(vec![
            controller::Button::KEYE2 as i32,
            controller::Button::KEY2 as i32,
            controller::Button::KEY6 as i32,
        ]) {
            if !is_2p_toggled {
                is_2p = !is_2p;
                is_2p_toggled = true;
                app_handle.emit_all("toggle2P", is_2p).unwrap();
            }
        } else {
            is_2p_toggled = false;
        }

        // E2 + 4でセーブ
        if controller.button_pressed_all(vec![
            controller::Button::KEYE2 as i32,
            controller::Button::KEY4 as i32,
        ]) {
            if !is_save_pressed {
                controller.save_count();
                is_save_pressed = true;
                app_handle.emit_all("savedCount", true).unwrap();
            }
        } else {
            is_save_pressed = false;
        }

        // 500us待機
        thread::sleep(Duration::from_micros(500));
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
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
