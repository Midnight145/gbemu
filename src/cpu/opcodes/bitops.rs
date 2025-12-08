use crate::cpu::Bus::Bus;
use crate::cpu::CPU::CPU;

impl CPU {
    pub fn rlca(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let carry =  cpu.A & 0b1000_0000 != 0;
        cpu.A = (cpu.A << 1) | (carry as u8);
        cpu.flags.C = carry;

        cpu.flags.H = false;
        cpu.flags.N = false;
        cpu.flags.Z = false;
        4
    }

    pub fn rrca(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let carry =  cpu.A & 0b1 != 0;
        cpu.A = (cpu.A >> 1) | ((carry as u8) << 7);
        cpu.flags.C = carry;

        cpu.flags.H = false;
        cpu.flags.N = false;
        cpu.flags.Z = false;
        4
    }

    pub fn rla(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let carry = cpu.flags.C as u8;
        cpu.flags.C = cpu.A & 0b1000_0000 != 0;
        cpu.A = (cpu.A << 1) | carry;
        cpu.flags.H = false;
        cpu.flags.N = false;
        cpu.flags.Z = false;
        4
    }

    pub fn rra(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let carry = cpu.flags.C as u8;
        cpu.flags.C = cpu.A & 0b1 != 0;
        cpu.A = (cpu.A >> 1) | (carry << 7);
        cpu.flags.H = false;
        cpu.flags.N = false;
        cpu.flags.Z = false;
        4
    }
}