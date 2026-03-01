use crate::psx::mem::{
    BIOS_WORDS, EXPANSION_1_WORDS, EXPANSION_2_WORDS, EXPANSION_3_WORDS, IO_CACHE_WORDS, IO_WORDS,
    MAIN_RAM_WORDS, SCRATCHPAD_WORDS,
};

mod cpu;
mod mem;

pub struct PSX {
    cpu: cpu::CPU,
    mem: mem::MemBus,
}

impl PSX {
    pub fn new() -> PSX {
        PSX {
            cpu: cpu::CPU::new(),
            mem: mem::MemBus {
                main_ram: [0; MAIN_RAM_WORDS],
                expansion_1: [0; EXPANSION_1_WORDS],
                scratchpad: [0; SCRATCHPAD_WORDS],
                io: [0; IO_WORDS],
                expansion_2: [0; EXPANSION_2_WORDS],
                expansion_3: [0; EXPANSION_3_WORDS],
                bios: [0; BIOS_WORDS],
                io_cache: [0; IO_CACHE_WORDS],
            },
        }
    }

    pub fn exec_next_instr(&mut self) {
        self.cpu.fetch_decode_execute(&mut self.mem);
        println!("{:?}", self.cpu);
    }
}
