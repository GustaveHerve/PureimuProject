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
            mem: mem::MemBus {},
        }
    }

    pub fn exec_next_instr(&mut self) {
        self.cpu.fetch_decode_execute(&mut self.mem);
        println!("{:?}", self.cpu);
    }
}
