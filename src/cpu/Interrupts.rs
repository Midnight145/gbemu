pub enum Interrupt {
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