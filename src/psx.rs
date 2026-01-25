use crate::psx::mem::{
    BIOS_SIZE, EXPANSION_1_SIZE, EXPANSION_2_SIZE, EXPANSION_3_SIZE, IO_CACHE_SIZE, IO_SIZE,
    MAIN_RAM_SIZE, SCRATCHPAD_SIZE,
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
                main_ram: [0; MAIN_RAM_SIZE],
                expansion_1: [0; EXPANSION_1_SIZE],
                scratchpad: [0; SCRATCHPAD_SIZE],
                io: [0; IO_SIZE],
                expansion_2: [0; EXPANSION_2_SIZE],
                expansion_3: [0; EXPANSION_3_SIZE],
                bios: [0; BIOS_SIZE],
                io_cache: [0; IO_CACHE_SIZE],
            },
        }
    }

    pub fn exec_next_instr(&mut self) {
        self.cpu.fetch_decode_execute(&mut self.mem);
        println!("{:?}", self.cpu);
    }
}
