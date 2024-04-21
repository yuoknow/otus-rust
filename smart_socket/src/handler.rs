use crate::smart_socket::SmartSocket;

pub struct RequestHandler {
    smart_socket: SmartSocket,
}

impl RequestHandler {
    pub fn new(smart_socket: SmartSocket) -> Self {
        Self { smart_socket }
    }

    pub fn handle(&mut self, request: u8) -> Response {
        match request {
            0 => {
                self.enable();
                Response::Ok
            }
            1 => {
                self.disable();
                Response::Ok
            }
            2 => Response::Info(self.smart_socket.is_enabled, self.smart_socket.power),
            _ => Response::Unknown,
        }
    }

    fn enable(&mut self) {
        self.smart_socket.enable();
    }

    fn disable(&mut self) {
        self.smart_socket.disable();
    }
}

#[derive(Debug)]
pub enum Response {
    Ok,
    Info(bool, f32),
    Unknown,
}

impl From<Response> for [u8; 5] {
    fn from(resp: Response) -> Self {
        let mut buffer = [0u8; 5];
        match resp {
            Response::Ok => {}
            Response::Info(enabled, pwr) => {
                buffer[0] = match enabled {
                    true => 1,
                    false => 2,
                };
                buffer[1..].copy_from_slice(&pwr.to_be_bytes())
            }
            Response::Unknown => buffer[0] = 255,
        };
        buffer
    }
}
