use crate::cartridge::mbc::MBC;

pub struct MBCRAM {
    pub ram: Vec<u8>
}
impl MBC for MBCRAM {
    fn read_rom(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }
    fn write_rom(&mut self, addr: u16, value: u8) {
        self.ram[addr as usize] = value;
    }
}