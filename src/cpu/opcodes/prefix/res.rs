#![allow(unused_variables)]

use crate::cpu::Bus::Bus;
use crate::cpu::CPU::{CPU};

// res
impl CPU {
    pub fn res_r8(bit: u8, register: &mut u8) {
        *register &= !(1 << bit);
    }
}

//res0
impl CPU{
    pub fn res0_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(0, &mut cpu.B);
        8
    }

    pub fn res0_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(0, &mut cpu.C);
        8
    }

    pub fn res0_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(0, &mut cpu.D);
        8
    }

    pub fn res0_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(0, &mut cpu.E);
        8
    }

    pub fn res0_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(0, &mut cpu.H);
        8
    }

    pub fn res0_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(0, &mut cpu.L);
        8
    }

    pub fn res0_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let mut byte = bus.read_byte(cpu.HL());
        Self::res_r8(0, &mut byte);
        bus.write_byte(cpu.HL(), byte);
        16
    }

    pub fn res0_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(0, &mut cpu.A);
        8
    }
}

// res1
impl CPU {
    pub fn res1_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(1, &mut cpu.B);
        8
    }

    pub fn res1_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(1, &mut cpu.C);
        8
    }

    pub fn res1_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(1, &mut cpu.D);
        8
    }

    pub fn res1_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(1, &mut cpu.E);
        8
    }

    pub fn res1_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(1, &mut cpu.H);
        8
    }

    pub fn res1_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(1, &mut cpu.L);
        8
    }

    pub fn res1_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let mut byte = bus.read_byte(cpu.HL());
        Self::res_r8(1, &mut byte);
        bus.write_byte(cpu.HL(), byte);
        16
    }

    pub fn res1_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(1, &mut cpu.A);
        8
    }
}
// res2
impl CPU {
    pub fn res2_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(2, &mut cpu.B);
        8
    }

    pub fn res2_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(2, &mut cpu.C);
        8
    }

    pub fn res2_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(2, &mut cpu.D);
        8
    }

    pub fn res2_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(2, &mut cpu.E);
        8
    }

    pub fn res2_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(2, &mut cpu.H);
        8
    }

    pub fn res2_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(2, &mut cpu.L);
        8
    }

    pub fn res2_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let mut byte = bus.read_byte(cpu.HL());
        Self::res_r8(2, &mut byte);
        bus.write_byte(cpu.HL(), byte);
        16
    }

    pub fn res2_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(2, &mut cpu.A);
        8
    }
}

// res3
impl CPU {
    pub fn res3_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(3, &mut cpu.B);
        8
    }

    pub fn res3_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(3, &mut cpu.C);
        8
    }

    pub fn res3_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(3, &mut cpu.D);
        8
    }

    pub fn res3_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(3, &mut cpu.E);
        8
    }

    pub fn res3_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(3, &mut cpu.H);
        8
    }

    pub fn res3_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(3, &mut cpu.L);
        8
    }

    pub fn res3_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let mut byte = bus.read_byte(cpu.HL());
        Self::res_r8(3, &mut byte);
        bus.write_byte(cpu.HL(), byte);
        16
    }

    pub fn res3_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(3, &mut cpu.A);
        8
    }
}

// res4
impl CPU {
    pub fn res4_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(4, &mut cpu.B);
        8
    }

    pub fn res4_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(4, &mut cpu.C);
        8
    }

    pub fn res4_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(4, &mut cpu.D);
        8
    }

    pub fn res4_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(4, &mut cpu.E);
        8
    }

    pub fn res4_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(4, &mut cpu.H);
        8
    }

    pub fn res4_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(4, &mut cpu.L);
        8
    }

    pub fn res4_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let mut byte = bus.read_byte(cpu.HL());
        Self::res_r8(4, &mut byte);
        bus.write_byte(cpu.HL(), byte);
        16
    }

    pub fn res4_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(4, &mut cpu.A);
        8
    }
}

// res5
impl CPU {
    pub fn res5_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(5, &mut cpu.B);
        8
    }

    pub fn res5_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(5, &mut cpu.C);
        8
    }

    pub fn res5_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(5, &mut cpu.D);
        8
    }

    pub fn res5_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(5, &mut cpu.E);
        8
    }

    pub fn res5_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(5, &mut cpu.H);
        8
    }

    pub fn res5_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(5, &mut cpu.L);
        8
    }

    pub fn res5_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let mut byte = bus.read_byte(cpu.HL());
        Self::res_r8(5, &mut byte);
        bus.write_byte(cpu.HL(), byte);
        16
    }

    pub fn res5_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(5, &mut cpu.A);
        8
    }
}

// res6
impl CPU {
    pub fn res6_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(6, &mut cpu.B);
        8
    }

    pub fn res6_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(6, &mut cpu.C);
        8
    }

    pub fn res6_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(6, &mut cpu.D);
        8
    }

    pub fn res6_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(6, &mut cpu.E);
        8
    }

    pub fn res6_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(6, &mut cpu.H);
        8
    }

    pub fn res6_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(6, &mut cpu.L);
        8
    }

    pub fn res6_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let mut byte = bus.read_byte(cpu.HL());
        Self::res_r8(6, &mut byte);
        bus.write_byte(cpu.HL(), byte);
        16
    }

    pub fn res6_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(6, &mut cpu.A);
        8
    }
}

// res7
impl CPU {
    pub fn res7_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(7, &mut cpu.B);
        8
    }

    pub fn res7_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(7, &mut cpu.C);
        8
    }

    pub fn res7_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(7, &mut cpu.D);
        8
    }

    pub fn res7_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(7, &mut cpu.E);
        8
    }

    pub fn res7_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(7, &mut cpu.H);
        8
    }

    pub fn res7_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(7, &mut cpu.L);
        8
    }

    pub fn res7_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let mut byte = bus.read_byte(cpu.HL());
        Self::res_r8(7, &mut byte);
        bus.write_byte(cpu.HL(), byte);
        16
    }

    pub fn res7_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::res_r8(7, &mut cpu.A);
        8
    }
}