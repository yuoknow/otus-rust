#[derive(Debug, Default)]
pub struct SmartThermometer {
    pub _current_temperature: f64,
}

impl SmartThermometer {
    fn _get_current_temperature(&self) -> f64 {
        self._current_temperature
    }
}
