#![allow(non_snake_case)]

pub struct MBC {
    ROM: Vec<u8>,
    ERAM: Vec<u8>,
    bank: u8
}

impl MBC {
    pub fn new() -> MBC {
        MBC {
            ROM: vec![0; 0x8000],
            ERAM: vec![0; 0x2000],
            bank: 1,
        }
    }
    pub fn write_rom(&self, addr: u16, value: u8) {

    }

    pub fn read_rom(&self, addr: u16) -> u8 {
        let real_address = self.calculate_bank_addr(addr);
        self.ROM[real_address]
    }

    pub fn write_eram(&mut self, addr: u16, value: u8) {
        self.ERAM[(addr - 0xA000) as usize] = value
    }

    pub fn read_eram(&self, addr: u16)  -> u8 {
        self.ERAM[(addr - 0xA000) as usize]
    }

    fn calculate_bank_addr(&self, addr: u16) -> usize {
        if addr <= 0x3FFF {
            addr as usize // bank 0, offset = addr
        } else if addr <= 0x7FFF {
            let bank = if self.bank == 0 { 1 } else { self.bank }; // bank 0 cannot be selected
            return ((bank as usize) * 0x4000) + ((addr - 0x4000) as usize);
        } else {
            panic!("Address out of ROM range");
        }
    }
}