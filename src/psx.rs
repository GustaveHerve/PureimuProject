mod cpu;
mod mem;

pub struct Psx {
    cpu: cpu::Cpu,
    mem: mem::MemBus,
}

impl Psx {
    pub fn new() -> Psx {
        Psx {
            cpu: cpu::Cpu::new(),
            mem: mem::MemBus::new(),
        }
    }

    pub fn exec_next_instr(&mut self) {
        self.cpu.fetch_decode_execute(&mut self.mem);
        println!("{:?}", self.cpu);
    }
}
