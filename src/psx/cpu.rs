mod opcodes;

use crate::psx::mem::MemBus;

#[derive(Debug)]
pub struct R3000 {
    gpr: [u32; 32],
    pc: u32,
    sp: u32,
    hi: u32,
    lo: u32,
}

#[derive(Debug)]
pub struct COP0 {}

pub struct CPU {
    core: R3000,
    cop0: COP0,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            core: R3000 {
                gpr: [0; 32],
                pc: 0,
                sp: 0,
                hi: 0,
                lo: 0,
            },
            cop0: COP0 {},
        }
    }

    pub fn fetch_decode(&self, membus: &MemBus) {
        membus.read_mem(0);
    }
}
