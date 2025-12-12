#![allow(unused_variables)]

use crate::cpu::Bus::Bus;
use crate::cpu::CPU::CPU;

// 8bit load operations
impl CPU {
    pub fn ld_bc_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let bc = cpu.BC();
        bus.write_byte(bc, cpu.A);
        8
    }

    pub fn ld_b_n8(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = cpu.read_u8_from_pc(bus);
        cpu.B = value;
        8
    }

    pub fn ld_a_bc(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.A = bus.read_byte(cpu.BC());
        8
    }

    pub fn ld_c_n8(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = cpu.read_u8_from_pc(bus);
        cpu.C = value;
        8
    }

    pub fn ld_de_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let de = cpu.DE();
        bus.write_byte(de, cpu.A);
        8
    }

    pub fn ld_d_n8(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = cpu.read_u8_from_pc(bus);
        cpu.D = value;
        8
    }

    pub fn ld_a_de(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.A = bus.read_byte(cpu.DE());
        8
    }

    pub fn ld_e_n8(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = cpu.read_u8_from_pc(bus);
        cpu.E = value;
        8
    }

    pub fn ld_hli_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        bus.write_byte(cpu.HL(), cpu.A);
        cpu.write_HL(cpu.HL().wrapping_add(1));
        8
    }

    pub fn ld_h_n8(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = cpu.read_u8_from_pc(bus);
        cpu.H = value;
        8
    }

    pub fn ld_a_hli(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = bus.read_byte(cpu.HL());
        cpu.A = value;
        cpu.write_HL(cpu.HL().wrapping_add(1));
        8
    }

    pub fn ld_l_n8(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = cpu.read_u8_from_pc(bus);
        cpu.L = value;
        8
    }

    pub fn ld_hld_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        bus.write_byte(cpu.HL(), cpu.A);
        cpu.write_HL(cpu.HL().wrapping_sub(1));
        8
    }

    pub fn ld_hl_n8(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = cpu.read_u8_from_pc(bus);
        bus.write_byte(cpu.HL(), value);
        12
    }

    pub fn ld_a_hld(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = bus.read_byte(cpu.HL());
        cpu.A = value;
        cpu.write_HL(cpu.HL().wrapping_sub(1));
        8
    }

    pub fn ld_a_n8(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = cpu.read_u8_from_pc(bus);
        cpu.A = value;
        8
    }

    pub fn ld_b_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        4
    }

    pub fn ld_b_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.B = cpu.C;
        4
    }
    pub fn ld_b_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.B = cpu.D;
        4
    }
    pub fn ld_b_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.B = cpu.E;
        4
    }
    pub fn ld_b_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.B = cpu.H;
        4
    }
    pub fn ld_b_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.B = cpu.L;
        4
    }
    pub fn ld_b_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = bus.read_byte(cpu.HL());
        cpu.B = value;
        8
    }
    pub fn ld_b_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.B = cpu.A;
        4
    }

    pub fn ld_c_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.C = cpu.B;
        4
    }

    pub fn ld_c_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        4
    }
    pub fn ld_c_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.C = cpu.D;
        4
    }
    pub fn ld_c_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.C = cpu.E;
        4
    }
    pub fn ld_c_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.C = cpu.H;
        4
    }
    pub fn ld_c_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.C = cpu.L;
        4
    }
    pub fn ld_c_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = bus.read_byte(cpu.HL());
        cpu.C = value;
        8
    }
    pub fn ld_c_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.C = cpu.A;
        4
    }

    pub fn ld_d_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.D = cpu.B;
        4
    }

    pub fn ld_d_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.D = cpu.C;
        4
    }
    pub fn ld_d_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        4
    }
    pub fn ld_d_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.D = cpu.E;
        4
    }
    pub fn ld_d_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.D = cpu.H;
        4
    }
    pub fn ld_d_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.D = cpu.L;
        4
    }
    pub fn ld_d_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = bus.read_byte(cpu.HL());
        cpu.D = value;
        8
    }
    pub fn ld_d_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.D = cpu.A;
        4
    }

    pub fn ld_e_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.E = cpu.B;
        4
    }

    pub fn ld_e_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.E = cpu.C;
        4
    }
    pub fn ld_e_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.E = cpu.D;
        4
    }
    pub fn ld_e_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        4
    }
    pub fn ld_e_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.E = cpu.H;
        4
    }
    pub fn ld_e_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.E = cpu.L;
        4
    }
    pub fn ld_e_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = bus.read_byte(cpu.HL());
        cpu.E = value;
        8
    }
    pub fn ld_e_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.E = cpu.A;
        4
    }

    pub fn ld_h_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.H = cpu.B;
        4
    }

    pub fn ld_h_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.H = cpu.C;
        4
    }
    pub fn ld_h_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.H = cpu.D;
        4
    }
    pub fn ld_h_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.H = cpu.E;
        4
    }
    pub fn ld_h_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        4
    }
    pub fn ld_h_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.H = cpu.L;
        4
    }
    pub fn ld_h_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = bus.read_byte(cpu.HL());
        cpu.H = value;
        8
    }
    pub fn ld_h_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.H = cpu.A;
        4
    }

    pub fn ld_l_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.L = cpu.B;
        4
    }

    pub fn ld_l_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.L = cpu.C;
        4
    }
    pub fn ld_l_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.L = cpu.D;
        4
    }
    pub fn ld_l_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.L = cpu.E;
        4
    }
    pub fn ld_l_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.L = cpu.H;
        4
    }
    pub fn ld_l_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        4
    }
    pub fn ld_l_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = bus.read_byte(cpu.HL());
        cpu.L = value;
        8
    }
    pub fn ld_l_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.L = cpu.A;
        4
    }

    pub fn ld_hl_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        bus.write_byte(cpu.HL(), cpu.B);
        8
    }

    pub fn ld_hl_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        bus.write_byte(cpu.HL(), cpu.C);
        8
    }

    pub fn ld_hl_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        bus.write_byte(cpu.HL(), cpu.D);
        8
    }

    pub fn ld_hl_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        bus.write_byte(cpu.HL(), cpu.E);
        8
    }

    pub fn ld_hl_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        bus.write_byte(cpu.HL(), cpu.H);
        8
    }

    pub fn ld_hl_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        bus.write_byte(cpu.HL(), cpu.L);
        8
    }

    pub fn ld_hl_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        bus.write_byte(cpu.HL(), cpu.A);
        8
    }

    pub fn ld_a_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.A = cpu.B;
        4
    }

    pub fn ld_a_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.A = cpu.C;
        4
    }
    pub fn ld_a_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.A = cpu.D;
        4
    }
    pub fn ld_a_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.A = cpu.E;
        4
    }
    pub fn ld_a_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.A = cpu.H;
        4
    }
    pub fn ld_a_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.A = cpu.L;
        4
    }
    pub fn ld_a_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = bus.read_byte(cpu.HL());
        cpu.A = value;
        8
    }
    pub fn ld_a_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        4
    }
}

