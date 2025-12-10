use crate::cpu::Bus::Bus;
use crate::cpu::CPU::{Flags, CPU};

// BIT
impl CPU {
    pub fn bit_r8(bit: u8, register: u8, flags: &mut Flags) {
        let mask = 1 << bit;
        flags.Z = register & mask == 0;
        flags.N = false;
        flags.H = true;
    }
}

//BIT0
impl CPU{
    pub fn bit0_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(0, cpu.B, &mut cpu.flags);
        8
    }

    pub fn bit0_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(0, cpu.C, &mut cpu.flags);
        8
    }

    pub fn bit0_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(0, cpu.D, &mut cpu.flags);
        8
    }

    pub fn bit0_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(0, cpu.E, &mut cpu.flags);
        8
    }

    pub fn bit0_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(0, cpu.H, &mut cpu.flags);
        8
    }

    pub fn bit0_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(0, cpu.L, &mut cpu.flags);
        8
    }

    pub fn bit0_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let byte = bus.read_byte(cpu.HL());
        Self::bit_r8(0, byte, &mut cpu.flags);
        12
    }

    pub fn bit0_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(0, cpu.A, &mut cpu.flags);
        8
    }
}

// BIT1
impl CPU {
    pub fn bit1_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(1, cpu.B, &mut cpu.flags);
        8
    }

    pub fn bit1_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(1, cpu.C, &mut cpu.flags);
        8
    }

    pub fn bit1_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(1, cpu.D, &mut cpu.flags);
        8
    }

    pub fn bit1_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(1, cpu.E, &mut cpu.flags);
        8
    }

    pub fn bit1_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(1, cpu.H, &mut cpu.flags);
        8
    }

    pub fn bit1_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(1, cpu.L, &mut cpu.flags);
        8
    }

    pub fn bit1_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let byte = bus.read_byte(cpu.HL());
        Self::bit_r8(1, byte, &mut cpu.flags);
        12
    }

    pub fn bit1_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(1, cpu.A, &mut cpu.flags);
        8
    }
}
// BIT2
impl CPU {
    pub fn bit2_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(2, cpu.B, &mut cpu.flags);
        8
    }

    pub fn bit2_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(2, cpu.C, &mut cpu.flags);
        8
    }

    pub fn bit2_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(2, cpu.D, &mut cpu.flags);
        8
    }

    pub fn bit2_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(2, cpu.E, &mut cpu.flags);
        8
    }

    pub fn bit2_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(2, cpu.H, &mut cpu.flags);
        8
    }

    pub fn bit2_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(2, cpu.L, &mut cpu.flags);
        8
    }

    pub fn bit2_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let byte = bus.read_byte(cpu.HL());
        Self::bit_r8(2, byte, &mut cpu.flags);
        12
    }

    pub fn bit2_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(2, cpu.A, &mut cpu.flags);
        8
    }
}

// BIT3
impl CPU {
    pub fn bit3_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(3, cpu.B, &mut cpu.flags);
        8
    }

    pub fn bit3_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(3, cpu.C, &mut cpu.flags);
        8
    }

    pub fn bit3_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(3, cpu.D, &mut cpu.flags);
        8
    }

    pub fn bit3_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(3, cpu.E, &mut cpu.flags);
        8
    }

    pub fn bit3_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(3, cpu.H, &mut cpu.flags);
        8
    }

    pub fn bit3_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(3, cpu.L, &mut cpu.flags);
        8
    }

    pub fn bit3_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let byte = bus.read_byte(cpu.HL());
        Self::bit_r8(3, byte, &mut cpu.flags);
        12
    }

    pub fn bit3_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(3, cpu.A, &mut cpu.flags);
        8
    }
}

// BIT4
impl CPU {
    pub fn bit4_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(4, cpu.B, &mut cpu.flags);
        8
    }

    pub fn bit4_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(4, cpu.C, &mut cpu.flags);
        8
    }

    pub fn bit4_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(4, cpu.D, &mut cpu.flags);
        8
    }

    pub fn bit4_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(4, cpu.E, &mut cpu.flags);
        8
    }

    pub fn bit4_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(4, cpu.H, &mut cpu.flags);
        8
    }

    pub fn bit4_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(4, cpu.L, &mut cpu.flags);
        8
    }

    pub fn bit4_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let byte = bus.read_byte(cpu.HL());
        Self::bit_r8(4, byte, &mut cpu.flags);
        12
    }

    pub fn bit4_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(4, cpu.A, &mut cpu.flags);
        8
    }
}

// BIT5
impl CPU {
    pub fn bit5_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(5, cpu.B, &mut cpu.flags);
        8
    }

    pub fn bit5_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(5, cpu.C, &mut cpu.flags);
        8
    }

    pub fn bit5_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(5, cpu.D, &mut cpu.flags);
        8
    }

    pub fn bit5_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(5, cpu.E, &mut cpu.flags);
        8
    }

    pub fn bit5_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(5, cpu.H, &mut cpu.flags);
        8
    }

    pub fn bit5_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(5, cpu.L, &mut cpu.flags);
        8
    }

    pub fn bit5_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let byte = bus.read_byte(cpu.HL());
        Self::bit_r8(5, byte, &mut cpu.flags);
        12
    }

    pub fn bit5_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(5, cpu.A, &mut cpu.flags);
        8
    }
}

// BIT6
impl CPU {
    pub fn bit6_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(6, cpu.B, &mut cpu.flags);
        8
    }

    pub fn bit6_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(6, cpu.C, &mut cpu.flags);
        8
    }

    pub fn bit6_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(6, cpu.D, &mut cpu.flags);
        8
    }

    pub fn bit6_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(6, cpu.E, &mut cpu.flags);
        8
    }

    pub fn bit6_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(6, cpu.H, &mut cpu.flags);
        8
    }

    pub fn bit6_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(6, cpu.L, &mut cpu.flags);
        8
    }

    pub fn bit6_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let byte = bus.read_byte(cpu.HL());
        Self::bit_r8(6, byte, &mut cpu.flags);
        12
    }

    pub fn bit6_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(6, cpu.A, &mut cpu.flags);
        8
    }
}

// BIT7
impl CPU {
    pub fn bit7_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(7, cpu.B, &mut cpu.flags);
        8
    }

    pub fn bit7_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(7, cpu.C, &mut cpu.flags);
        8
    }

    pub fn bit7_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(7, cpu.D, &mut cpu.flags);
        8
    }

    pub fn bit7_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(7, cpu.E, &mut cpu.flags);
        8
    }

    pub fn bit7_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(7, cpu.H, &mut cpu.flags);
        8
    }

    pub fn bit7_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(7, cpu.L, &mut cpu.flags);
        8
    }

    pub fn bit7_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let byte = bus.read_byte(cpu.HL());
        Self::bit_r8(7, byte, &mut cpu.flags);
        12
    }

    pub fn bit7_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::bit_r8(7, cpu.A, &mut cpu.flags);
        8
    }
}