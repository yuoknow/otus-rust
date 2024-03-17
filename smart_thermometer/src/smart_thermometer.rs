use std::io::Error;
use std::net::{ToSocketAddrs, UdpSocket};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Debug, Default)]
pub struct SmartThermometer {
    pub temperature: Arc<AtomicU32>,
}

impl SmartThermometer {
    pub fn new(addr: impl ToSocketAddrs) -> Result<SmartThermometer, Error> {
        let udp_socket = UdpSocket::bind(addr)?;
        udp_socket.set_read_timeout(Option::from(Duration::from_secs(1)))?;
        let temperature = Arc::new(AtomicU32::default());
        let temperature_clone = temperature.clone();
        thread::spawn(move || loop {
            let mut buf = [0u8; 256];
            if let Err(err) = udp_socket.recv_from(&mut buf) {
                println!("cant receive datagram {}", err);
                continue;
            }
            let data_size = buf[0];
            if data_size != 4 {
                println!("wrong data size expected 4 actual {}", data_size)
            } else {
                let mut temperature_bytes = [0u8; 4];
                let end_index = data_size + 1;
                temperature_bytes.copy_from_slice(&buf[1..end_index.into()]);
                let as_u32 = f32::from_be_bytes(temperature_bytes).to_bits();
                temperature_clone.store(as_u32, Ordering::Relaxed);
            }
        });

        Ok(Self { temperature })
    }

    pub fn get_temperature(&self) -> f32 {
        f32::from_bits(self.temperature.load(Ordering::Relaxed))
    }
}
