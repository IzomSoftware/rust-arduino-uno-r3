#[allow(dead_code)]
pub enum Type {
    Analog,
    Digital,
}

#[allow(dead_code)]
pub enum Mode {
    Input,
    Output,
}

impl Mode {
    pub fn get_val(&self) -> bool {
        match self {
            Mode::Input => false,
            Mode::Output => true,
        }
    }
}

#[allow(dead_code)]
pub struct Pin {
    pub pin: u8,
}
