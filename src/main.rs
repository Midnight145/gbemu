use crate::cpu::Bus::Bus;
use crate::cpu::CPU::CPU;

mod cpu;

fn main() {
    let _cpu = CPU::new();
    let _bus = Bus::new();
    println!("Hello, world!");
}
