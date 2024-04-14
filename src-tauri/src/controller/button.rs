#[derive(Debug)]
pub struct Button {
    // 1つ前のボタンの状態
    prev_button_state: Vec<bool>,
}

impl Button {
    pub fn new() -> Self {
        Self {
            prev_button_state: vec![false; 16],
        }
    }

    pub fn check_pressed(&mut self, button_state: Vec<bool>) -> Vec<bool> {
        let mut button_pressed = vec![false; 16];

        for i in 0..16 {
            if button_state[i] && !self.prev_button_state[i] {
                button_pressed[i] = true;
            }
        }

        self.prev_button_state = button_state.clone();

        return button_pressed;
    }
}
