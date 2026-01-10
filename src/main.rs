#![allow(dead_code)]

mod cpu;

mod opcodes;

fn main() {
    let cpu = cpu::R3000::new();
    let running = true;
    while running {
        cpu.fetch_decode();
    }
}
