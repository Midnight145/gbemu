#![allow(non_snake_case)]

use std::ops::Add;
use crate::cpu::Bus::Bus;

pub struct CPU {
    pub A: u8,
    pub B: u8,
    pub C: u8,
    pub D: u8,
    pub E: u8,
    pub F: u8,
    pub H: u8,
    pub L: u8,
    pub PC: u16,
    pub SP: u16,
    pub flags: Flags,
    pub ime: bool,
    pub ime_scheduled: bool, // effects of IME being enabled are delayed by one cycle
    pub halt: bool,
    pub stop: bool,
    pub executed_invalid: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            A: 0x01,
            B: 0x00,
            C: 0x13,
            D: 0x00,
            E: 0xD8,
            F: 0b10110000,
            H: 0x01,
            L: 0x4D,
            SP: 0xFFFE,
            PC: 0x100, // cartridge entry point
            flags: Flags {
                Z: true,
                N: false,
                H: true,
                C: true,
            },
            ime: false,
            ime_scheduled: false,
            halt: false,
            stop: false,
            executed_invalid: false,
        }
    }

    pub fn reset(&mut self) {
        self.A = 0;
        self.B = 0;
        self.C = 0;
        self.D = 0;
        self.E = 0;
        self.F = 0;
        self.H = 0;
        self.L = 0;
        self.SP = 0;
        self.PC = 0;
        self.flags.Z = false;
        self.flags.N = false;
        self.flags.H = false;
        self.flags.C = false;
    }

    pub fn AF(&mut self) -> u16 {
        self.F = self.flags.to_u8();
        ((self.A as u16) << 8) | self.F as u16
    }

    pub fn write_AF(&mut self, value: u16) {
        self.A = (value >> 8) as u8;
        self.F = (value & 0xF0) as u8;
        self.flags = Flags::from_u8(self.F);
    }

    pub fn BC(&self) -> u16 {
        ((self.B as u16) << 8) | (self.C as u16)
    }

    pub fn write_BC(&mut self, value: u16) {
        self.B = (value >> 8) as u8;
        self.C = (value & 0xFF) as u8;
    }

    pub fn DE(&self) -> u16 {
        ((self.D as u16) << 8) | (self.E as u16)
    }

    pub fn write_DE(&mut self, value: u16) {
        self.D = (value >> 8) as u8;
        self.E = (value & 0xFF) as u8;
    }

    pub fn HL(&self) -> u16 {
        ((self.H as u16) << 8) | (self.L as u16)
    }

    pub fn write_HL(&mut self, value: u16) {
        self.H = (value >> 8) as u8;
        self.L = (value & 0xFF) as u8;
    }

    pub fn read_u8_from_pc(&mut self, bus: &Bus) -> u8 {
        let byte = bus.read_byte(self.PC);
        // print!("{:02x} ", byte);
        self.PC = self.PC.wrapping_add(1);
        byte
    }

    pub fn read_u16_from_pc(&mut self, bus: &Bus) -> u16 {
        let low = self.read_u8_from_pc(bus);
        let high = self.read_u8_from_pc(bus);
        (high as u16) << 8 | (low as u16)
    }



}
pub struct Flags {
    pub Z: bool,
    pub N: bool,
    pub H: bool,
    pub C: bool,
}

impl Flags {
    pub fn check_half_carry_add_u8(&self, a: u8, b: u8, carry: u8) -> bool {
        (a & 0x0F).wrapping_add((b & 0x0F).wrapping_add(carry)) & 0x10 != 0
    }

    pub fn check_half_carry_sub_u8(&self, a: u8, b: u8, carry: u8) -> bool {
        (a & 0x0F) < ((b & 0x0F).wrapping_add(carry))
    }

    pub fn check_full_carry_add_u8(&self, a: u8, b: u8, carry: u8) -> bool {
        (a as u16 + (b as u16 + carry as u16)) > 0xFF
    }

    pub fn check_full_carry_sub_u8(&self, a: u8, b: u8, carry: u8) -> bool {
        (a as u16) < (b as u16).wrapping_add(carry as u16)
    }

    pub fn check_half_carry_add_u16(&self, a: u16, b: u16) -> bool {
        ((a & 0x0FFF) + (b & 0x0FFF)) & 0x1000 == 0x1000
    }

    pub fn check_full_carry_add_u16(&self, a: u16, b: u16) -> bool {
        (a as u32 + b as u32) > 0xFFFF
    }

    pub fn check_half_carry_sub_u16(&self, a: u16, b: u16) -> bool {
        (a & 0x0FFF) < (b & 0x0FFF)
    }

    pub fn check_full_carry_sub_u16(&self, a: u16, b: u16) -> bool {
        a < b
    }

    pub fn from_u8(value: u8) -> Flags {
        Flags{
            Z: value & 0b1000_0000 != 0,
            N: value & 0b0100_0000 != 0,
            H: value & 0b0010_0000 != 0,
            C: value & 0b0001_0000 != 0
        }
    }

    pub fn to_u8(&self) -> u8 {
        let mut f = 0u8;

        if self.Z { f |= 0b1000_0000; }
        if self.N { f |= 0b0100_0000; }
        if self.H { f |= 0b0010_0000; }
        if self.C { f |= 0b0001_0000; }
        f
    }

    pub fn to_str(&self) -> String {
        let mut s: String = String::new();
        s += if self.Z {
            "Z"
        } else {
            "-"
        };
        s += if self.N {
            "N"
        } else {
            "-"
        };
        s += if self.H {
            "H"
        } else {
            "-"
        };
        s += if self.C {
            "C"
        } else {
            "-"
        };
        s
    }

}