mod instructions;

use instructions::alu::{AluImmOp, AluRegFunct, ShiftFunct};
use instructions::load_store::{LoadOp, StoreOp};
use instructions::{IType, Instr, JType, RType};

use crate::psx::mem::MemBus;

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
    pub fn get_gpr(&self, idx: usize) -> u32 {
        assert!(idx <= 32);
        if idx == 0 { 0 } else { self.gpr[idx - 1] }
    }

    pub fn set_gpr(&mut self, idx: usize, val: u32) {
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

    fn dispatch_itype(&mut self, bus: &mut MemBus, itype: IType) -> Result<(), ()> {
        if let Ok(alu_op) = AluImmOp::try_from(itype.op()) {
            self.alu_imm(bus, alu_op, itype.rs(), itype.rt(), itype.imm());
        } else if let Ok(load_op) = LoadOp::try_from(itype.op()) {
            self.load(bus, load_op, itype.rs(), itype.rt(), itype.imm());
        } else if let Ok(store_op) = StoreOp::try_from(itype.op()) {
            self.store(bus, store_op, itype.rs(), itype.rt(), itype.imm());
        } else {
            return Err(());
        }

        Ok(())
    }

    fn dispatch_jtype(&mut self, bus: &MemBus, jtype: JType) -> Result<(), ()> {
        todo!()
    }

    fn dispatch_rtype(&mut self, bus: &MemBus, rtype: RType) -> Result<(), ()> {
        if let Ok(alu_op) = AluRegFunct::try_from(rtype.funct()) {
            self.alu_reg(bus, alu_op, rtype.rs(), rtype.rt(), rtype.rd());
        } else if let Ok(shift_op) = ShiftFunct::try_from(rtype.funct()) {
            self.shift(
                bus,
                shift_op,
                rtype.rs(),
                rtype.rt(),
                rtype.rd(),
                rtype.shamt(),
            );
        } else {
            return Err(());
        }

        Ok(())
    }

    pub fn fetch_decode_execute(&mut self, bus: &mut MemBus) {
        let instr: Instr = bus.read_u32(self.core.pc).into();
        let Ok(_) = (match instr {
            Instr::IType(itype) => self.dispatch_itype(bus, itype),
            Instr::JType(jtype) => self.dispatch_jtype(bus, jtype),
            Instr::RType(rtype) => self.dispatch_rtype(bus, rtype),
        }) else {
            panic!("Invalid instruction (PC=0x{:x})", self.core.pc);
        };
    }
}
