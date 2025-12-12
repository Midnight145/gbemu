#![allow(unused_variables)]

use crate::cpu::Bus::Bus;
use crate::cpu::CPU::{Flags, CPU};

// 8bit inc/dec operations
impl CPU {
    fn inc_r8(reg: &mut u8, flags: &mut Flags) {
        flags.H = flags.check_half_carry_add_u8(*reg, 1, 0);
        *reg = reg.wrapping_add(1);
        flags.Z = *reg == 0;
        flags.N = false;
    }

    fn dec_r8(reg: &mut u8, flags: &mut Flags) {
        flags.H = flags.check_half_carry_sub_u8(*reg, 1, 0);
        *reg = reg.wrapping_sub(1);
        flags.Z = *reg == 0;
        flags.N = true;
    }

    pub fn inc_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::inc_r8(&mut cpu.B, &mut cpu.flags); 4
    }

    pub fn dec_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::dec_r8(&mut cpu.B, &mut cpu.flags); 4
    }

    pub fn inc_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::inc_r8(&mut cpu.C, &mut cpu.flags); 4
    }

    pub fn dec_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::dec_r8(&mut cpu.C, &mut cpu.flags); 4
    }

    pub fn inc_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::inc_r8(&mut cpu.D, &mut cpu.flags); 4
    }

    pub fn dec_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::dec_r8(&mut cpu.D, &mut cpu.flags); 4
    }

    pub fn inc_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::inc_r8(&mut cpu.E, &mut cpu.flags); 4
    }

    pub fn dec_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::dec_r8(&mut cpu.E, &mut cpu.flags); 4
    }


    pub fn inc_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::inc_r8(&mut cpu.H, &mut cpu.flags); 4
    }

    pub fn dec_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::dec_r8(&mut cpu.H, &mut cpu.flags); 4
    }

    pub fn inc_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::inc_r8(&mut cpu.L, &mut cpu.flags); 4
    }

    pub fn dec_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::dec_r8(&mut cpu.L, &mut cpu.flags); 4
    }

    pub fn inc_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::inc_r8(&mut cpu.A, &mut cpu.flags); 4
    }

    pub fn dec_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::dec_r8(&mut cpu.A, &mut cpu.flags); 4
    }

    pub fn inc_hl_ptr(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let byte = bus.read_byte(cpu.HL());
        let added = byte.wrapping_add(1);
        cpu.flags.H = cpu.flags.check_half_carry_add_u8(byte, 1, 0);
        cpu.flags.Z = added == 0;
        cpu.flags.N = false;
        bus.write_byte(cpu.HL(), added);
        12
    }

    pub fn dec_hl_ptr(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let byte = bus.read_byte(cpu.HL());
        let sub = byte.wrapping_sub(1);
        cpu.flags.H = cpu.flags.check_half_carry_sub_u8(byte, 1, 0);
        cpu.flags.Z = sub == 0;
        cpu.flags.N = true;
        bus.write_byte(cpu.HL(), sub);
        12
    }
}

// 16bit inc/dec operations
impl CPU {
    pub fn inc_bc(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.write_BC(cpu.BC().wrapping_add(1)); 8
    }

    pub fn dec_bc(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.write_BC(cpu.BC().wrapping_sub(1)); 8
    }

    pub fn inc_de(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.write_DE(cpu.DE().wrapping_add(1)); 8
    }

    pub fn dec_de(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.write_DE(cpu.DE().wrapping_sub(1)); 8
    }

    pub fn inc_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.write_HL(cpu.HL().wrapping_add(1)); 8
    }

    pub fn dec_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.write_HL(cpu.HL().wrapping_sub(1)); 8
    }

    pub fn inc_sp(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.SP = cpu.SP.wrapping_add(1); 8
    }

    pub fn dec_sp(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.SP = cpu.SP.wrapping_sub(1); 8
    }
}

