#[repr(C)]
pub struct SmartSocket {
    is_enabled: bool,
    power: f32,
}

#[no_mangle]
pub extern "C" fn enable(_socket: SmartSocket) -> SmartSocket {
    SmartSocket {
        is_enabled: true,
        power: 3.5,
    }
}

#[no_mangle]
pub extern "C" fn disable(_socket: SmartSocket) -> SmartSocket {
    SmartSocket {
        is_enabled: false,
        power: 0.0,
    }
}
