use crate::cartridge::header::Header;
use crate::cartridge::mbc::mbc1::MBC1;
use crate::cartridge::mbc::ram_only::MBCRAM;

pub mod mbc1;
mod ram_only;

pub trait MBC {

    fn read_rom(&self, addr: u16) -> u8 {
        return 0;
    }
    fn write_rom(&mut self, addr: u16, value: u8) {}
}

pub fn new(header: &Header, rom: Vec<u8>) -> Box<dyn MBC> {
    match header.cartridge_type {
        0x01 => Box::new(MBC1::new(header, rom)),
        _ => todo!(),
    }
}

pub fn default() -> Box<dyn MBC> {
    Box::new(MBCRAM {
        ram: vec![0;0xC000]
    })
}

