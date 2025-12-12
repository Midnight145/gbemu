#![allow(non_snake_case)]

use crate::cartridge::mbc;
use crate::cartridge::mbc::MBC;
use crate::cpu::Interrupts;
use crate::cpu::Interrupts::Interrupt;

pub struct Bus {
    pub read: fn(&Bus, u16) -> u8,
    pub write: fn(&mut Bus, u16, u8),
    pub mbc: Box<dyn MBC>,
    pub VRAM: Vec<u8>,
    pub WRAM: Vec<u8>,
    pub OAM: [u8; 0xA0],
    pub HRAM: [u8; 0x80],
    pub registers: [u8; 0x80],
    pub timer: Timer,
    pub joypad: u8,
    pub interrupt_enable: u8,
    ram: Vec<u8> // used only for tests
}

impl Bus {
    pub fn new(mbc: Box<dyn MBC>) -> Bus {
        Bus {
            read: Bus::map_read_byte,
            write: Bus::map_write_byte,
            mbc,
            VRAM: vec![0; 0x2000],
            WRAM: vec![0; 0x2000],
            OAM: [0; 0xA0],
            registers: [0; 0x80],
            HRAM: [0; 0x80],
            timer: Timer::new(),
            joypad: 0,
            interrupt_enable: 0,
            ram: vec![]
        }
    }

    pub fn default() -> Bus {
        Bus {
            read: Bus::map_read_byte,
            write: Bus::map_write_byte,
            mbc: mbc::default(),
            VRAM: vec![0; 0x2000],
            WRAM: vec![0; 0x2000],
            OAM: [0; 0xA0],
            registers: [0; 0x80],
            HRAM: [0; 0x80],
            timer: Timer::new(),
            joypad: 0,
            interrupt_enable: 0,
            ram: vec![]
        }
    }

