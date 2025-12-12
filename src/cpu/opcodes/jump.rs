#![allow(unused_variables)]


use crate::cpu::Bus::Bus;
use crate::cpu::CPU::CPU;

// JR insns
impl CPU {
    pub fn jr_e8(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let offset = cpu.read_u8_from_pc(bus) as i8;
        cpu.PC = cpu.PC.wrapping_add(offset as i16 as u16);
        12
    }

    pub fn jr_nz_e8(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let offset = cpu.read_u8_from_pc(bus) as i8;
        if !cpu.flags.Z {
            cpu.PC = cpu.PC.wrapping_add(offset as u16);
            return 12;
        }
        8
    }

    pub fn jr_z_e8(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let offset = cpu.read_u8_from_pc(bus) as i8;
        if cpu.flags.Z {
            cpu.PC = cpu.PC.wrapping_add(offset as i16 as u16);
            return 12;
        }
        8
    }

    pub fn jr_nc_e8(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let offset = cpu.read_u8_from_pc(bus) as i8;
        if !cpu.flags.C {
            cpu.PC = cpu.PC.wrapping_add(offset as i16 as u16);
            return 12;
        }
        8
    }

    pub fn jr_c_e8(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let offset = cpu.read_u8_from_pc(bus) as i8;
        if cpu.flags.C {
            cpu.PC = cpu.PC.wrapping_add(offset as i16 as u16);
            return 12;
        }
        8
    }
}

// JP insns
impl CPU {
    pub fn jp_a16(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let addr = cpu.read_u16_from_pc(bus);
        cpu.PC = addr;
        16
    }

    pub fn jp_hl(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        cpu.PC = cpu.HL();
        4
    }

    pub fn jp_nz_a16(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let addr = cpu.read_u16_from_pc(bus);
        if !cpu.flags.Z {
            cpu.PC = addr;
            return 16;
        }
        12
    }

    pub fn jp_z_a16(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let addr = cpu.read_u16_from_pc(bus);
        if cpu.flags.Z {
            cpu.PC = addr;
            return 16;
        }
        12
    }

    pub fn jp_nc_a16(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let addr = cpu.read_u16_from_pc(bus);
        if !cpu.flags.C {
            cpu.PC = addr;
            return 16;
        }
        12
    }

    pub fn jp_c_a16(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let addr = cpu.read_u16_from_pc(bus);
        if cpu.flags.C {
            cpu.PC = addr;
            return 16;
        }
        12
    }
}

// Call/RST insns
impl CPU {
    pub fn call_a16(cpu: &mut CPU, bus: &mut Bus) -> u8 {

        let addr = cpu.read_u16_from_pc(bus);
        Self::push_u16(cpu, bus, cpu.PC);
        cpu.PC = addr;
        24
    }

    pub fn call_nz_a16(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let addr = cpu.read_u16_from_pc(bus);
        if !cpu.flags.Z {
            Self::push_u16(cpu, bus, cpu.PC);
            cpu.PC = addr;
            return 24;
        }
        12
    }

    pub fn call_z_a16(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let addr = cpu.read_u16_from_pc(bus);
        if cpu.flags.Z {
            Self::push_u16(cpu, bus, cpu.PC);
            cpu.PC = addr;
            return 24;
        }
        12
    }

    pub fn call_nc_a16(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let addr = cpu.read_u16_from_pc(bus);
        if !cpu.flags.C {
            Self::push_u16(cpu, bus, cpu.PC);
            cpu.PC = addr;
            return 24;
        }
        12
    }

    pub fn call_c_a16(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let addr = cpu.read_u16_from_pc(bus);
        if cpu.flags.C {
            Self::push_u16(cpu, bus, cpu.PC);
            cpu.PC = addr;
            return 24;
        }
        12
    }
}

// RST opcodes

impl CPU {
    pub fn rst(cpu: &mut CPU, bus: &mut Bus, addr: u16) {
        Self::push_u16(cpu, bus, cpu.PC);  // push return address
        cpu.PC = addr
    }

    pub fn rst_00(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::push_u16(cpu, bus, cpu.PC);  // push return address
        cpu.PC = 0x00;
        16
    }

    pub fn rst_08(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::push_u16(cpu, bus, cpu.PC);  // push return address
        cpu.PC = 0x08;
        16
    }

    pub fn rst_10(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::push_u16(cpu, bus, cpu.PC);  // push return address
        cpu.PC = 0x10;
        16
    }

    pub fn rst_18(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::push_u16(cpu, bus, cpu.PC);  // push return address
        cpu.PC = 0x18;
        16
    }

    pub fn rst_20(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::push_u16(cpu, bus, cpu.PC);  // push return address
        cpu.PC = 0x20;
        16
    }

    pub fn rst_28(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::push_u16(cpu, bus, cpu.PC);  // push return address
        cpu.PC = 0x28;
        16
    }

    pub fn rst_30(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::push_u16(cpu, bus, cpu.PC);  // push return address
        cpu.PC = 0x30;
        16
    }

    pub fn rst_38(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::push_u16(cpu, bus, cpu.PC);  // push return address
        cpu.PC = 0x38;
        16
    }
}

// RET insns
impl CPU {
    pub fn ret(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        let value = Self::pop_u16(cpu, bus);
        cpu.PC = value;
        16
    }

    pub fn ret_nz(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        if !cpu.flags.Z {
            let value = Self::pop_u16(cpu, bus);
            cpu.PC = value;
            return 20;
        }
        8
    }

    pub fn ret_z(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        if cpu.flags.Z {
            let value = Self::pop_u16(cpu, bus);
            cpu.PC = value;
            return 20;
        }
        8
    }

    pub fn ret_nc(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        if !cpu.flags.C {
            let value = Self::pop_u16(cpu, bus);
            cpu.PC = value;
            return 20;
        }
        8
    }

    pub fn ret_c(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        if cpu.flags.C {
            let value = Self::pop_u16(cpu, bus);
            cpu.PC = value;
            return 20;
        }
        8
    }

    pub fn reti(cpu: &mut CPU, bus: &mut Bus) -> u8 {
        Self::ret(cpu, bus);
        cpu.ime = true;
        16
    }
}