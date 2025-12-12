use crate::cartridge::header::Header;
use crate::cartridge::mbc::MBC;

pub struct MBC1 {
    rom: Vec<u8>,
    eram: Vec<u8>,
    registers: Registers
}

impl MBC1 {
    pub(crate) fn new(header: &Header, rom: Vec<u8>) -> MBC1 {
        MBC1 {
            rom,
            eram: vec![0;header.ram_size as usize],
            registers: Registers {
                ram_enable: false,
                rom_bank_number: 1,
                ram_bank_number: 0,
                bank_mode_select: 0
            }
        }
    }

    fn calculate_rom_addr(addr: u16, bank: u8) -> usize {
        ((bank as u16 * 0x4000) + addr) as usize
    }

    fn calculate_ram_addr(addr: u16, bank: u8) -> usize {
        ((bank as u16 * 0x2000) + addr) as usize
    }
}

impl MBC for MBC1 {
    fn read_rom(&self, addr: u16) -> u8 {
        let mut bank = 0;

        if addr <= 0x3FFF {
            if self.registers.bank_mode_select == 0 {
                return self.rom[addr as usize];
            }
            let banked_addr = Self::calculate_rom_addr(addr, bank);
            return self.rom[banked_addr];
        }
        if addr <= 0x7FFF {
            if self.registers.bank_mode_select == 0 {
                bank = self.registers.rom_bank_number | (self.registers.ram_bank_number << 5)
            } else {
                bank = self.registers.rom_bank_number;
            }
            let banked_addr = Self::calculate_rom_addr(addr, bank);
            return self.rom[banked_addr];
        }
        if self.registers.bank_mode_select == 0 {
            0
        } else {
            let banked_addr = Self::calculate_ram_addr(addr, self.registers.ram_bank_number);
            self.eram[banked_addr]
        }

    }

    fn write_rom(&mut self, addr: u16, value: u8) {
        if addr <= 0x1FFF {
            self.registers.ram_enable = value & 0x0A == 0xA;
        } else if addr <= 0x3FFF {
            if value == 0 {
                self.registers.rom_bank_number = 1;
            } else {
                self.registers.rom_bank_number = value & 0x1F;
            }
        } else if addr <= 0x5FFF {
            if self.registers.bank_mode_select == 1 {
                self.eram[(addr - 0x4000) as usize] = value
            } else {
                self.registers.ram_bank_number = value & 0x03;
            }
        } else if addr <= 0x7FFF {
            self.registers.bank_mode_select = value & 1;
        } else {
            if self.registers.ram_enable {
                self.eram[(addr - 0x8000) as usize] = value;
            }
        }
    }
}


struct Registers {
    ram_enable: bool,
    rom_bank_number: u8,
    ram_bank_number: u8, // also used for low bank
    bank_mode_select: u8
}