    pub fn test() -> Bus {
        Bus {
            read: Bus::test_read_byte,
            write: Bus::test_write_byte,
            mbc: mbc::default(),
            VRAM: vec![0; 0x2000],
            WRAM: vec![0; 0x2000],
            OAM: [0; 0xA0],
            registers: [0; 0x80],
            HRAM: [0; 0x80],
            timer: Timer::new(),
            joypad: 0,
            interrupt_enable: 0,
            ram: vec![0; 65536]
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        (self.write)(self, addr, value);
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        (self.read)(self, addr)
    }

    pub fn map_write_byte(&mut self, addr: u16, value: u8) {
        if addr <= 0x7FFF {
            self.mbc.write_rom(addr, value)
        }
        else if addr <= 0x9FFF {
            self.VRAM[(addr - 0x8000) as usize] = value
        }
        else if addr <= 0xBFFF {
        self.mbc.write_rom(addr, value)
        }
        else if addr <= 0xDFFF {
            self.WRAM[(addr - 0xC000) as usize] = value
        }
        else if addr <= 0xFDFF {
            let wram_addr = addr - 0xE000; // E000 → 0
            self.WRAM[wram_addr as usize] = value;
        }
        else if addr <= 0xFE9F {
            self.OAM[(addr - 0xFE00) as usize] = value
        }
        else if addr <= 0xFEFF {
            // TODO: https://gbdev.io/pandocs/Memory_Map.html#fea0feff-range
        }
        else if addr == 0xFF00 {
            self.joypad = value;
            return;
        }
        else if addr <= 0xFF7F {
            if 0xFF04 <= addr && addr <= 0xFF07 {
                match addr {
                    0xFF04 => self.timer.write_div(value),
                    0xFF05 => {
                        self.timer.write_tima(value)
                    },
                    0xFF06 => self.timer.write_tma(value),
                    0xFF07 => self.timer.write_tac(value),
                    _ => ()
                }
            } else {
                if addr == 0xFF01 || addr == 0xFF02 {
                    // print!("{}", value)
                }
                self.registers[(addr - 0xFF00) as usize] = value
            }
        }
        else if addr <= 0xFFFE{
            self.HRAM[(addr - 0xFF80) as usize] = value
        }
        else if addr == 0xFFFF {
            self.interrupt_enable = value;
        }
    }

    pub fn map_read_byte(&self, addr: u16) -> u8 {
        if addr <= 0x7FFF {
            return self.mbc.read_rom(addr)
        }
        else if addr <= 0x9FFF {
            return self.VRAM[(addr - 0x8000) as usize]
        }
        else if addr <= 0xBFFF {
            return self.mbc.read_rom(addr)
        }
        else if addr <= 0xDFFF {
            return self.WRAM[(addr - 0xC000) as usize]
        }
        else if addr <= 0xFDFF {
            return self.read_byte(addr - 0x2000)
        }
        else if addr <= 0xFE9F {
            return self.OAM[(addr - 0xFE00) as usize]
        }
        else if addr <= 0xFEFF {
            // TODO: https://gbdev.io/pandocs/Memory_Map.html#fea0feff-range
            return 0xFF
        }
        else if addr <= 0xFF7F {
            if 0xFF04 <= addr && addr <= 0xFF07 {
                return match addr {
                    0xFF04 => self.timer.DIV,
                    0xFF05 => self.timer.TIMA,
                    0xFF06 => self.timer.TMA,
                    0xFF07 => self.timer.TAC,
                    _ => 0
                }
            }
            return self.registers[(addr - 0xFF00) as usize]
        }
        else if addr <= 0xFFFE{
            return self.HRAM[(addr - 0xFF80) as usize]
        }
        self.interrupt_enable
    }

    pub fn test_write_byte(&mut self, addr: u16, value: u8) {
        self.ram[addr as usize] = value;
    }
    pub fn test_read_byte(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    pub fn write_interrupt(&mut self, interrupt: Interrupt) {
        let interrupts = self.read_byte(Interrupts::ADDRESS);
        self.write_byte(Interrupts::ADDRESS, interrupts | interrupt as u8)
    }

    pub fn read_interrupts(&mut self) -> Interrupts::Interrupts {
        let byte = self.read_byte(0xFF0F);
        Interrupts::Interrupts {
            vblank: byte & Interrupt::VBLANK as u8 != 0,
            lcd: byte & Interrupt::LCD as u8 != 0,
            timer: byte & Interrupt::TIMER as u8 != 0,
            serial: byte & Interrupt::SERIAL as u8 != 0,
            joypad: byte & Interrupt::JOYPAD as u8 != 0,
            un5: byte & Interrupt::UN5 as u8 != 0,
            un6: byte & Interrupt::UN6 as u8 != 0,
            un7: byte & Interrupt::UN7 as u8 != 0,
        }
    }
}


pub struct Timer {
    counter: u16,
    DIV: u8,
    TIMA: u8,
    TMA: u8,
    TAC: u8
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            counter: 0,
            DIV: 0,
            TIMA: 0,
            TMA: 0,
            TAC: 0
        }
    }

    pub fn update(&mut self, cycles: u16) -> bool {
        self.counter = self.counter.wrapping_add(cycles);
        self.DIV = self.DIV.wrapping_add((self.counter >> 8) as u8);

        if self.TAC & 0b100 == 0 {
            return false;
        }

        let inc = match self.TAC & 0b11 {
            0b00 => 1024,
            0b01 => 16,
            0b10 => 64,
            0b11 => 256,
            _ => unreachable!()
        };
        
        let mut interrupt = false;

        while self.counter >= inc {
            if self.TIMA == 0xFF {
                self.TIMA = self.TMA;
                interrupt = true;
            } else {
                self.TIMA = self.TIMA.wrapping_add(1);
            }
            self.counter -= inc;
        }

        interrupt
    }

    #[allow(unused_variables)]
    pub fn write_div(&mut self, value: u8) {
        self.DIV = 0
    }

    pub fn write_tima(&mut self, value: u8) {
        self.TIMA = value
    }

    pub fn write_tma(&mut self, value: u8) {
        self.TMA = value;
    }

    pub fn write_tac(&mut self, value: u8) {
        self.TAC = value
    }
}
