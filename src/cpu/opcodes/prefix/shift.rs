#![allow(unused_variables)]

use crate::cpu::Bus::Bus;
use crate::cpu::CPU::{Flags, CPU};

// SLA
impl CPU {
    pub fn sla_r8(register: &mut u8, flags: &mut Flags) {
        flags.C = *register & 0b1000_0000 != 0;
        *register <<= 1;
        flags.Z = *register == 0;
        flags.N = false;
        flags.H = false;
    }

    pub fn sla_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::sla_r8(&mut cpu.B, &mut cpu.flags);
        8
    }
    pub fn sla_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::sla_r8(&mut cpu.C, &mut cpu.flags);
        8
    }
    pub fn sla_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::sla_r8(&mut cpu.D, &mut cpu.flags);
        8
    }
    pub fn sla_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::sla_r8(&mut cpu.E, &mut cpu.flags);
        8
    }
    pub fn sla_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::sla_r8(&mut cpu.H, &mut cpu.flags);
        8
    }
    pub fn sla_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::sla_r8(&mut cpu.L, &mut cpu.flags);
        8
    }
    pub fn sla_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let mut byte = bus.read_byte(cpu.HL());
        Self::sla_r8(&mut byte, &mut cpu.flags);
        bus.write_byte(cpu.HL(), byte);
        16
    }
    pub fn sla_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::sla_r8(&mut cpu.A, &mut cpu.flags);
        8
    }
}

// SRA
impl CPU {
    pub fn sra_r8(register: &mut u8, flags: &mut Flags) {
        let bit7 = *register & 0b1000_0000;
        flags.C = *register & 1 != 0;
        *register >>= 1;
        *register |= bit7;
        flags.Z = *register == 0;
        flags.N = false;
        flags.H = false;
    }

    pub fn sra_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::sra_r8(&mut cpu.B, &mut cpu.flags);
        8
    }
    pub fn sra_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::sra_r8(&mut cpu.C, &mut cpu.flags);
        8
    }
    pub fn sra_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::sra_r8(&mut cpu.D, &mut cpu.flags);
        8
    }
    pub fn sra_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::sra_r8(&mut cpu.E, &mut cpu.flags);
        8
    }
    pub fn sra_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::sra_r8(&mut cpu.H, &mut cpu.flags);
        8
    }
    pub fn sra_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::sra_r8(&mut cpu.L, &mut cpu.flags);
        8
    }
    pub fn sra_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let mut byte = bus.read_byte(cpu.HL());
        Self::sra_r8(&mut byte, &mut cpu.flags);
        bus.write_byte(cpu.HL(), byte);
        16
    }
    pub fn sra_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::sra_r8(&mut cpu.A, &mut cpu.flags);
        8
    }
}

// SRL
impl CPU {
    pub fn srl_r8(register: &mut u8, flags: &mut Flags) {
        flags.C = *register & 1 != 0;
        *register >>= 1;
        flags.Z = *register == 0;
        flags.N = false;
        flags.H = false;
    }

    pub fn srl_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::srl_r8(&mut cpu.B, &mut cpu.flags);
        8
    }
    pub fn srl_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::srl_r8(&mut cpu.C, &mut cpu.flags);
        8
    }
    pub fn srl_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::srl_r8(&mut cpu.D, &mut cpu.flags);
        8
    }
    pub fn srl_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::srl_r8(&mut cpu.E, &mut cpu.flags);
        8
    }
    pub fn srl_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::srl_r8(&mut cpu.H, &mut cpu.flags);
        8
    }
    pub fn srl_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::srl_r8(&mut cpu.L, &mut cpu.flags);
        8
    }
    pub fn srl_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let mut byte = bus.read_byte(cpu.HL());
        Self::srl_r8(&mut byte, &mut cpu.flags);
        bus.write_byte(cpu.HL(), byte);
        16
    }
    pub fn srl_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::srl_r8(&mut cpu.A, &mut cpu.flags);
        8
    }
}

// SWAP
impl CPU {
    pub fn swap_r8(register: &mut u8, flags: &mut Flags) {
        let mut tmp = 0b0000_0000;
        tmp |= *register << 4;
        tmp |= *register >> 4;
        *register = tmp;
        flags.Z = *register == 0;
        flags.N = false;
        flags.H = false;
        flags.C = false;
    }

    pub fn swap_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::swap_r8(&mut cpu.B, &mut cpu.flags);
        8
    }
    pub fn swap_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::swap_r8(&mut cpu.C, &mut cpu.flags);
        8
    }
    pub fn swap_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::swap_r8(&mut cpu.D, &mut cpu.flags);
        8
    }
    pub fn swap_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::swap_r8(&mut cpu.E, &mut cpu.flags);
        8
    }
    pub fn swap_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::swap_r8(&mut cpu.H, &mut cpu.flags);
        8
    }
    pub fn swap_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::swap_r8(&mut cpu.L, &mut cpu.flags);
        8
    }
    pub fn swap_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let mut byte = bus.read_byte(cpu.HL());
        Self::swap_r8(&mut byte, &mut cpu.flags);
        bus.write_byte(cpu.HL(), byte);
        16
    }
    pub fn swap_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::swap_r8(&mut cpu.A, &mut cpu.flags);
        8
    }
}