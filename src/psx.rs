mod cpu;
mod mem;
mod opcodes;

pub struct PSX {
    cpu: cpu::CPU,
    mem: mem::MemBus,
}

impl PSX {
    pub fn new() -> PSX {
        PSX {
            cpu: cpu::CPU::new(),
            mem: mem::MemBus {},
        }
    }

    pub fn next_instruction(&self) {
        self.cpu.fetch_decode(&self.mem);
    }
}