// 8bit add/sub register-register operations
impl CPU {
    pub fn add_r8_r8(reg1: &mut u8, reg2: u8, flags: &mut Flags) {
        flags.H = flags.check_half_carry_add_u8(*reg1, reg2, 0);
        flags.C = flags.check_full_carry_add_u8(*reg1, reg2, 0);
        *reg1 = reg1.wrapping_add(reg2);
        flags.Z = *reg1 == 0;
        flags.N = false;
    }

    pub fn sub_r8_r8(reg1: &mut u8, reg2: u8, flags: &mut Flags) {
        flags.H = flags.check_half_carry_sub_u8(*reg1, reg2, 0);
        flags.C = flags.check_full_carry_sub_u8(*reg1, reg2, 0);
        *reg1 = reg1.wrapping_sub(reg2);
        flags.Z = *reg1 == 0;
        flags.N = true;
    }

    pub fn add_a_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let a = cpu.A;
        Self::add_r8_r8(&mut cpu.A, a, &mut cpu.flags); 4
    }


    pub fn add_a_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::add_r8_r8(&mut cpu.A, cpu.B, &mut cpu.flags); 4
    }


    pub fn add_a_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::add_r8_r8(&mut cpu.A, cpu.C, &mut cpu.flags); 4
    }


    pub fn add_a_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::add_r8_r8(&mut cpu.A, cpu.D, &mut cpu.flags); 4
    }


    pub fn add_a_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::add_r8_r8(&mut cpu.A, cpu.E, &mut cpu.flags); 4
    }


    pub fn add_a_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::add_r8_r8(&mut cpu.A, cpu.H, &mut cpu.flags); 4
    }

    pub fn add_a_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::add_r8_r8(&mut cpu.A, cpu.L, &mut cpu.flags); 4
    }

    pub fn add_a_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = bus.read_byte(cpu.HL());
        Self::add_r8_r8(&mut cpu.A, value, &mut cpu.flags);
        8
    }


    pub fn sub_a_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let a = cpu.A;
        Self::sub_r8_r8(&mut cpu.A, a, &mut cpu.flags); 4
    }


    pub fn sub_a_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::sub_r8_r8(&mut cpu.A, cpu.B, &mut cpu.flags); 4
    }


    pub fn sub_a_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::sub_r8_r8(&mut cpu.A, cpu.C, &mut cpu.flags); 4
    }


    pub fn sub_a_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::sub_r8_r8(&mut cpu.A, cpu.D, &mut cpu.flags); 4
    }


    pub fn sub_a_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::sub_r8_r8(&mut cpu.A, cpu.E, &mut cpu.flags); 4
    }


    pub fn sub_a_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::sub_r8_r8(&mut cpu.A, cpu.H, &mut cpu.flags); 4
    }


    pub fn sub_a_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::sub_r8_r8(&mut cpu.A, cpu.L, &mut cpu.flags); 4
    }

    pub fn sub_a_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = bus.read_byte(cpu.HL());
        Self::sub_r8_r8(&mut cpu.A, value, &mut cpu.flags);
        8
    }
}

// 16bit add operations
impl CPU {
    fn add_hl_r16(cpu: &mut CPU, r16: u16) {
        let lhs = cpu.HL();
        let rhs = r16;

        cpu.flags.N = false;
        cpu.flags.H = cpu.flags.check_half_carry_add_u16(lhs, rhs);
        cpu.flags.C = cpu.flags.check_full_carry_add_u16(lhs, rhs);

        cpu.write_HL(lhs.wrapping_add(rhs));
    }

    pub fn add_hl_bc(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let rhs = cpu.BC();
        Self::add_hl_r16(cpu, rhs);
        12
    }

    pub fn add_hl_de(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let rhs = cpu.DE();
        Self::add_hl_r16(cpu, rhs);
        12
    }

    pub fn add_hl_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let rhs = cpu.HL();
        Self::add_hl_r16(cpu, rhs);
        12
    }

    pub fn add_hl_sp(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let rhs = cpu.SP;
        Self::add_hl_r16(cpu, rhs);
        12
    }

    pub fn add_sp_e8(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let imm = cpu.read_u8_from_pc(bus) as i8 as i16 as u16;
        cpu.flags.Z = false;
        cpu.flags.N = false;
        cpu.flags.H = ((cpu.SP & 0xF) + (imm & 0xF)) > 0xF;
        cpu.flags.C = ((cpu.SP & 0xFF) + (imm & 0xFF)) > 0xFF;
        cpu.SP = cpu.SP.wrapping_add(imm);
        12
    }
}

