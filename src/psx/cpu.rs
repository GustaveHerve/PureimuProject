mod cop0;
mod exceptions;
mod instructions;
mod mem;
mod timers;

use std::array::from_fn;
use std::collections::VecDeque;

use super::mem::MemBus;
use crate::psx::mem::{MemAddr, MemException, MemSegment};
use bitfield::bitfield;
use cop0::COP0;
use instructions::alu::AluImmOp;
use instructions::alu::{AluRegOp, HiLoOp, MulDivOp, ShiftOp};
use instructions::jump::BranchOp;
use instructions::load_store::{LoadOp, StoreOp};
use instructions::{IType, JType, RType};

const ICACHE_SIZE: usize = 4096;
const ICACHE_LINE_SIZE: usize = 16;
const ICACHE_LINE_LEN: usize = ICACHE_SIZE / ICACHE_LINE_SIZE;

#[derive(Debug)]
struct MulDivRegs {
    hi: u32,
    lo: u32,
}

#[derive(Debug)]
struct R3000 {
    gpr: [u32; 31],
    pc: u32,
    hilo: MulDivRegs,
}

impl R3000 {
    pub fn get_gpr(&self, idx: usize) -> u32 {
        if idx == 0 { 0 } else { self.gpr[idx - 1] }
    }

    pub fn set_gpr(&mut self, idx: usize, val: u32) {
        if idx != 0 {
            self.gpr[idx - 1] = val;
        }
    }

    pub fn get_hilo(&self) -> u64 {
        ((self.hilo.hi as u64) << 32) | (self.hilo.lo as u64)
    }

    pub fn set_hilo(&mut self, val: u64) {
        self.hilo.lo = val as u32;
        self.hilo.hi = (val >> 32) as u32;
    }
}

bitfield! {
    pub struct ICacheTag(u32);
    impl Debug;
    impl new;

    u8;
    pub(super) valid, set_valid : 3, 0;
    u32;
    pub(super) phys_addr, set_phys_addr : 31, 12;
}

#[derive(Debug, Clone, Copy)]
struct ICacheLine {
    tag: u32,
    word: [u32; 4],
}

