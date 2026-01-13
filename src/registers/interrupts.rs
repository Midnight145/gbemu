enum Interrupt {
    VBLANK = 0b1,
    LCD =    0b10,
    TIMER =  0b100,
    SERIAL = 0b1000,
    JOYPAD = 0b10000,
    UN5 =    0b100000,
    UN6 =    0b1000000,
    UN7 =    0b10000000
}

pub const ADDRESS: u16 = 0xFF0F;

pub struct Interrupts {
    pub vblank: bool,
    pub lcd: bool,
    pub timer: bool,
    pub serial: bool,
    pub joypad: bool,
    pub un5: bool,
    pub un6: bool,
    pub un7: bool
}

impl Interrupts {
    pub fn new() -> Interrupts {
        Interrupts {
            vblank: false,
            lcd: false,
            timer: false,
            serial: false,
            joypad: false,
            un5: false,
            un6: false,
            un7: false,
        }
    }

    pub fn write_interrupts(&mut self, byte: u8) {
        self.vblank = byte & 0b1 == 0b1;
        self.lcd = byte & 0b10 == 0b10;
        self.timer = byte & 0b100 == 0b100;
        self.serial = byte & 0b1000 == 0b1000;
        self.joypad = byte & 0b10000 == 0b10000;
        self.un5 = byte & 0b100000 == 0b100000;
        self.un6 = byte & 0b1000000 == 0b1000000;
        self.un7 = byte & 0b10000000 == 0b10000000;
    }

    pub fn read_interrupts(&self) -> u8 {
        (self.un7 as u8) << 7 | (self.un6 as u8) << 6 |
            (self.un5 as u8) << 5 | (self.joypad as u8) << 4 |
            (self.serial as u8) << 3 | (self.timer as u8) << 2 |
            (self.lcd as u8) << 1 | (self.vblank as u8)
    }


}