// 16bit load operations
impl CPU {
    pub fn ld_bc_n16(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = cpu.read_u16_from_pc(&bus);
        cpu.write_BC(value);
        12
    }
    pub fn ld_n16_sp(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let addr = cpu.read_u16_from_pc(bus);
        bus.write_byte(addr, (cpu.PC & 0xFF) as u8);
        bus.write_byte(addr + 1, (cpu.PC >> 8) as u8);
        20
    }
    pub fn ld_de_n16(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = cpu.read_u16_from_pc(&bus);
        cpu.write_DE(value);
        12
    }
    pub fn ld_hl_n16(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = cpu.read_u16_from_pc(&bus);
        cpu.write_HL(value);
        12
    }
    pub fn ld_sp_n16(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = cpu.read_u16_from_pc(&bus);
        cpu.SP = value;
        12
    }
    pub fn ld_hl_sp_e8(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let e8 = cpu.read_u8_from_pc(bus) as i16;
        let sp = cpu.SP as i16;
        let result = sp.wrapping_add(e8);

        cpu.write_HL(result as u16);

        let sp_low = (cpu.SP & 0xFF) as u8;
        let e8_u = (e8 as i8) as u8;

        cpu.flags.Z = false;
        cpu.flags.N = false;
        cpu.flags.H = cpu.flags.check_half_carry_add_u8(sp_low, e8_u);
        cpu.flags.C = cpu.flags.check_full_carry_add_u8(sp_low, e8_u);
        12
    }
    pub fn ld_sp_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.SP = cpu.HL();
        12
    }
}

// LDH (High RAM) operations
impl CPU {
    // a8 is an 8-bit address which gets added to 0xFF00
    pub fn ldh_a8_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let mut addr = cpu.read_u8_from_pc(bus) as u16;
        addr |= 0xFF00;
        bus.write_byte(addr, cpu.A);
        12
    }

    pub fn ldh_a_a8(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let mut addr = cpu.read_u8_from_pc(bus) as u16;
        addr |= 0xFF00;
        cpu.A = bus.read_byte(addr);
        12
    }

    pub fn ldh_c_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let addr = cpu.C as u16 | 0xFF00;
        bus.write_byte(addr, cpu.A);
        8
    }

    pub fn ldh_a_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let addr = cpu.C as u16 | 0xFF00;
        cpu.A = bus.read_byte(addr);
        8
    }
}

// 8/16bit hybrid operations
impl CPU {
    pub fn ld_a_a16(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let addr = cpu.read_u16_from_pc(bus);
        cpu.A = bus.read_byte(addr);
        16
    }

    pub fn ld_a16_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let addr = cpu.read_u16_from_pc(bus);
        bus.write_byte(addr, cpu.A);
        16
    }
}

// stack operations
impl CPU {

    pub fn push_u16(cpu: &mut CPU, bus: &mut Bus, value: u16) {
        cpu.SP = cpu.SP.wrapping_sub(1);
        bus.write_byte(cpu.SP, (value >> 8) as u8);
        cpu.SP = cpu.SP.wrapping_sub(1);
        bus.write_byte(cpu.SP, (value & 0xFF) as u8);
    }

    pub fn pop_u16(cpu: &mut CPU, bus: &mut Bus) -> u16 {
        let lo = bus.read_byte(cpu.SP) as u16;
        cpu.SP = cpu.SP.wrapping_add(1);
        let hi = bus.read_byte(cpu.SP) as u16;
        cpu.SP = cpu.SP.wrapping_add(1);
        hi << 8 | lo
    }

    pub fn push_af(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let af = cpu.AF();
        Self::push_u16(cpu, bus, af);
        16
    }

    pub fn pop_af(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = Self::pop_u16(cpu, bus);
        cpu.write_AF(value & 0xF0);
        // flags autocomputed
        12
    }

    pub fn push_bc(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let bc = cpu.BC();
        Self::push_u16(cpu, bus, bc);
        16
    }

    pub fn pop_bc(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = Self::pop_u16(cpu, bus);
        cpu.write_BC(value);
        12
    }

    pub fn push_de(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let de = cpu.DE();
        Self::push_u16(cpu, bus, de);
        16
    }

    pub fn pop_de(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = Self::pop_u16(cpu, bus);
        cpu.write_DE(value);
        12
    }

    pub fn push_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let hl = cpu.HL();
        Self::push_u16(cpu, bus, hl);
        16
    }

    pub fn pop_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = Self::pop_u16(cpu, bus);
        cpu.write_HL(value);
        12
    }

}