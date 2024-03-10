mod cpu;

use crate::cpu::*;
use crate::cpu::defs::*;

fn main() {
    let cpu: CPU = CPU::new();
    println!("CPU: {:#?}", cpu);
}
