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
    button: button::Button,
    button_state: Vec<bool>,
    button_pressed: Vec<bool>,
    button_count: [i32; 7],
    scratch: scratch::Scratch,
    scratch_state: f64,
    scratch_activated: bool,
    scratch_count: i32,
}

impl Controller {
    pub fn new() -> Self {
        // コンストラクタ
        Self {
            controller: None,
            button: button::Button::new(),
            button_state: vec![false; 16],
            button_pressed: vec![false; 16],
            button_count: [0; 7],
            scratch: scratch::Scratch::new(),
            scratch_state: 0.0,
            scratch_activated: false,
            scratch_count: 0,
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
        self.button_pressed = self.button.check_pressed(self.button_state.clone());
        self.scratch_activated = self.scratch.check_input(self.scratch_state);
    }

    pub fn get_button_count(&self) -> [i32; 7] {
        self.button_count.clone()
    }

    pub fn get_scratch_count(&self) -> i32 {
        self.scratch_count
    }

    pub fn update_count(&mut self) -> () {
        // ボタンカウントの更新
        for i in 0..7 {
            if self.button_pressed[i] {
                self.button_count[i] += 1;
            }
        }

        // スクラッチカウントの更新
        if self.scratch_activated {
            self.scratch_count += 1;
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
}
