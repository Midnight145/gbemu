use std::fs;
use crate::cartridge::{header, mbc};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::cpu::Bus::Bus;
use crate::cpu::CPU::CPU;
use serde::Deserialize;
use serde_json;
use crate::tests::{TestCase, TestFile};

mod cpu;
mod cartridge;
mod tests;


fn main() {
    let path = "./sm83/v1/";
    let readdir = std::fs::read_dir(path);
    match readdir {
        Err(_) => panic!("failed to open directory"),
        Ok(_) => {}
    }
    let files = readdir.unwrap();
    for file in files {
        if file.is_ok() {
            let f = file.unwrap();
            let contents = fs::read_to_string(f.path());
            if contents.is_ok() {
                let json = serde_json::from_str::<TestFile>(&contents.unwrap());
                if json.is_ok() {
                    let cases = json.unwrap();
                    for case in cases {
                        if case.initial.pc <= 0xFEFF && case.initial.pc >= 0xFEA0 {
                            continue; // unused range
                        }
                        let (mut cpu, mut bus) = tests::create(&case);
                        // cpu.reset();
                        for value in case.initial.ram.iter().clone() {
                            bus.write_byte(value.0, value.1);
                            // println!("addr: {:04x}, value: {:02x}", value.0, value.1);
                            // assert_eq!(value.1, bus.read_byte(value.0));
                        }
                        tests::execute(&mut cpu, &mut bus);
                        tests::check(case, cpu, bus)
                    }
                } else {
                    let err = json.err().unwrap();
                    println!("{:?}: {}", f.file_name(), err);
                }
            }
        }
    }
}

fn test_cpuinstrs() {
    let path = Path::new("test.gb");

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
        println!("Executing opcode: {:x}", opcode);

        print!("A:{:02x} F:{} BC:{:04x} DE:{:04x} HL:{:04x} SP:{:04x} PC:{:04x} (cy: {}) | {:02x} ", cpu.A, cpu.flags.to_str(), cpu.BC(), cpu.DE(), cpu.HL(), cpu.SP, cpu.PC, total_cycles, opcode);
        cpu.PC = cpu.PC.wrapping_add(1);
        let op_cycles = cpu::opcodes::OPCODES[opcode as usize](&mut cpu, &mut bus);
        total_cycles += op_cycles as u128;
        bus.timer.update(op_cycles as u16);
        instr += 1;
        println!()
    }
}
