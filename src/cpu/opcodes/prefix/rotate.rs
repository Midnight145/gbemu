#![allow(unused_variables)]

use crate::cpu::Bus::Bus;
use crate::cpu::CPU::{Flags, CPU};

// RLC
impl CPU {
    pub fn rlc_r8(register: &mut u8, flags: &mut Flags) {
        let carry = *register & 0b1000_0000 != 0;
        *register = (*register << 1) | (carry as u8);
        flags.C = carry;

        flags.N = false;
        flags.H = false;
        flags.Z = *register == 0;
    }

    pub fn rlc_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rlc_r8(&mut cpu.B, &mut cpu.flags);
        8
    }

    pub fn rlc_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rlc_r8(&mut cpu.C, &mut cpu.flags);
        8
    }

    pub fn rlc_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rlc_r8(&mut cpu.D, &mut cpu.flags);
        8
    }

    pub fn rlc_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rlc_r8(&mut cpu.E, &mut cpu.flags);
        8
    }

    pub fn rlc_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rlc_r8(&mut cpu.H, &mut cpu.flags);
        8
    }

    pub fn rlc_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rlc_r8(&mut cpu.L, &mut cpu.flags);
        8
    }

    pub fn rlc_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let mut byte = bus.read_byte(cpu.HL());
        Self::rlc_r8(&mut byte, &mut cpu.flags);
        bus.write_byte(cpu.HL(), byte);
        16
    }

    pub fn rlc_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rlc_r8(&mut cpu.A, &mut cpu.flags);
        8
    }
}

// RRC
impl CPU {
    pub fn rrc_r8(register: &mut u8, flags: &mut Flags) {
        let carry =  *register & 0b1 != 0;
        *register = (*register >> 1) | ((carry as u8) << 7);
        flags.C = carry;

        flags.N = false;
        flags.H = false;
        flags.Z = *register == 0;
    }

    pub fn rrc_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rrc_r8(&mut cpu.B, &mut cpu.flags);
        8
    }

    pub fn rrc_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rrc_r8(&mut cpu.C, &mut cpu.flags);
        8
    }

    pub fn rrc_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rrc_r8(&mut cpu.D, &mut cpu.flags);
        8
    }

    pub fn rrc_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rrc_r8(&mut cpu.E, &mut cpu.flags);
        8
    }

    pub fn rrc_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rrc_r8(&mut cpu.H, &mut cpu.flags);
        8
    }

    pub fn rrc_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rrc_r8(&mut cpu.L, &mut cpu.flags);
        8
    }

    pub fn rrc_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let mut byte = bus.read_byte(cpu.HL());
        Self::rrc_r8(&mut byte, &mut cpu.flags);
        bus.write_byte(cpu.HL(), byte);
        16
    }

    pub fn rrc_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rrc_r8(&mut cpu.A, &mut cpu.flags);
        8
    }
}

// RL
impl CPU {
    pub fn rl_r8(register: &mut u8, flags: &mut Flags) -> u8 {
        let carry = flags.C as u8;
        flags.C = *register & 0b1000_0000 != 0;
        *register = (*register << 1) | carry;
        flags.N = false;
        flags.H = false;
        flags.Z = *register == 0;
        4
    }

    pub fn rl_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rl_r8(&mut cpu.B, &mut cpu.flags);
        8
    }

    pub fn rl_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rl_r8(&mut cpu.C, &mut cpu.flags);
        8
    }

    pub fn rl_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rl_r8(&mut cpu.D, &mut cpu.flags);
        8
    }

    pub fn rl_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rl_r8(&mut cpu.E, &mut cpu.flags);
        8
    }

    pub fn rl_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rl_r8(&mut cpu.H, &mut cpu.flags);
        8
    }

    pub fn rl_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rl_r8(&mut cpu.L, &mut cpu.flags);
        8
    }

    pub fn rl_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let mut byte = bus.read_byte(cpu.HL());
        Self::rl_r8(&mut byte, &mut cpu.flags);
        bus.write_byte(cpu.HL(), byte);
        16
    }

    pub fn rl_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rl_r8(&mut cpu.A, &mut cpu.flags);
        8
    }
}

impl CPU {
    pub fn rr_r8(register: &mut u8, flags: &mut Flags) {
        let carry = flags.C as u8;
        flags.C = *register & 0b1 != 0;
        *register = (*register >> 1) | (carry << 7);
        flags.N = false;
        flags.H = false;
        flags.Z = *register == 0;
    }


    pub fn rr_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rr_r8(&mut cpu.B, &mut cpu.flags);
        8
    }

    pub fn rr_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rr_r8(&mut cpu.C, &mut cpu.flags);
        8
    }

    pub fn rr_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rr_r8(&mut cpu.D, &mut cpu.flags);
        8
    }

    pub fn rr_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rr_r8(&mut cpu.E, &mut cpu.flags);
        8
    }

    pub fn rr_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rr_r8(&mut cpu.H, &mut cpu.flags);
        8
    }

    pub fn rr_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rr_r8(&mut cpu.L, &mut cpu.flags);
        8
    }

    pub fn rr_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let mut byte = bus.read_byte(cpu.HL());
        Self::rr_r8(&mut byte, &mut cpu.flags);
        bus.write_byte(cpu.HL(), byte);
        16
    }

    pub fn rr_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::rr_r8(&mut cpu.A, &mut cpu.flags);
        8
    }
}