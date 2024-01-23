use crate::house::devices::smart_socket::SmartSocket;
use crate::house::devices::smart_thermometer::SmartThermometer;

pub enum Device {
    SmartSocket(SmartSocket),
    SmartThermometer(SmartThermometer),
}

impl Device {
    pub fn status(&self) -> String {
        match self {
            Device::SmartThermometer(state) => format!("{:?}", state),
            Device::SmartSocket(state) => format!("{:?}", state),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::house::devices::device::Device;
    use crate::house::devices::smart_socket::SmartSocket;
    use crate::house::devices::smart_thermometer::SmartThermometer;

    #[test]
    fn should_show_socket_status() {
        let socket = SmartSocket {
            _description: "socket".to_string(),
            _is_enabled: false,
            _power: 10.0,
        };

        assert_eq!(
            Device::SmartSocket(socket).status(),
            "SmartSocket { _description: \"socket\", _is_enabled: false, _power: 10.0 }"
        );
    }

    #[test]
    fn should_show_thermo_status() {
        let thermo = SmartThermometer {
            _current_temperature: -10.0,
        };

        assert_eq!(
            Device::SmartThermometer(thermo).status(),
            "SmartThermometer { _current_temperature: -10.0 }"
        );
    }
}
