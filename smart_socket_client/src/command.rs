pub enum Command {
    TurnOn,
    TurnOff,
    Info,
    Unknown,
}

impl From<Command> for u8 {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::TurnOn => 0,
            Command::TurnOff => 1,
            Command::Info => 2,
            Command::Unknown => 255,
        }
    }
}