#[derive(Debug)]
pub struct Cpu {
    core: R3000,
    cop0: COP0,
    icache: [ICacheLine; ICACHE_LINE_LEN],
    pending_loads: VecDeque<(usize, u32, bool)>,
    pending_branch: Option<(u32, u32)>,
    cycles: u64,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            core: R3000 {
                gpr: [0; 31],
                pc: 0,
                hilo: MulDivRegs { lo: 0, hi: 0 },
            },
            cop0: COP0 { regs: [0; 16] },
            icache: [ICacheLine {
                tag: 0,
                word: [0; 4],
            }; ICACHE_LINE_LEN],
            pending_loads: VecDeque::with_capacity(2),
            pending_branch: None,
            cycles: 0,
        }
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut res = Cpu::default();
        res.initialize();
        res
    }

    fn initialize(&mut self) {
        let mut sr = self.cop0.get_sr();
        sr.set_cu3(0);
        sr.set_cu2(1);
        sr.set_cu1(0);
        sr.set_cu0(1);
        sr.set_ts(1);
        self.cop0.set_sr(&sr);
    }

    fn lookup_icache(&self, addr: u32) -> Option<u32> {
        let line_idx: usize = (addr as usize >> 4) & 0xff; // Line index is address bits [11:4]
        let tag = ICacheTag(self.icache[line_idx].tag);
        if tag.phys_addr() == (addr & !0xfff) {
            let word_idx: usize = ((addr >> 2) & 0b11) as usize;
            // Check if accessed word is valid in cache
            if (tag.valid() & (0b1 << word_idx)) == 0 {
                // Accessed word is invalid, trigger a cache miss
                None
            } else {
                Some(self.icache[line_idx].word[word_idx])
            }
        } else {
            None
        }
    }

    fn handle_icache_miss(&mut self, bus: &MemBus, addr: u32) -> Result<u32, MemException> {
        let line_idx: usize = (addr as usize >> 4) & 0xff;
        let start_addr: u32 = addr & !0xf;
        // On cache miss CPU fills the accessed line from accessed word to end of cache line
        // TODO: IBLKSZ field of BIU register should limit maximum burst length
        let start_word_idx: usize = ((addr >> 2) & 0b11) as usize;
        for word_idx in start_word_idx..4 {
            self.icache[line_idx].word[word_idx] =
                self.read_u32_protected(bus, start_addr + (word_idx * 4) as u32)?
        }
        self.icache[line_idx].tag = ICacheTag::new(0xf << start_word_idx, addr & !0xfff).0;

        // Return the word stored at addr
        let word_idx: usize = ((addr & 0xf) / 4) as usize;
        Ok(self.icache[line_idx].word[word_idx])
    }

    fn delayed_load(&mut self, reg: usize, val: u32) {
        self.pending_loads.push_back((reg, val, true));
    }

    fn handle_pending_load(&mut self) {
        let e = self.pending_loads.pop_front();
        let Some(load) = e else {
            return;
        };
        if load.2 {
            // Pending load should be applied next call, put it back in queue
            self.pending_loads.push_front((load.0, load.1, false));
            return;
        }

        // Apply the pending load
        self.core.set_gpr(load.0, load.1);

        // Take into account delay of the potential next pending load
        let e = self.pending_loads.pop_front();
        let Some(load) = e else {
            return;
        };
        if load.2 {
            self.pending_loads.push_front((load.0, load.1, false));
        }
    }

    fn decode(&mut self, bus: &mut MemBus, raw_instr: u32, instr_pc: u32) {
        let i_instr = IType(raw_instr);
        match i_instr.op() {
            0x00 => self.decode_special(bus, RType(raw_instr), instr_pc),
            0x01 => self.branch(bus, i_instr, BranchOp::Bcondz),
            0x02 => self.j(bus, JType(raw_instr)),
            0x03 => self.jal(bus, JType(raw_instr)),
            0x04 => self.branch(bus, i_instr, BranchOp::Beq),
            0x05 => self.branch(bus, i_instr, BranchOp::Bne),
            0x06 => self.branch(bus, i_instr, BranchOp::Blez),
            0x07 => self.branch(bus, i_instr, BranchOp::Bgtz),
            0x08 => self.alu_imm(i_instr, instr_pc, AluImmOp::Addi),
            0x09 => self.alu_imm(i_instr, instr_pc, AluImmOp::Addiu),
            0x0a => self.alu_imm(i_instr, instr_pc, AluImmOp::Slti),
            0x0b => self.alu_imm(i_instr, instr_pc, AluImmOp::Sltiu),
            0x0c => self.alu_imm(i_instr, instr_pc, AluImmOp::Andi),
            0x0d => self.alu_imm(i_instr, instr_pc, AluImmOp::Ori),
            0x0e => self.alu_imm(i_instr, instr_pc, AluImmOp::Xori),
            0x0f => self.alu_imm(i_instr, instr_pc, AluImmOp::Lui),
            0x10 => self.decode_cop0(RType(raw_instr), instr_pc),
            0x12 => self.decode_cop2(RType(raw_instr)),
            0x11 | 0x13 => todo!("Unusable coprocessor exception not implemented yet"),
            0x20 => self.load(bus, i_instr, LoadOp::Lb),
            0x21 => self.load(bus, i_instr, LoadOp::Lh),
            0x22 => self.load(bus, i_instr, LoadOp::Lwl),
            0x23 => self.load(bus, i_instr, LoadOp::Lw),
            0x24 => self.load(bus, i_instr, LoadOp::Lbu),
            0x25 => self.load(bus, i_instr, LoadOp::Lhu),
            0x26 => self.load(bus, i_instr, LoadOp::Lwr),
            0x28 => self.store(bus, i_instr, StoreOp::Sb),
            0x29 => self.store(bus, i_instr, StoreOp::Sh),
            0x2a => self.store(bus, i_instr, StoreOp::Swl),
            0x2b => self.store(bus, i_instr, StoreOp::Sw),
            0x2e => self.store(bus, i_instr, StoreOp::Swr),
            0x30 => self.lwc0(bus, i_instr),
            x @ 0x31..=0x33 => todo!("LWC{} not implemented yet", x),
            0x38 => self.swc0(bus, i_instr),
            x @ 0x39..=0x3b => todo!("SWC{} not implemented yet", x),
            _ => todo!("Reserved Instruction Exception handling not implemented yet"),
        }
    }

    fn decode_special(&mut self, bus: &mut MemBus, instr: RType, instr_pc: u32) {
        match instr.funct() {
            0x00 => self.shift(instr, ShiftOp::Sll),
            0x02 => self.shift(instr, ShiftOp::Srl),
            0x03 => self.shift(instr, ShiftOp::Sra),
            0x04 => self.shift(instr, ShiftOp::Sllv),
            0x06 => self.shift(instr, ShiftOp::Srlv),
            0x07 => self.shift(instr, ShiftOp::Srav),
            0x08 => self.jr(bus, instr),
            0x09 => self.jalr(bus, instr),
            0x0c => todo!("SYSCALL not implemented yet"),
            0x0d => todo!("BREAK not implemented yet"),
            0x10 => self.move_hilo(instr, HiLoOp::Mfhi),
            0x11 => self.move_hilo(instr, HiLoOp::Mthi),
            0x12 => self.move_hilo(instr, HiLoOp::Mflo),
            0x13 => self.move_hilo(instr, HiLoOp::Mtlo),
            0x18 => self.muldiv(instr, MulDivOp::Mult),
            0x19 => self.muldiv(instr, MulDivOp::Multu),
            0x1a => self.muldiv(instr, MulDivOp::Div),
            0x1b => self.muldiv(instr, MulDivOp::Divu),
            0x20 => self.alu_reg(instr, instr_pc, AluRegOp::Add),
            0x21 => self.alu_reg(instr, instr_pc, AluRegOp::Addu),
            0x22 => self.alu_reg(instr, instr_pc, AluRegOp::Sub),
            0x23 => self.alu_reg(instr, instr_pc, AluRegOp::Subu),
            0x24 => self.alu_reg(instr, instr_pc, AluRegOp::And),
            0x25 => self.alu_reg(instr, instr_pc, AluRegOp::Or),
            0x26 => self.alu_reg(instr, instr_pc, AluRegOp::Xor),
            0x27 => self.alu_reg(instr, instr_pc, AluRegOp::Nor),
            0x2a => self.alu_reg(instr, instr_pc, AluRegOp::Slt),
            0x2b => self.alu_reg(instr, instr_pc, AluRegOp::Sltu),
            _ => todo!("Reserved Instruction Exception handling not implemented yet"),
        }
    }

    pub fn fetch_decode_execute(&mut self, bus: &mut MemBus) {
        let mem_addr = MemAddr::from(self.core.pc);
        let instr: u32 = match mem_addr.seg {
            MemSegment::Kuseg | MemSegment::Kseg0 => {
                self.lookup_icache(self.core.pc).unwrap_or_else(|| {
                    self.handle_icache_miss(bus, self.core.pc)
                        .expect("PC attemped to read an unauthorized memory address ({})")
                })
            }
            MemSegment::Kseg1 => self.read_u32_protected(bus, self.core.pc).unwrap(),
            MemSegment::Kseg2 => unreachable!("CPU attempted to execute KSEG2 memory !"),
        };

        let prev_pc: u32 = self.core.pc;
        self.core.pc = self.core.pc.wrapping_add(4);
        self.decode(bus, instr, prev_pc);

        self.handle_pending_load();
    }
}
