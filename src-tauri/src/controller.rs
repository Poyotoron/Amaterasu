use std::{iter::Sum, path::Path};

use chrono::Utc;

use windows::Gaming::Input::{GameControllerSwitchPosition, RawGameController};

mod button;
mod scratch;

pub enum Button {
    KEY1 = 0,
    KEY2 = 1,
    KEY3 = 2,
    KEY4 = 3,
    KEY5 = 4,
    KEY6 = 5,
    KEY7 = 6,
    KEYE1 = 8,
    KEYE2 = 9,
    KEYE3 = 10,
    KEYE4 = 11,
}

pub struct Controller {
    controller: Option<RawGameController>,
    date_id: String,
    csv_name: String,
    button: button::Button,
    button_state: Vec<bool>,
    button_pressed: Vec<bool>,
    button_count: [i32; 7],
    button_diff: bool,
    button_count_diff: bool,
    scratch: scratch::Scratch,
    scratch_state: f64,
    scratch_activated: bool,
    scratch_count: i32,
    scratch_diff: bool,
    scratch_count_diff: bool,
}

impl Controller {
    pub fn new() -> Self {
        // コンストラクタ
        Self {
            controller: None,
            date_id: Utc::now().format("%F").to_string(),
            csv_name: "key_counter.csv".to_string(),
            button: button::Button::new(),
            button_state: vec![false; 16],
            button_pressed: vec![false; 16],
            button_count: [0; 7],
            button_diff: false,
            button_count_diff: false,
            scratch: scratch::Scratch::new(),
            scratch_state: 0.0,
            scratch_activated: false,
            scratch_count: 0,
            scratch_diff: false,
            scratch_count_diff: false,
        }
    }

