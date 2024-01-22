#[derive(Debug, Default)]
pub struct SmartSocket {
    pub _description: String,
    pub _is_enabled: bool,
    pub _power: f64,
}

impl SmartSocket {
    fn _enable(&mut self) {
        self._is_enabled = true
    }

    fn _disable(&mut self) {
        self._is_enabled = false
    }

    fn _get_description(self) -> String {
        self._description
    }

    fn _is_enabled(&self) -> bool {
        self._is_enabled
    }

    fn _get_power(&self) -> f64 {
        self._power
    }
}

#[cfg(test)]
mod tests {
    use crate::house::devices::smart_socket::SmartSocket;

    #[test]
    fn should_enable_socket() {
        let mut socket = SmartSocket {
            _description: "socket".to_string(),
            _is_enabled: false,
            _power: 10.0,
        };

        socket._enable();

        assert_eq!(socket._is_enabled(), true);
    }

    #[test]
    fn should_disable_socket() {
        let mut socket = SmartSocket {
            _description: "socket".to_string(),
            _is_enabled: false,
            _power: 10.0,
        };

        socket._disable();

        assert_eq!(socket._is_enabled(), false);
    }
}
