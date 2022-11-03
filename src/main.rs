mod cpu;
mod ram;

use cpu::CPU;
use std::{env, process};

fn main() {
    println!("Chip 8 emulator");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let cpu = CPU::build();

    if let Err(e) = cpu.run(file_path) {
        eprintln!("CPU error: {e}");
        process::exit(1)
    }
}