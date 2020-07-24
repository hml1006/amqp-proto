
#[derive(Clone, Copy)]
pub enum Class {
    Connection,
    Channel,
    Access,
    Exchange,
    Queue,
    Basic,
    Tx,
    Confirm,
    Unknown
}

impl Class {
    pub fn class_id(&self) -> u16 {
        match self {
            Class::Connection => 10,
            Class::Channel => 20,
            Class::Access => 30,
            Class::Exchange => 40,
            Class::Queue => 50,
            Class::Basic => 60,
            Class::Confirm => 85,
            Class::Tx => 90,
            Class::Unknown => 0xffff
        }
    }
}

impl From<u16> for Class {
    fn from(class_id: u16) -> Self {
        match class_id {
            10 => Class::Connection,
            20 => Class::Channel,
            30 => Class::Access,
            40 => Class::Exchange,
            50 => Class::Queue,
            60 => Class::Basic,
            85 => Class::Confirm,
            90 => Class::Tx,
            _  => Class::Unknown
        }
    }
}

impl Default for Class {
    fn default() -> Self {
        Class::Unknown
    }
}
