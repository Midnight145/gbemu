
pub struct Header {
    pub entrypoint: [u8; 4],
    pub logo: [u8; 0x30],
    pub title: String,
    // pub manufacturer_code: [u8; 4],
    pub cgb_only: u8,
    pub new_licensee_code: [u8; 2],
    pub sgb_flag: u8,
    pub cartridge_type: u8,
    pub rom_size: u8,
    pub ram_size: u8,
    pub dest_code: u8,
    pub old_licensee_code: u8,
    pub mask_rom_version_number: u8,
    pub header_checksum: u8,
    pub global_checksum: u16
}

impl Header {
    pub fn new(bytes: Vec<u8>) -> Header {
        let b = &*bytes;
        let header = &b[0x100..0x150];

        let entrypoint: [u8; 4]        = header[0x00..0x04].try_into().unwrap();
        let logo: [u8; 0x30]           = header[0x04..0x34].try_into().unwrap();
        let title_bytes          = &header[0x34..0x43]; // 0x0F bytes
        let title                 = str::from_utf8(title_bytes).unwrap();
        let cgb_flag               = header[0x43];
        let new_licensee_code: [u8; 2] = header[0x44..0x46].try_into().unwrap();
        let sgb_flag               = header[0x46];
        let cartridge_type         = header[0x47];
        let rom_size               = header[0x48];
        let ram_size               = header[0x49];
        let dest_code              = header[0x4A];
        let old_licensee_code      = header[0x4B];
        let mask_rom_version       = header[0x4C];
        let header_checksum        = header[0x4D];
        let global_checksum       = u16::from_be_bytes(header[0x4E..0x50].try_into().unwrap());

        Header {
            entrypoint,
            logo,
            title: title.to_string(),
            cgb_only: cgb_flag,
            new_licensee_code,
            sgb_flag,
            cartridge_type,
            rom_size,
            ram_size,
            dest_code,
            old_licensee_code,
            mask_rom_version_number: mask_rom_version,
            header_checksum,
            global_checksum,
        }
    }
}

// pretty print debug with hex parsing instead of dec
use std::fmt;

struct Hex<T>(T);
impl<T: fmt::LowerHex> fmt::Debug for Hex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}

struct HexSlice<'a>(&'a [u8]);
impl<'a> fmt::Debug for HexSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[")?;
        for (i, b) in self.0.iter().enumerate() {
            if i != 0 {
                f.write_str(", ")?;
            }
            write!(f, "0x{:02x}", b)?;
        }
        f.write_str("]")
    }
}

impl fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Header")
            .field("entrypoint", &HexSlice(&self.entrypoint))
            .field("logo", &HexSlice(&self.logo))
            .field("title", &self.title)
            .field("cgb_only", &Hex(self.cgb_only))
            .field("new_licensee_code", &HexSlice(&self.new_licensee_code))
            .field("sgb_flag", &Hex(self.sgb_flag))
            .field("cartridge_type", &Hex(self.cartridge_type))
            .field("rom_size", &Hex(self.rom_size))
            .field("ram_size", &Hex(self.ram_size))
            .field("dest_code", &Hex(self.dest_code))
            .field("old_licensee_code", &Hex(self.old_licensee_code))
            .field("mask_rom_version_number", &Hex(self.mask_rom_version_number))
            .field("header_checksum", &Hex(self.header_checksum))
            .field("global_checksum", &Hex(self.global_checksum))
            .finish()
    }
}
