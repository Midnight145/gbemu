pub mod interrupts;

pub struct Serial {
    pub data: u8,
    pub status: u8
}

impl Serial {
    pub fn new() -> Serial {
        Serial {
            data: 0,
            status: 0
        }
    }
}

pub struct Joypad {
    a_right: bool,
    b_left: bool,
    select_up: bool,
    start_down: bool,

    dpad_select: bool,
    button_select: bool,
    unk6: u8,
    unk7: u8,
}

impl Joypad {
    pub fn new() -> Joypad{
        Joypad {
            a_right: false,
            b_left: false,
            select_up: false,
            start_down: false,
            dpad_select: false,
            button_select: false,
            unk6: 0,
            unk7: 0
        }
    }

    pub fn write_joypad(&mut self, value: u8) {
        self.unk7 = value & 0x80;
        self.unk6 = value & 0x40;
        self.button_select = value & 0x20 != 0;
    }

    pub fn read_joypad(&self) -> u8 {
        self.unk7 << 7 |
        self.unk6 << 6 |
        (self.button_select as u8) << 5 |
        (self.dpad_select as u8) << 4 |
        (self.start_down as u8) << 3 |
        (self.select_up as u8) << 2 |
        (self.b_left as u8) << 1 |
        (self.a_right as u8)
    }
}