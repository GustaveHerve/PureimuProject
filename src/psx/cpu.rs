mod cop0;
mod exceptions;
mod instructions;
mod mem;
mod timers;

use instructions::alu::AluImmOp;
use instructions::{IType, JType, RType};

use super::mem::MemBus;
use instructions::alu::{AluRegOp, HiLoOp, MulDivOp, ShiftOp};
use instructions::jmp::BranchOp;
use instructions::load_store::{LoadOp, StoreOp};

use cop0::COP0;

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
        assert!(idx <= 32);
        if idx == 0 { 0 } else { self.gpr[idx - 1] }
    }

    pub fn set_gpr(&mut self, idx: usize, val: u32) {
        assert!(idx <= 32);
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

#[derive(Debug, Clone, Copy)]
struct ICacheLine {
    tag: u32,
    word: [u32; 4],
}

#[derive(Debug)]
pub struct CPU {
    core: R3000,
    cop0: COP0,
    icache: [ICacheLine; ICACHE_LINE_LEN],
}

impl CPU {
    pub fn new() -> CPU {
        let mut res = CPU {
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
        };
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
        let line_idx: usize = (addr as usize / ICACHE_LINE_SIZE) & 0xff;
        if self.icache[line_idx].tag == (addr & !0xf) {
            Some(self.icache[line_idx].word[((addr >> 2) & 0b11) as usize])
        } else {
            None
        }
    }

    fn load_icache_line(&mut self, bus: &MemBus, addr: u32) -> u32 {
        let line_idx: usize = (addr as usize >> 4) & 0xff;
        let start_addr: u32 = addr & !0xf;
        for idx in 0..4 {
            self.icache[line_idx].word[idx] =
                self.read_u32(bus, start_addr + (idx * 4) as u32).unwrap();
        }
        self.icache[line_idx].tag = start_addr;

        // Return the word stored at addr
        let word_idx: usize = (addr & 0xf / 4) as usize;
        self.icache[line_idx].word[word_idx]
    }

    fn decode(&mut self, bus: &mut MemBus, raw_instr: u32, instr_pc: u32) {
        let i_instr = IType(raw_instr);
        match i_instr.op() {
            0x00 => self.decode_special(bus, RType(raw_instr), instr_pc),
            0x01 => self.branch(bus, i_instr, BranchOp::BCondZ),
            0x02 => self.j(bus, JType(raw_instr)),
            0x03 => self.jal(bus, JType(raw_instr)),
            0x04 => self.branch(bus, i_instr, BranchOp::BEQ),
            0x05 => self.branch(bus, i_instr, BranchOp::BNE),
            0x06 => self.branch(bus, i_instr, BranchOp::BLEZ),
            0x07 => self.branch(bus, i_instr, BranchOp::BGTZ),
            0x08 => self.alu_imm(i_instr, instr_pc, AluImmOp::ADDI),
            0x09 => self.alu_imm(i_instr, instr_pc, AluImmOp::ADDIU),
            0x0a => self.alu_imm(i_instr, instr_pc, AluImmOp::SLTI),
            0x0b => self.alu_imm(i_instr, instr_pc, AluImmOp::SLTIU),
            0x0c => self.alu_imm(i_instr, instr_pc, AluImmOp::ANDI),
            0x0d => self.alu_imm(i_instr, instr_pc, AluImmOp::ORI),
            0x0e => self.alu_imm(i_instr, instr_pc, AluImmOp::XORI),
            0x0f => self.alu_imm(i_instr, instr_pc, AluImmOp::LUI),
            0x10 => self.decode_cop0(RType(raw_instr), instr_pc),
            0x12 => self.decode_cop2(RType(raw_instr)),
            0x11 | 0x13 => todo!("Unusable coprocessor exception not implemented yet"),
            0x20 => self.load(bus, i_instr, LoadOp::LB),
            0x21 => self.load(bus, i_instr, LoadOp::LH),
            0x22 => self.load(bus, i_instr, LoadOp::LWL),
            0x23 => self.load(bus, i_instr, LoadOp::LW),
            0x24 => self.load(bus, i_instr, LoadOp::LBU),
            0x25 => self.load(bus, i_instr, LoadOp::LHU),
            0x26 => self.load(bus, i_instr, LoadOp::LWR),
            0x28 => self.store(bus, i_instr, StoreOp::SB),
            0x29 => self.store(bus, i_instr, StoreOp::SH),
            0x2a => self.store(bus, i_instr, StoreOp::SWL),
            0x2b => self.store(bus, i_instr, StoreOp::SW),
            0x2e => self.store(bus, i_instr, StoreOp::SWR),
            0x30 => self.lwc0(bus, i_instr),
            x @ 0x31..=0x33 => todo!("LWC{} not implemented yet", x),
            0x38 => self.swc0(bus, i_instr),
            x @ 0x39..=0x3b => todo!("SWC{} not implemented yet", x),
            _ => todo!("Reserved Instruction Exception handling not implemented yet"),
        }
    }

    fn decode_special(&mut self, bus: &mut MemBus, instr: RType, instr_pc: u32) {
        match instr.funct() {
            0x00 => self.shift(instr, ShiftOp::SLL),
            0x02 => self.shift(instr, ShiftOp::SRL),
            0x03 => self.shift(instr, ShiftOp::SRA),
            0x04 => self.shift(instr, ShiftOp::SLLV),
            0x06 => self.shift(instr, ShiftOp::SRLV),
            0x07 => self.shift(instr, ShiftOp::SRAV),
            0x08 => self.jr(bus, instr),
            0x09 => self.jalr(bus, instr),
            0x0c => todo!("SYSCALL not implemented yet"),
            0x0d => todo!("BREAK not implemented yet"),
            0x10 => self.move_hilo(instr, HiLoOp::MFHI),
            0x11 => self.move_hilo(instr, HiLoOp::MTHI),
            0x12 => self.move_hilo(instr, HiLoOp::MFLO),
            0x13 => self.move_hilo(instr, HiLoOp::MTLO),
            0x18 => self.muldiv(instr, MulDivOp::MULT),
            0x19 => self.muldiv(instr, MulDivOp::MULTU),
            0x1a => self.muldiv(instr, MulDivOp::DIV),
            0x1b => self.muldiv(instr, MulDivOp::DIVU),
            0x20 => self.alu_reg(instr, instr_pc, AluRegOp::ADD),
            0x21 => self.alu_reg(instr, instr_pc, AluRegOp::ADDU),
            0x22 => self.alu_reg(instr, instr_pc, AluRegOp::SUB),
            0x23 => self.alu_reg(instr, instr_pc, AluRegOp::SUBU),
            0x24 => self.alu_reg(instr, instr_pc, AluRegOp::AND),
            0x25 => self.alu_reg(instr, instr_pc, AluRegOp::OR),
            0x26 => self.alu_reg(instr, instr_pc, AluRegOp::XOR),
            0x27 => self.alu_reg(instr, instr_pc, AluRegOp::NOR),
            0x2a => self.alu_reg(instr, instr_pc, AluRegOp::SLT),
            0x2b => self.alu_reg(instr, instr_pc, AluRegOp::SLTU),
            _ => todo!("Reserved Instruction Exception handling not implemented yet"),
        }
    }

    pub fn fetch_decode_execute(&mut self, bus: &mut MemBus) {
        let instr: u32 = self
            .lookup_icache(self.core.pc)
            .unwrap_or_else(|| self.load_icache_line(bus, self.core.pc));
        let prev_pc: u32 = self.core.pc;
        self.core.pc += 4;
        self.decode(bus, instr, prev_pc);
    }
}
