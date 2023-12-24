fn main() {}

#[derive(Debug)]
pub struct _SmartOutlet<'a> {
    _description: &'a str,
    _is_enabled: &'a bool,
    _power: &'a f64,
}

#[derive(Debug)]
pub struct _Thermometer<'a> {
    _current_temperature: &'a f64,
}

impl _SmartOutlet<'_> {
    fn _enable(&mut self) {
        self._is_enabled = &true
    }

    fn _disable(&mut self) {
        self._is_enabled = &false
    }

    fn _get_description(&self) -> &str {
        self._description
    }

    fn _is_enabled(&self) -> &bool {
        self._is_enabled
    }

    fn _get_power(&self) -> &f64 {
        self._power
    }
}

impl _Thermometer<'_> {
    fn _get_current_temperature(&self) -> &f64 {
        self._current_temperature
    }
}
