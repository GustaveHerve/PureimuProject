mod instructions;

use instructions::{IType, Instr, JType, RType};

use crate::psx::{
    cpu::instructions::alu::{AluImmOp, AluRegFunct, ShiftFunct},
    mem::MemBus,
};

use instructions::load_store::{LoadOp, StoreOp};

const ICACHE_SIZE: usize = 4096;

#[derive(Debug)]
struct R3000 {
    gpr: [u32; 31],
    pc: u32,
    sp: u32,
    hi: u32,
    lo: u32,
}

impl R3000 {
    pub fn get_reg(&self, idx: usize) -> u32 {
        assert!(idx <= 32);
        if idx == 0 { 0 } else { self.gpr[idx - 1] }
    }

    pub fn set_reg(&mut self, idx: usize, val: u32) {
        assert!(idx <= 32);
        if idx != 0 {
            self.gpr[idx - 1] = val;
        }
    }
}

#[derive(Debug)]
pub struct COP0 {}

#[derive(Debug)]
pub struct CPU {
    core: R3000,
    cop0: COP0,
    icache: [u32; ICACHE_SIZE],
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            core: R3000 {
                gpr: [0; 31],
                pc: 0,
                sp: 0,
                hi: 0,
                lo: 0,
            },
            cop0: COP0 {},
            icache: [0; ICACHE_SIZE],
        }
    }

    fn dispatch_itype(&mut self, bus: &MemBus, itype: IType) -> Result<(), ()> {
        Ok(())
    }

    fn dispatch_jtype(&mut self, bus: &MemBus, jtype: JType) -> Result<(), ()> {
        Ok(())
    }

    fn dispatch_rtype(&mut self, bus: &MemBus, rtype: RType) -> Result<(), ()> {
        if let Ok(e) = AluRegFunct::try_from(rtype.funct()) {
        } else if let Ok(e) = ShiftFunct::try_from(rtype.funct()) {
            todo!()
        } else if let Ok(e) = LoadOp::try_from(rtype.op()) {
            todo!()
        } else if let Ok(e) = StoreOp::try_from(rtype.op()) {
            todo!()
        } else {
            return Err(());
        }

        Ok(())
    }

    pub fn fetch_decode(&mut self, bus: &MemBus) {
        // TODO: Fetch instruction
        let raw_instr: u32 = bus.read_u32(0);

        let instr: Instr = raw_instr.into();

        let Ok(_) = (match instr {
            Instr::IType(itype) => self.dispatch_itype(bus, itype),
            Instr::JType(jtype) => self.dispatch_jtype(bus, jtype),
            Instr::RType(rtype) => self.dispatch_rtype(bus, rtype),
        }) else {
            panic!("Invalid instruction (PC=0x{:x})", self.core.pc);
        };
    }
}
