// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;
use std::time::Duration;
// use std::{process::exit, sync::mpsc};

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

struct Scratch {
    // 1つ前のスクラッチの値
    prev_value: f32,
    // 1つ前のスクラッチのアクティブ状態
    prev_active: bool,
    // 1つ前のスクラッチの方向
    prev_dir: bool,
    // 停止検知カウンタ
    counter: i32,
    // カウンタのリミット
    counter_limit: i32,
    // 初期化フラグ
    initialized: bool,
}

impl Scratch {
    pub fn new() -> Self {
        // TODO: counterは設定可能に
        Self {
            prev_value: 0.0,
            prev_active: false,
            prev_dir: false,
            counter: 0,
            counter_limit: 100,
            initialized: false,
        }
    }

    pub fn check_input(&mut self, value: f32) -> bool {
        let scratch_value: f32 = value;
        let mut scratch_active = self.prev_active;
        let mut scratch_dir = self.prev_dir;

        let mut scratch_started = false;

        if !self.initialized {
            self.prev_value = scratch_value;
            self.initialized = true;
            return false;
        }

        if scratch_value != self.prev_value {
            scratch_active = true;
            self.counter = 0;

            let scratch_diff = scratch_value - self.prev_value;
            if scratch_diff.abs() > 0.8 {
                if scratch_diff < 0.0 {
                    scratch_dir = true;
                } else {
                    scratch_dir = false;
                }
            } else {
                if scratch_diff > 0.0 {
                    scratch_dir = true;
                } else {
                    scratch_dir = false;
                }
            }
        } else {
            self.counter += 1;

            if self.counter > self.counter_limit {
                scratch_active = false;
            }
        }

        if (scratch_active && !self.prev_active) || (scratch_dir != self.prev_dir) {
            scratch_started = true;
        }

        self.prev_value = scratch_value;
        self.prev_active = scratch_active;
        self.prev_dir = scratch_dir;

        return scratch_started;
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Rust→Viewの通信用ハンドル
            let app_handle = app.app_handle();

            // コントローラー接続
            let controller: RawGameController = connect_wait().unwrap();

            // 100マイクロ秒ごとにコントローラーの状態を取る
            // ボタンとスクラッチの状態と押下・回転開始をViewに送る
            thread::spawn(move || {
                let mut scratch = Scratch::new();

                let prev_button_state = &mut [false; 16];
                let button_state = &mut [false; 16];
                let switch_state = &mut [GameControllerSwitchPosition::Center];
                let axis_state = &mut [0.0 as f64; 2];

                let mut button_counts = [0; 7];
                let mut scratch_count = 0;

                let mut is_pause = false;
                let mut is_pause_toggled = false;

                loop {
                    let _buttons = RawGameController::GetCurrentReading(
                        &controller,
                        button_state,
                        switch_state,
                        axis_state,
                    )
                    .unwrap();

                    // ボタンの状態をViewに送る
                    app_handle
                        .emit_all("buttonState", &button_state[0..7])
                        .unwrap();

                    if !is_pause {
                        // 押下されたボタンをViewに送る
                        for i in 0..button_counts.len() {
                            if button_state[i] && !prev_button_state[i] {
                                button_counts[i] += 1;
                            }
                        }

                        // ボタンのカウントをViewに送る
                        app_handle
                            .emit_all("buttonCounter", &button_counts)
                            .unwrap();
                    }

                    // スクラッチの状態をViewに送る
                    app_handle.emit_all("scratchState", &axis_state[0]).unwrap();

                    if !is_pause {
                        // スクラッチが回転開始の場合Viewに送る
                        if scratch.check_input(axis_state[0] as f32) {
                            scratch_count += 1;
                        }

                        // スクラッチのカウントをViewに送る
                        app_handle.emit_all("scratchCount", scratch_count).unwrap();
                    }

                    // E1 + E4でリセット
                    if button_state[8] && button_state[11] {
                        for i in 0..button_counts.len() {
                            button_counts[i] = 0;
                        }
                        scratch_count = 0;
                    }

                    // E3 + E4でポーズ(カウントを増やさない)
                    if button_state[10] && button_state[11] {
                        if !is_pause_toggled {
                            is_pause = !is_pause;
                            is_pause_toggled = true;
                            app_handle.emit_all("togglePause", is_pause).unwrap();
                        }
                    } else {
                        is_pause_toggled = false;
                    }

                    prev_button_state.copy_from_slice(button_state);

                    thread::sleep(Duration::from_micros(500));
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
