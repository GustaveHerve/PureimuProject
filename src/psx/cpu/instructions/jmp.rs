use crate::{
    define_repr_enum,
    psx::{cpu::CPU, mem::MemBus},
};

define_repr_enum! {
    pub enum JmpImmOp : u8 {
        J = 0b000010,
        JAL = 0b000011,
    }
}

define_repr_enum! {
    pub enum JmpRegFunct : u8 {
        JR = 0b001000,
        JALR = 0b001001,
    }
}

define_repr_enum! {
    pub enum BranchRt : u8 {
        BLTZ = 0b00000,
        BGEZ = 0b00001,
        BLTZAL = 0b10000,
        BGEZAL = 0b10001,
    }
}

define_repr_enum! {
    pub enum BranchOp : u8 {
        BEQ = 0b000100,
        BNE = 0b000101,
        BLEZ = 0b000110,
        BGTZ = 0b000111,
    }
}

define_repr_enum! {
    pub enum Branch : u8 {
        BLTZ = 0b00000,
        BGEZ = 0b00001,
        BLTZAL = 0b10000,
        BGEZAL = 0b10001,

        BEQ = 0b000100,
        BNE = 0b000101,
        BLEZ = 0b000110,
        BGTZ = 0b000111,
    }
}

impl CPU {
    // TODO: handle jump delay
    // TODO: handle exceptions
    pub fn jmp_imm(&mut self, bus: &MemBus, jmp_op: JmpImmOp, rs: u8, imm: u32) {
        if let JmpImmOp::JAL = jmp_op {
            self.core.set_gpr(31, self.core.pc + 8);
        }

        self.core.pc = (self.core.pc & 0xf0000000) | (imm << 2);
    }

    pub fn jmp_reg(&mut self, bus: &MemBus, jmp_op: JmpRegFunct, rs: u8, rd: u8) {
        let rs_idx = rs as usize;
        let rs_val = self.core.get_gpr(rs_idx);
        let rd_idx = rd as usize;
        let rd_val = self.core.get_gpr(rd_idx);

        if let JmpRegFunct::JALR = jmp_op {
            self.core.set_gpr(rd_idx, self.core.pc + 8);
        }

        self.core.pc = rs_val;
    }

    pub fn branch(&mut self, bus: &MemBus, branch_op: Branch, rs: u8, rt: u8, imm: u16) {
        let rs_idx = rs as usize;
        let rs_val = self.core.get_gpr(rs_idx);
        let rt_idx = rt as usize;
        let rt_val = self.core.get_gpr(rt_idx);

        let cond = match branch_op {
            Branch::BLTZ | Branch::BLTZAL => rs_val >> 31 == 1,
            Branch::BGEZ | Branch::BGEZAL => rs_val >> 31 == 0,
            Branch::BEQ => rs_val == rt_val,
            Branch::BNE => rs_val != rt_val,
            Branch::BLEZ => rs_val >> 31 == 1 || rs_val == 0,
            Branch::BGTZ => rs_val >> 31 == 0 && rs_val != 0,
        };

        match branch_op {
            Branch::BLTZAL | Branch::BGEZAL => self.core.set_gpr(31, self.core.pc + 8),
            _ => (),
        }

        if cond {
            let imm_ex = imm as i16 as i32 as u32;
            let target = imm_ex << 2;
            self.core.pc = self.core.pc + target;
        }
    }

    // TODO: handle syscall/break instructions
}
