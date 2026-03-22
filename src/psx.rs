mod cpu;
mod mem;

use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq)]
struct Event {
    tick_ts: usize,
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.tick_ts.cmp(&self.tick_ts)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

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

    pub fn execution_loop(&mut self) {
        let mut heap: BinaryHeap<Event> = BinaryHeap::new();
        heap.push(Event { tick_ts: 1 });
        heap.push(Event { tick_ts: 5 });
        heap.push(Event { tick_ts: 4 });
        let test = heap.pop();
        println!("{:#?}", test);
        let test = heap.pop();
        println!("{:#?}", test);
        let test = heap.pop();
        println!("{:#?}", test);
        for _ in 0..50 {
            self.exec_next_instr();
        }
    }
}