// 8bit adc/sbc register-register operations
impl CPU {
    pub fn adc_r8_r8(reg1: &mut u8, reg2: u8, flags: &mut Flags) {
        let carry = flags.C as u8;
        let a = *reg1;
        let b = reg2;
        *reg1 = reg1.wrapping_add(b.wrapping_add(carry));
        flags.Z = *reg1 == 0;
        flags.N = false;
        flags.H = flags.check_half_carry_add_u8(a, b, carry);
        flags.C = flags.check_full_carry_add_u8(a, b, carry);
    }

    pub fn sbc_r8_r8(reg1: &mut u8, reg2: u8, flags: &mut Flags) {
        let carry = flags.C as u8;
        let a = *reg1;
        let b = reg2;
        *reg1 = reg1.wrapping_sub(b.wrapping_add(carry));
        flags.Z = *reg1 == 0;
        flags.N = true;
        flags.H = flags.check_half_carry_sub_u8(a, b, carry);
        flags.C = flags.check_full_carry_sub_u8(a, b, carry);
    }


    pub fn adc_a_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let a = cpu.A;
        Self::adc_r8_r8(&mut cpu.A, a, &mut cpu.flags); 4
    }


    pub fn adc_a_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::adc_r8_r8(&mut cpu.A, cpu.B, &mut cpu.flags); 4
    }


    pub fn adc_a_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::adc_r8_r8(&mut cpu.A, cpu.C, &mut cpu.flags); 4
    }


    pub fn adc_a_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::adc_r8_r8(&mut cpu.A, cpu.D, &mut cpu.flags); 4
    }


    pub fn adc_a_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::adc_r8_r8(&mut cpu.A, cpu.E, &mut cpu.flags); 4
    }


    pub fn adc_a_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::adc_r8_r8(&mut cpu.A, cpu.H, &mut cpu.flags); 4
    }

    pub fn adc_a_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::adc_r8_r8(&mut cpu.A, cpu.L, &mut cpu.flags); 4
    }

    pub fn adc_a_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = bus.read_byte(cpu.HL());
        Self::adc_r8_r8(&mut cpu.A, value, &mut cpu.flags);
        8
    }

    pub fn sbc_a_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let a = cpu.A;
        Self::sbc_r8_r8(&mut cpu.A, a, &mut cpu.flags); 4
    }

    pub fn sbc_a_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::sbc_r8_r8(&mut cpu.A, cpu.B, &mut cpu.flags); 4
    }

    pub fn sbc_a_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::sbc_r8_r8(&mut cpu.A, cpu.C, &mut cpu.flags); 4
    }

    pub fn sbc_a_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::sbc_r8_r8(&mut cpu.A, cpu.D, &mut cpu.flags); 4
    }

    pub fn sbc_a_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::sbc_r8_r8(&mut cpu.A, cpu.E, &mut cpu.flags); 4
    }

    pub fn sbc_a_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::sbc_r8_r8(&mut cpu.A, cpu.H, &mut cpu.flags); 4
    }

    pub fn sbc_a_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::sbc_r8_r8(&mut cpu.A, cpu.L, &mut cpu.flags); 4
    }

    pub fn sbc_a_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = bus.read_byte(cpu.HL());
        Self::sbc_r8_r8(&mut cpu.A, value, &mut cpu.flags);
        8
    }
}

// Bitwise operations
impl CPU {
    pub fn and_r8_r8(reg1: &mut u8, reg2: u8, flags: &mut Flags) {
        *reg1 = *reg1 & reg2;
        flags.Z = *reg1 == 0;
        flags.N = false;
        flags.H = true;
        flags.C = false;
    }

