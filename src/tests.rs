use crate::cpu::Bus::Bus;
use crate::cpu::CPU::CPU;
use crate::{cpu, tests, state};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

pub type TestFile = Vec<TestCase>;

#[derive(Serialize, Deserialize, Debug)]
pub struct TestCase {
    pub name: String,
    pub initial: CpuInitialState,
    #[serde(rename = "final")]
    pub final_: CpuFinalState,

    #[serde(rename = "cycles")]
    pub cycles: Vec<CycleEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CpuInitialState {
    pub pc: u16,
    pub sp: u16,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub ime: u8,
    pub ie: u8,

    pub ram: Vec<RamEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CpuFinalState {
    pub pc: u16,
    pub sp: u16,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub ime: u8,

    pub ram: Vec<RamEntry>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct RamEntry(pub u16, pub u8);

#[derive(Serialize, Deserialize, Debug)]
pub struct CycleEntry(
    pub u16,
    pub Option<u8>,
    pub String
);

pub fn create(case: &TestCase) -> (CPU, Bus) {
    let mut cpu = CPU::new();
    let mut bus = Bus::test();
    cpu.A = case.initial.a;
    cpu.B = case.initial.b;
    cpu.C = case.initial.c;
    cpu.D = case.initial.d;
    cpu.E = case.initial.e;
    cpu.F = case.initial.f;
    cpu.H = case.initial.h;
    cpu.L = case.initial.l;
    cpu.ime = if case.initial.ime > 0 {
        true
    } else {
        false
    };
    cpu.flags = cpu::CPU::Flags::from_u8(case.initial.f);
    cpu.PC = case.initial.pc;
    cpu.SP = case.initial.sp;
    bus.interrupt_enable = case.initial.ie;


    (cpu, bus)
}

pub fn execute(cpu: &mut CPU, bus: &mut Bus) {
    let opcode = bus.read_byte(cpu.PC);
    cpu.PC = cpu.PC.wrapping_add(1);
    let op_cycles = cpu::opcodes::OPCODES[opcode as usize](cpu, bus);
    cpu.F = cpu.flags.to_u8();
}


pub fn check(case: TestCase, cpu: CPU, bus: Bus) -> bool {
    let debug =  state::STATE.lock().unwrap().DEBUG;

    let mut failed = false;
    let mut log = String::new();

    log.push_str(&format!("==== FAILED TEST: {} ====\n", case.name));
    log.push_str("Initial State:\n");
    log.push_str(&format!("{:#?}\n", case.initial));
    log.push_str("Expected Final State:\n");
    log.push_str(&format!("{:#?}\n", case.final_));
    log.push_str("Actual Final State:\n");
    log.push_str(&format!(
        "A:{:#04x} B:{:#04x} C:{:#04x} D:{:#04x} E:{:#04x} F:{:#04x} H:{:#04x} L:{:#04x} \
         PC:{:#06x} SP:{:#06x} IME:{}\n",
        cpu.A, cpu.B, cpu.C, cpu.D, cpu.E, cpu.F,
        cpu.H, cpu.L, cpu.PC, cpu.SP, cpu.ime
    ));
    log.push_str("Mismatches:\n");

    macro_rules! check_reg {
        ($actual:expr, $expected:expr, $name:expr) => {
            if $actual != $expected {
                println!(
                    "  {} mismatch: expected {:#04x}, got {:#04x}",
                    $name, $expected, $actual
                );
                log.push_str(&format!(
                    "  {} mismatch: expected {:#04x}, got {:#04x}\n",
                    $name, $expected, $actual
                ));
                failed = true;
            }
        };
    }

    check_reg!(cpu.A, case.final_.a, "A");
    check_reg!(cpu.B, case.final_.b, "B");
    check_reg!(cpu.C, case.final_.c, "C");
    check_reg!(cpu.D, case.final_.d, "D");
    check_reg!(cpu.E, case.final_.e, "E");
    check_reg!(cpu.F, case.final_.f, "F");
    check_reg!(cpu.H, case.final_.h, "H");
    check_reg!(cpu.L, case.final_.l, "L");
    check_reg!(cpu.PC, case.final_.pc, "PC");
    check_reg!(cpu.SP, case.final_.sp, "SP");

    let expected_ime = case.final_.ime != 0;
    if cpu.ime != expected_ime {
        if debug {
            println!(
                "  IME mismatch: expected {}, got {}",
                expected_ime, cpu.ime
            );
        }
        log.push_str(&format!(
            "  IME mismatch: expected {}, got {}\n",
            expected_ime, cpu.ime
        ));
        failed = true;
    }

    for entry in case.final_.ram {
        let actual = bus.read_byte(entry.0);
        if actual != entry.1 {
            if debug {
                println!(
                    "  RAM[{:#06x}] mismatch: expected {:#04x}, got {:#04x}",
                    entry.0, entry.1, actual
                );
            }
            log.push_str(&format!(
                "  RAM[{:#06x}] mismatch: expected {:#04x}, got {:#04x}\n",
                entry.0, entry.1, actual
            ));
            failed = true;
        }
    }

    if failed {
        if debug {
            println!("RESULT: FAILED\n");
        }

        log.push('\n');

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("failed_tests.log")
            .expect("Could not open failed_tests.log");

        file.write_all(log.as_bytes()).unwrap();
        return false
    }
    true
}


pub fn run_tests() {
    let path = "./sm83/v1/";
    let readdir = std::fs::read_dir(path);
    match readdir {
        Err(_) => panic!("failed to open directory"),
        Ok(_) => {}
    }
    let mut test_count = 0;
    let mut succeeded = 0;
    let mut failed = 0;
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

                        let (mut cpu, mut bus) = tests::create(&case);
                        for value in case.initial.ram.iter().clone() {
                            bus.write_byte(value.0, value.1);
                        }

                        execute(&mut cpu, &mut bus);
                        if check(case, cpu, bus) {
                            succeeded += 1
                        } else {
                            failed += 1
                        }
                        test_count += 1;
                    }
                } else {
                    let err = json.err().unwrap();
                    println!("{:?}: {}", f.file_name(), err);
                }
            }
        }
    }
    println!("Tests complete. {}/{} passed, {}/{} failed. | {:02}% success rate.", succeeded, test_count, failed, test_count, ((succeeded as f32) / (test_count as f32) * 100f32));
}

