use crate::cartridge::{header, mbc};
use crate::cpu::Bus::Bus;
use crate::cpu::CPU::CPU;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod cpu;
mod cartridge;
mod tests;
mod state;
mod registers;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"-v".to_string()) || args.contains(&"--verbose".to_string()) {
        let mut state = state::STATE.lock().unwrap();
        state.DEBUG = true;
    }
    if args.contains(&"--test".to_string()) {
        tests::run_tests();
    } else if args.contains(&"--cpuinstrs".to_string()) {
    }
    test_cpuinstrs();
}

fn test_cpuinstrs() {
    let path = Path::new("cpu_instrs.gb");

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("couldn't open file")
    };
    let mut bytes = Vec::new();
    match file.read_to_end(&mut bytes) {
        Ok(_) => {}
        Err(_) => {panic!("failed to read file")}
    }

    let header = header::Header::new(&bytes);
    let cartridge = mbc::new(&header, bytes);
    let mut cpu = CPU::new();
    let mut bus = Bus::new(cartridge);

    let mut total_cycles: u128 = 0;
    let mut instr = 0;
    bus.write_byte(0xFF44, 0x90);
    loop {
        let opcode = bus.read_byte(cpu.PC);
        if state::STATE.lock().unwrap().DEBUG {
            print!("A:{:02x} F:{} BC:{:04x} DE:{:04x} HL:{:04x} SP:{:04x} PC:{:04x} (cy: {}) | {:02x} ", cpu.A, cpu.flags.to_str(), cpu.BC(), cpu.DE(), cpu.HL(), cpu.SP, cpu.PC, total_cycles, opcode);
        }
        cpu.PC = cpu.PC.wrapping_add(1);
        let op_cycles = cpu::opcodes::OPCODES[opcode as usize](&mut cpu, &mut bus);
        total_cycles += op_cycles as u128;
        bus.timer.update(op_cycles as u16);
        instr += 1;
    }
}
