mod instructions;

use instructions::alu::{AluImmOp, AluRegFunct, ShiftFunct};
use instructions::load_store::{LoadOp, StoreOp};
use instructions::{IType, Instr, JType, RType};

use crate::psx::mem::MemBus;

const ICACHE_SIZE: usize = 4096;
const ICACHE_LINE_LEN: usize = 16;

const ICACHE_LEN_LINE: usize = ICACHE_SIZE / ICACHE_LINE_LEN;

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
struct COP0 {}

#[derive(Debug, Clone, Copy)]
struct ICacheLineEntry {
    tag: bool,
    word: [u32; 4],
}

#[derive(Debug)]
pub struct CPU {
    core: R3000,
    cop0: COP0,
    icache: [ICacheLineEntry; ICACHE_LEN_LINE],
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
            icache: [ICacheLineEntry {
                tag: false,
                word: [0; 4],
            }; ICACHE_LEN_LINE],
        }
    }

    fn lookup_icache(&self, addr: u32) -> Option<u32> {
        let line_idx: usize = (addr as usize >> 4) & 0xff;
        if self.icache[line_idx].tag {
            Some(self.icache[line_idx].word[((addr >> 2) & 0b11) as usize])
        } else {
            None
        }
    }

    fn load_icache_line(&mut self, bus: &MemBus, addr: u32) {
        let line_idx: usize = (addr as usize >> 4) & 0xff;
        let start_addr: u32 = addr & !0xf;
        for idx in 0..4 {
            self.icache[line_idx].word[idx] = bus.read_u32(start_addr + (idx * 4) as u32);
        }
        self.icache[line_idx].tag = true;
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
        let instr: Instr = if let Some(raw_instr) = self.lookup_icache(self.core.pc) {
            raw_instr
        } else {
            bus.read_u32(self.core.pc)
        }
        .into();
        let Ok(_) = (match instr {
            Instr::IType(itype) => self.dispatch_itype(bus, itype),
            Instr::JType(jtype) => self.dispatch_jtype(bus, jtype),
            Instr::RType(rtype) => self.dispatch_rtype(bus, rtype),
        }) else {
            panic!("Invalid instruction (PC={:#x})", self.core.pc);
        };
    }
}
