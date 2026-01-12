mod opcodes;

use crate::psx::mem::MemBus;

use opcodes::load_store::{LoadOp, StoreOp};

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

    pub fn fetch_decode(&mut self, bus: &MemBus) {
        let instr: u32 = bus.read_u32(0);
        let op = opcodes::IType {
            op: LoadOp::LB as u8,
            rs: 1,
            rt: 2,
            imm: 3,
        };
        match instr {
            _ => self.load(bus, &op),
        }
    }
}
