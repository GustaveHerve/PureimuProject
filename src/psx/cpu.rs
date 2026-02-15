mod exceptions;
mod instructions;
mod timers;

use instructions::alu::AluImmOp;
use instructions::{IType, JType, RType};

use crate::psx::cpu::instructions::alu::{AluRegOp, HiLoOp, MulDivOp};
use crate::psx::cpu::instructions::jmp::BranchOp;
use crate::psx::cpu::instructions::load_store::{LoadOp, StoreOp};
use crate::psx::mem::MemBus;

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
    sp: u32,
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

#[derive(Debug)]
struct COP0 {
    pub regs: [u32; 16],
}

impl COP0 {
    pub fn get_reg(&self, idx: usize) -> u32 {
        assert!(idx < 64);
        self.regs[idx]
    }
    pub fn set_reg(&mut self, idx: usize, value: u32) {
        assert!(idx < 64);
        self.regs[idx] = value;
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
        CPU {
            core: R3000 {
                gpr: [0; 31],
                pc: 0,
                sp: 0,
                hilo: MulDivRegs { lo: 0, hi: 0 },
            },
            cop0: COP0 { regs: [0; 16] },
            icache: [ICacheLine {
                tag: 0,
                word: [0; 4],
            }; ICACHE_LINE_LEN],
        }
    }

    fn lookup_icache(&self, addr: u32) -> Option<u32> {
        let line_idx: usize = (addr as usize >> 4) & 0xff;
        if self.icache[line_idx].tag == (addr & !0xf) {
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
        self.icache[line_idx].tag = start_addr;
    }

    fn decode(&mut self, bus: &mut MemBus, raw_instr: u32) {
        let i_instr = IType(raw_instr);
        match i_instr.op() {
            0x00 => self.decode_special(bus, RType(raw_instr)),
            0x01 => self.branch(bus, i_instr, BranchOp::BCondZ),
            0x02 => self.j(bus, JType(raw_instr)),
            0x03 => self.jal(bus, JType(raw_instr)),
            0x04 => self.branch(bus, i_instr, BranchOp::BEQ),
            0x05 => self.branch(bus, i_instr, BranchOp::BNE),
            0x06 => self.branch(bus, i_instr, BranchOp::BLEZ),
            0x07 => self.branch(bus, i_instr, BranchOp::BGTZ),
            0x08 => self.alu_imm(bus, i_instr, AluImmOp::ADDI),
            0x09 => self.alu_imm(bus, i_instr, AluImmOp::ADDIU),
            0x0a => self.alu_imm(bus, i_instr, AluImmOp::SLTI),
            0x0b => self.alu_imm(bus, i_instr, AluImmOp::SLTIU),
            0x0c => self.alu_imm(bus, i_instr, AluImmOp::ANDI),
            0x0d => self.alu_imm(bus, i_instr, AluImmOp::ORI),
            0x0e => self.alu_imm(bus, i_instr, AluImmOp::XORI),
            0x0f => self.alu_imm(bus, i_instr, AluImmOp::LUI),
            0x10..=0x13 => todo!("COPn not implemented yet"),
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

    fn decode_special(&mut self, bus: &mut MemBus, instr: RType) {
        match instr.funct() {
            0x00 => todo!("SLL not implemented yet"),
            0x02 => todo!("SRL not implemented yet"),
            0x08 => self.jr(bus, instr),
            0x09 => self.jalr(bus, instr),
            0x0c => todo!("SYSCALL not implemented yet"),
            0x0d => todo!("BREAK not implemented yet"),
            0x10 => self.move_hilo(bus, instr, HiLoOp::MFHI),
            0x11 => self.move_hilo(bus, instr, HiLoOp::MTHI),
            0x12 => self.move_hilo(bus, instr, HiLoOp::MFLO),
            0x13 => self.move_hilo(bus, instr, HiLoOp::MTLO),
            0x18 => self.muldiv(bus, instr, MulDivOp::MULT),
            0x19 => self.muldiv(bus, instr, MulDivOp::MULTU),
            0x1a => self.muldiv(bus, instr, MulDivOp::DIV),
            0x1b => self.muldiv(bus, instr, MulDivOp::DIVU),
            0x20 => self.alu_reg(bus, instr, AluRegOp::ADD),
            0x21 => self.alu_reg(bus, instr, AluRegOp::ADDU),
            0x22 => self.alu_reg(bus, instr, AluRegOp::SUB),
            0x23 => self.alu_reg(bus, instr, AluRegOp::SUBU),
            0x24 => self.alu_reg(bus, instr, AluRegOp::AND),
            0x25 => self.alu_reg(bus, instr, AluRegOp::OR),
            0x26 => self.alu_reg(bus, instr, AluRegOp::XOR),
            0x27 => self.alu_reg(bus, instr, AluRegOp::NOR),
            0x2a => self.alu_reg(bus, instr, AluRegOp::SLT),
            0x2b => self.alu_reg(bus, instr, AluRegOp::SLTU),
            _ => todo!("Reserved Instruction Exception handling not implemented yet"),
        }
    }

    pub fn fetch_decode_execute(&mut self, bus: &mut MemBus) {
        let instr: u32 = self
            .lookup_icache(self.core.pc)
            .unwrap_or_else(|| bus.read_u32(self.core.pc));

        self.decode(bus, instr);
        // TODO: PC increment should be done right after fetching
        self.core.pc += 1;
    }
}
