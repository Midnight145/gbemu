#![allow(unused_variables)]

use crate::cpu::Bus::Bus;
use crate::cpu::CPU::CPU;

// set
impl CPU {
    pub fn set_r8(bit: u8, register: &mut u8) {
        *register |= 1 << bit;
    }
}

//set0
impl CPU{
    pub fn set0_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(0, &mut cpu.B);
        8
    }

    pub fn set0_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(0, &mut cpu.C);
        8
    }

    pub fn set0_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(0, &mut cpu.D);
        8
    }

    pub fn set0_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(0, &mut cpu.E);
        8
    }

    pub fn set0_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(0, &mut cpu.H);
        8
    }

    pub fn set0_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(0, &mut cpu.L);
        8
    }

    pub fn set0_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let mut byte = bus.read_byte(cpu.HL());
        Self::set_r8(0, &mut byte);
        bus.write_byte(cpu.HL(), byte);
        16
    }

    pub fn set0_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(0, &mut cpu.A);
        8
    }
}

// set1
impl CPU {
    pub fn set1_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(1, &mut cpu.B);
        8
    }

    pub fn set1_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(1, &mut cpu.C);
        8
    }

    pub fn set1_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(1, &mut cpu.D);
        8
    }

    pub fn set1_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(1, &mut cpu.E);
        8
    }

    pub fn set1_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(1, &mut cpu.H);
        8
    }

    pub fn set1_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(1, &mut cpu.L);
        8
    }

    pub fn set1_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let mut byte = bus.read_byte(cpu.HL());
        Self::set_r8(1, &mut byte);
        bus.write_byte(cpu.HL(), byte);
        16
    }

    pub fn set1_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(1, &mut cpu.A);
        8
    }
}
// set2
impl CPU {
    pub fn set2_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(2, &mut cpu.B);
        8
    }

    pub fn set2_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(2, &mut cpu.C);
        8
    }

    pub fn set2_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(2, &mut cpu.D);
        8
    }

    pub fn set2_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(2, &mut cpu.E);
        8
    }

    pub fn set2_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(2, &mut cpu.H);
        8
    }

    pub fn set2_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(2, &mut cpu.L);
        8
    }

    pub fn set2_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let mut byte = bus.read_byte(cpu.HL());
        Self::set_r8(2, &mut byte);
        bus.write_byte(cpu.HL(), byte);
        16
    }

    pub fn set2_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(2, &mut cpu.A);
        8
    }
}

// set3
impl CPU {
    pub fn set3_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(3, &mut cpu.B);
        8
    }

    pub fn set3_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(3, &mut cpu.C);
        8
    }

    pub fn set3_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(3, &mut cpu.D);
        8
    }

    pub fn set3_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(3, &mut cpu.E);
        8
    }

    pub fn set3_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(3, &mut cpu.H);
        8
    }

    pub fn set3_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(3, &mut cpu.L);
        8
    }

    pub fn set3_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let mut byte = bus.read_byte(cpu.HL());
        Self::set_r8(3, &mut byte);
        bus.write_byte(cpu.HL(), byte);
        16
    }

    pub fn set3_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(3, &mut cpu.A);
        8
    }
}

// set4
impl CPU {
    pub fn set4_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(4, &mut cpu.B);
        8
    }

    pub fn set4_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(4, &mut cpu.C);
        8
    }

    pub fn set4_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(4, &mut cpu.D);
        8
    }

    pub fn set4_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(4, &mut cpu.E);
        8
    }

    pub fn set4_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(4, &mut cpu.H);
        8
    }

    pub fn set4_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(4, &mut cpu.L);
        8
    }

    pub fn set4_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let mut byte = bus.read_byte(cpu.HL());
        Self::set_r8(4, &mut byte);
        bus.write_byte(cpu.HL(), byte);
        16
    }

    pub fn set4_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(4, &mut cpu.A);
        8
    }
}

// set5
impl CPU {
    pub fn set5_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(5, &mut cpu.B);
        8
    }

    pub fn set5_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(5, &mut cpu.C);
        8
    }

    pub fn set5_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(5, &mut cpu.D);
        8
    }

    pub fn set5_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(5, &mut cpu.E);
        8
    }

    pub fn set5_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(5, &mut cpu.H);
        8
    }

    pub fn set5_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(5, &mut cpu.L);
        8
    }

    pub fn set5_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let mut byte = bus.read_byte(cpu.HL());
        Self::set_r8(5, &mut byte);
        bus.write_byte(cpu.HL(), byte);
        16
    }

    pub fn set5_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(5, &mut cpu.A);
        8
    }
}

// set6
impl CPU {
    pub fn set6_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(6, &mut cpu.B);
        8
    }

    pub fn set6_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(6, &mut cpu.C);
        8
    }

    pub fn set6_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(6, &mut cpu.D);
        8
    }

    pub fn set6_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(6, &mut cpu.E);
        8
    }

    pub fn set6_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(6, &mut cpu.H);
        8
    }

    pub fn set6_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(6, &mut cpu.L);
        8
    }

    pub fn set6_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let mut byte = bus.read_byte(cpu.HL());
        Self::set_r8(6, &mut byte);
        bus.write_byte(cpu.HL(), byte);
        16
    }

    pub fn set6_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(6, &mut cpu.A);
        8
    }
}

// set7
impl CPU {
    pub fn set7_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(7, &mut cpu.B);
        8
    }

    pub fn set7_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(7, &mut cpu.C);
        8
    }

    pub fn set7_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(7, &mut cpu.D);
        8
    }

    pub fn set7_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(7, &mut cpu.E);
        8
    }

    pub fn set7_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(7, &mut cpu.H);
        8
    }

    pub fn set7_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(7, &mut cpu.L);
        8
    }

    pub fn set7_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let mut byte = bus.read_byte(cpu.HL());
        Self::set_r8(7, &mut byte);
        bus.write_byte(cpu.HL(), byte);
        16
    }

    pub fn set7_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::set_r8(7, &mut cpu.A);
        8
    }
}