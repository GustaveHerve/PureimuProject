use crate::psx::{
    cpu::{
        CPU,
        instructions::{IType, JType, RType},
    },
    mem::MemBus,
};

pub enum BranchRt {
    BLTZ = 0b00000,
    BGEZ = 0b00001,
    BLTZAL = 0b10000,
    BGEZAL = 0b10001,
}

pub enum BranchOp {
    BCondZ,

    BEQ,
    BNE,
    BLEZ,
    BGTZ,
}

impl CPU {
    // TODO: handle jump delay
    // TODO: handle exceptions

    pub fn j(&mut self, bus: &MemBus, instr: JType) {
        self.core.pc = (self.core.pc & 0xf0000000) | (instr.imm() << 2);
    }

    pub fn jal(&mut self, bus: &MemBus, instr: JType) {
        self.core.set_gpr(31, self.core.pc + 8);
        self.core.pc = (self.core.pc & 0xf0000000) | (instr.imm() << 2);
    }

    pub fn jr(&mut self, bus: &MemBus, instr: RType) {
        let rs_idx = instr.rs() as usize;
        let rs_val = self.core.get_gpr(rs_idx);

        self.core.pc = rs_val;
    }

    pub fn jalr(&mut self, bus: &MemBus, instr: RType) {
        let rs_idx = instr.rs() as usize;
        let rs_val = self.core.get_gpr(rs_idx);
        let rd_idx = instr.rd() as usize;
        let rd_val = self.core.get_gpr(rd_idx);

        self.core.set_gpr(rd_idx, self.core.pc + 8);
        self.core.pc = rs_val;
    }

    pub fn branch(&mut self, bus: &MemBus, instr: IType, branch_op: BranchOp) {
        let rs_idx = instr.rs() as usize;
        let rs_val = self.core.get_gpr(rs_idx);
        let rt_idx = instr.rt() as usize;
        let rt_val = self.core.get_gpr(rt_idx);

        let cond = match branch_op {
            BranchOp::BCondZ => match instr.rt() {
                0b00000 => rs_val >> 31 == 1,
                0b00001 => rs_val >> 31 == 0,
                0b10000 => {
                    self.core.set_gpr(31, self.core.pc + 8);
                    rs_val >> 31 == 1
                }
                0b10001 => {
                    self.core.set_gpr(31, self.core.pc + 8);
                    rs_val >> 31 == 0
                }
                _ => panic!("Invalid BCondZ instruction"),
            },
            BranchOp::BEQ => rs_val == rt_val,
            BranchOp::BNE => rs_val != rt_val,
            BranchOp::BLEZ => rs_val >> 31 == 1 || rs_val == 0,
            BranchOp::BGTZ => rs_val >> 31 == 0 && rs_val != 0,
        };

        if cond {
            let imm_ex = instr.imm() as i16 as i32 as u32;
            let target = imm_ex << 2;
            self.core.pc = self.core.pc + target;
        }
    }

    // TODO: handle syscall/break instructions
}