    fn connect(&mut self, id: i32) -> Result<RawGameController, &'static str> {
        // コントローラーの接続
        // 起動直後はコントローラーが認識されないことがあるため、接続待ちを行う
        let controller = RawGameController::RawGameControllers().unwrap();
        if controller.Size().unwrap() > id as u32 {
            Ok(controller.GetAt(id as u32).unwrap())
        } else {
            Err("No controller found")
        }
    }

    pub fn connect_wait(&mut self, id: i32) -> bool {
        // コントローラーの接続待ち
        // TODO: タイムアウト処理を追加
        loop {
            match self.connect(id) {
                Ok(controller) => {
                    self.controller = Some(controller);
                    return true;
                }
                Err(_) => std::thread::sleep(std::time::Duration::from_secs(1)),
            }
        }
    }

    pub fn init(&mut self) -> () {
        // 初期化
        self.init_count();
    }

    pub fn get_button_state(&self) -> Vec<bool> {
        self.button_state.clone()
    }

    pub fn get_scratch_state(&self) -> f64 {
        self.scratch_state
    }

    pub fn update_state(&mut self) -> () {
        let buttonarray = &mut [false; 16];
        let switcharray = &mut [GameControllerSwitchPosition::Center; 8];
        let axisarray = &mut [0.0 as f64; 2];

        if let Some(controller) = &mut self.controller {
            let _ = RawGameController::GetCurrentReading(
                controller,
                buttonarray,
                switcharray,
                axisarray,
            );
        }

        // stateに代入
        self.button_state = buttonarray.to_vec();
        self.scratch_state = axisarray[0];

        // ボタン・スクラッチの作動状態を更新
        (self.button_pressed, self.button_diff) =
            self.button.check_pressed(self.button_state.clone());
        (self.scratch_activated, self.scratch_diff) = self.scratch.check_input(self.scratch_state);
    }

    pub fn init_count(&mut self) -> () {
        if Path::new(&self.csv_name).exists() {
            // ファイルが存在する場合は読み込み
            let mut rdr = csv::Reader::from_path(&self.csv_name).unwrap();
            let mut last_row = None;
            for record in rdr.records() {
                last_row = record.ok();
            }

            if let Some(row) = last_row {
                if row[0] == self.date_id {
                    self.button_count = [
                        row[1].parse::<i32>().unwrap(),
                        row[2].parse::<i32>().unwrap(),
                        row[3].parse::<i32>().unwrap(),
                        row[4].parse::<i32>().unwrap(),
                        row[5].parse::<i32>().unwrap(),
                        row[6].parse::<i32>().unwrap(),
                        row[7].parse::<i32>().unwrap(),
                    ];
                    self.scratch_count = row[9].parse::<i32>().unwrap();
                }
            }
        }
    }

    pub fn save_count(&mut self) -> () {
        // カウントの保存
        // 最終行とdate_idが一致する場合は上書き、一致しない場合は追記
        if Path::new(&self.csv_name).exists() {
            let mut rdr = csv::Reader::from_path(&self.csv_name).unwrap();
            let csv_data = rdr.records().collect::<Vec<_>>();

            let mut wtr = csv::Writer::from_path(&self.csv_name).unwrap();
            wtr.write_record(&[
                "date_id",
                "key1",
                "key2",
                "key3",
                "key4",
                "key5",
                "key6",
                "key7",
                "key_total",
                "scratch",
            ])
            .unwrap();

            for record in csv_data {
                let r = record.unwrap();
                if r[0] != self.date_id {
                    wtr.write_record(&r).unwrap();
                }
            }

            wtr.write_record(&[
                &self.date_id,
                &self.button_count[0].to_string(),
                &self.button_count[1].to_string(),
                &self.button_count[2].to_string(),
                &self.button_count[3].to_string(),
                &self.button_count[4].to_string(),
                &self.button_count[5].to_string(),
                &self.button_count[6].to_string(),
                &self.button_count.iter().sum::<i32>().to_string(),
                &self.scratch_count.to_string(),
            ])
            .unwrap();

            wtr.flush().unwrap();
        } else {
            let mut wtr = csv::Writer::from_path(&self.csv_name).unwrap();
            wtr.write_record(&[
                "date_id",
                "key1",
                "key2",
                "key3",
                "key4",
                "key5",
                "key6",
                "key7",
                "key_total",
                "scratch",
            ])
            .unwrap();
            wtr.write_record(&[
                &self.date_id,
                &self.button_count[0].to_string(),
                &self.button_count[1].to_string(),
                &self.button_count[2].to_string(),
                &self.button_count[3].to_string(),
                &self.button_count[4].to_string(),
                &self.button_count[5].to_string(),
                &self.button_count[6].to_string(),
                &self.button_count.iter().sum::<i32>().to_string(),
                &self.scratch_count.to_string(),
            ])
            .unwrap();
            wtr.flush().unwrap();
        }
    }

    pub fn get_button_count(&self) -> [i32; 7] {
        self.button_count.clone()
    }

    pub fn get_scratch_count(&self) -> i32 {
        self.scratch_count
    }

    pub fn update_count(&mut self) -> () {
        self.button_count_diff = false;
        self.scratch_count_diff = false;

        // ボタンカウントの更新
        for i in 0..7 {
            if self.button_pressed[i] {
                self.button_count[i] += 1;
                self.button_count_diff = true;
            }
        }

        // スクラッチカウントの更新
        if self.scratch_activated {
            self.scratch_count += 1;
            self.scratch_count_diff = true;
        }
    }

    pub fn reset_count(&mut self) -> () {
        self.button_count = [0; 7];
        self.scratch_count = 0;
    }

    pub fn button_pressed(&self, buttons: Vec<i32>) -> Vec<bool> {
        let mut result = vec![false; buttons.len()];
        for (i, button) in buttons.iter().enumerate() {
            if self.button_state[*button as usize] {
                result[i] = true;
            }
        }
        result
    }

    pub fn button_pressed_all(&self, buttons: Vec<i32>) -> bool {
        for button in buttons.iter() {
            if !self.button_state[*button as usize] {
                return false;
            }
        }
        true
    }

    pub fn get_button_diff(&self) -> bool {
        self.button_diff
    }

    pub fn get_button_count_diff(&self) -> bool {
        self.button_count_diff
    }

    pub fn get_scratch_diff(&self) -> bool {
        self.scratch_diff
    }

    pub fn get_scratch_count_diff(&self) -> bool {
        self.scratch_count_diff
    }
}
