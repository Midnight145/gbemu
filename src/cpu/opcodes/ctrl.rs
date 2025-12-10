use crate::cpu::Bus::Bus;
use crate::cpu::CPU::CPU;
use crate::cpu::opcodes::PREFIXED_OPCODES;

// Control OPCODES
impl CPU {
    pub fn nop(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        4
    }

    pub fn stop_n8(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.read_u8_from_pc(bus)
        // todo: https://gbdev.io/pandocs/Reducing_Power_Consumption.html#using-the-stop-instruction
    }

    pub fn ei(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.ime_scheduled = true;
        4
    }

    pub fn di(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.ime = false;
        4
    }

    pub fn halt(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.halt = true;
        4
    }

    pub fn prefix(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let opcode = cpu.read_u8_from_pc(bus);
        4 + PREFIXED_OPCODES[opcode as usize](cpu, bus)
    }
    
    pub fn invalid(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.executed_invalid = true;
        4
    }
}