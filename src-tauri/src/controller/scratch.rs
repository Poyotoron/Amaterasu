#[derive(Debug)]
pub struct Scratch {
    // 1つ前のスクラッチの値
    prev_value: f64,
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

    pub fn check_input(&mut self, value: f64) -> (bool, bool) {
        let scratch_value: f64 = value;
        let mut scratch_active = self.prev_active;
        let mut scratch_dir = self.prev_dir;

        let mut scratch_started = false;

        let mut scratch_diff = false;

        if !self.initialized {
            self.prev_value = scratch_value;
            self.initialized = true;
            return (false, true);
        }

        if scratch_value != self.prev_value {
            scratch_active = true;
            scratch_diff = true;
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

        return (scratch_started, scratch_diff);
    }
}
