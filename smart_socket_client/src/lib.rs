#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct SmartSocket {
    enabled: bool,
    power: f32,
}

#[link(name = "smart_socket", kind = "dylib")]
extern "C" {
    fn enable(socket: SmartSocket) -> SmartSocket;
    fn disable(socket: SmartSocket) -> SmartSocket;
}

impl SmartSocket {
    pub fn enable(&mut self) {
        let smart_socket = unsafe { enable(*self) };
        *self = SmartSocket { ..smart_socket };
    }

    pub fn disable(&mut self) {
        let smart_socket = unsafe { disable(*self) };
        *self = SmartSocket { ..smart_socket };
    }
}