    pub fn xor_r8_r8(reg1: &mut u8, reg2: u8, flags: &mut Flags) {
        *reg1 = *reg1 ^ reg2;
        flags.Z = *reg1 == 0;
        flags.N = false;
        flags.H = false;
        flags.C = false;
    }

    pub fn or_r8_r8(reg1: &mut u8, reg2: u8, flags: &mut Flags) {
        *reg1 = *reg1 | reg2;
        flags.Z = *reg1 == 0;
        flags.N = false;
        flags.H = false;
        flags.C = false;
    }

    pub fn cp_r8_r8(reg1: &mut u8, reg2: u8, flags: &mut Flags) {

        flags.H = flags.check_half_carry_sub_u8(*reg1, reg2, 0);
        flags.C = flags.check_full_carry_sub_u8(*reg1, reg2, 0);
        flags.Z = reg1.wrapping_sub(reg2) == 0;
        flags.N = true;
    }

    pub fn and_a_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let a = cpu.A;
        Self::and_r8_r8(&mut cpu.A, a, &mut cpu.flags); 4
    }


    pub fn and_a_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::and_r8_r8(&mut cpu.A, cpu.B, &mut cpu.flags); 4
    }


    pub fn and_a_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::and_r8_r8(&mut cpu.A, cpu.C, &mut cpu.flags); 4
    }


    pub fn and_a_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::and_r8_r8(&mut cpu.A, cpu.D, &mut cpu.flags); 4
    }


    pub fn and_a_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::and_r8_r8(&mut cpu.A, cpu.E, &mut cpu.flags); 4
    }


    pub fn and_a_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::and_r8_r8(&mut cpu.A, cpu.H, &mut cpu.flags); 4
    }

    pub fn and_a_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::and_r8_r8(&mut cpu.A, cpu.L, &mut cpu.flags); 4
    }

    pub fn and_a_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = bus.read_byte(cpu.HL());
        Self::and_r8_r8(&mut cpu.A, value, &mut cpu.flags);
        8
    }

    pub fn xor_a_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let a = cpu.A;
        Self::xor_r8_r8(&mut cpu.A, a, &mut cpu.flags); 4
    }


    pub fn xor_a_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::xor_r8_r8(&mut cpu.A, cpu.B, &mut cpu.flags); 4
    }


    pub fn xor_a_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::xor_r8_r8(&mut cpu.A, cpu.C, &mut cpu.flags); 4
    }


    pub fn xor_a_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::xor_r8_r8(&mut cpu.A, cpu.D, &mut cpu.flags); 4
    }


    pub fn xor_a_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::xor_r8_r8(&mut cpu.A, cpu.E, &mut cpu.flags); 4
    }


    pub fn xor_a_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::xor_r8_r8(&mut cpu.A, cpu.H, &mut cpu.flags); 4
    }

    pub fn xor_a_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::xor_r8_r8(&mut cpu.A, cpu.L, &mut cpu.flags); 4
    }

    pub fn xor_a_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = bus.read_byte(cpu.HL());
        Self::xor_r8_r8(&mut cpu.A, value, &mut cpu.flags);
        8
    }

    pub fn or_a_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let a = cpu.A;
        Self::or_r8_r8(&mut cpu.A, a, &mut cpu.flags); 4
    }


    pub fn or_a_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::or_r8_r8(&mut cpu.A, cpu.B, &mut cpu.flags); 4
    }


    pub fn or_a_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::or_r8_r8(&mut cpu.A, cpu.C, &mut cpu.flags); 4
    }


    pub fn or_a_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::or_r8_r8(&mut cpu.A, cpu.D, &mut cpu.flags); 4
    }


    pub fn or_a_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::or_r8_r8(&mut cpu.A, cpu.E, &mut cpu.flags); 4
    }


    pub fn or_a_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::or_r8_r8(&mut cpu.A, cpu.H, &mut cpu.flags); 4
    }

    pub fn or_a_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::or_r8_r8(&mut cpu.A, cpu.L, &mut cpu.flags); 4
    }

    pub fn or_a_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = bus.read_byte(cpu.HL());
        Self::or_r8_r8(&mut cpu.A, value, &mut cpu.flags);
        8
    }

    pub fn cp_a_a(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let a = cpu.A;
        Self::cp_r8_r8(&mut cpu.A, a, &mut cpu.flags); 4
    }

    pub fn cp_a_b(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::cp_r8_r8(&mut cpu.A, cpu.B, &mut cpu.flags); 4
    }

    pub fn cp_a_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::cp_r8_r8(&mut cpu.A, cpu.C, &mut cpu.flags); 4
    }

    pub fn cp_a_d(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::cp_r8_r8(&mut cpu.A, cpu.D, &mut cpu.flags); 4
    }

    pub fn cp_a_e(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::cp_r8_r8(&mut cpu.A, cpu.E, &mut cpu.flags); 4
    }

    pub fn cp_a_h(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::cp_r8_r8(&mut cpu.A, cpu.H, &mut cpu.flags); 4
    }

    pub fn cp_a_l(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::cp_r8_r8(&mut cpu.A, cpu.L, &mut cpu.flags); 4
    }

    pub fn cp_a_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = bus.read_byte(cpu.HL());
        Self::cp_r8_r8(&mut cpu.A, value, &mut cpu.flags);
        8
    }
}

