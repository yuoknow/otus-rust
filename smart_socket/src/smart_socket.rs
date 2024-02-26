#[derive(Debug, Default, Copy, Clone)]
pub struct SmartSocket {
    pub is_enabled: bool,
    pub power: f32,
}

impl SmartSocket {
    pub fn enable(&mut self) {
        self.is_enabled = true;
        self.power = 3.5;
    }

    pub fn disable(&mut self) {
        self.is_enabled = false;
        self.power = 0.0;
    }

    fn _is_enabled(&self) -> bool {
        self.is_enabled
    }

    fn _get_power(&self) -> f32 {
        self.power
    }
}