// 8bit reg/value operations
impl CPU {
    pub fn add_a_u8(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let byte = cpu.read_u8_from_pc(bus);
        Self::add_r8_r8(&mut cpu.A, byte, &mut cpu.flags);
        8
    }

    pub fn sub_a_u8(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let byte = cpu.read_u8_from_pc(bus);
        Self::sub_r8_r8(&mut cpu.A, byte, &mut cpu.flags);
        8
    }

    pub fn adc_a_u8(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let byte = cpu.read_u8_from_pc(bus);
        Self::adc_r8_r8(&mut cpu.A, byte, &mut cpu.flags);
        8
    }

    pub fn sbc_a_u8(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let byte = cpu.read_u8_from_pc(bus);
        Self::sbc_r8_r8(&mut cpu.A, byte, &mut cpu.flags);
        8
    }

    pub fn and_a_u8(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let byte = cpu.read_u8_from_pc(bus);
        Self::and_r8_r8(&mut cpu.A, byte, &mut cpu.flags);
        8
    }

    pub fn xor_a_u8(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let byte = cpu.read_u8_from_pc(bus);
        Self::xor_r8_r8(&mut cpu.A, byte, &mut cpu.flags);
        8
    }

    pub fn or_a_u8(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let byte = cpu.read_u8_from_pc(bus);
        Self::or_r8_r8(&mut cpu.A, byte, &mut cpu.flags);
        8
    }

    pub fn cp_a_u8(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let byte = cpu.read_u8_from_pc(bus);
        Self::cp_r8_r8(&mut cpu.A, byte, &mut cpu.flags);
        8
    }
}

// weird one-off instructions
impl CPU {
    pub fn daa(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let mut adjustment = 0u8;
        if cpu.flags.N {
            if cpu.flags.H {
                adjustment |= 0x6
            }
            if cpu.flags.C {
                adjustment |= 0x60
            }
            cpu.A = cpu.A.wrapping_sub(adjustment)
        } else {
            if cpu.flags.H || (cpu.A & 0xF > 0x9) {
                adjustment |= 0x6
            }
            if cpu.flags.C || cpu.A > 0x99 {
                adjustment |= 0x60;
                cpu.flags.C = true;
            }
            cpu.A = cpu.A.wrapping_add(adjustment)
        }
        cpu.flags.H = false;
        cpu.flags.Z = cpu.A == 0;
        4
    }

    pub fn cpl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.A = !cpu.A;
        cpu.flags.N = true;
        cpu.flags.H = true;
        4
    }

    pub fn scf(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.flags.N = false;
        cpu.flags.H = false;
        cpu.flags.C = true;
        4
    }

    pub fn ccf(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.flags.N = false;
        cpu.flags.H = false;
        cpu.flags.C = !cpu.flags.C;
        4
    }
}