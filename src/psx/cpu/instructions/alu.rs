use crate::psx::cpu::{
    Cpu,
    exceptions::ExceptionType,
    instructions::{IType, RType},
};

pub enum AluRegOp {
    Add,
    Addu,
    Sub,
    Subu,
    Slt,
    Sltu,
    And,
    Or,
    Xor,
    Nor,
}

pub enum AluImmOp {
    Addi,
    Addiu,
    Slti,
    Sltiu,
    Andi,
    Ori,
    Xori,
    Lui,
}

pub enum ShiftOp {
    Sll,
    Srl,
    Sra,
    Sllv,
    Srlv,
    Srav,
}

pub enum MulDivOp {
    Mult,
    Multu,
    Div,
    Divu,
}

pub enum HiLoOp {
    Mfhi,
    Mthi,
    Mflo,
    Mtlo,
}

impl Cpu {
    pub fn alu_reg(&mut self, instr: RType, instr_pc: u32, alu_op: AluRegOp) {
        let rs_idx = instr.rs() as usize;
        let rs_val = self.core.get_gpr(rs_idx);
        let rt_idx = instr.rt() as usize;
        let rt_val = self.core.get_gpr(rt_idx);

        let res: u32 = match alu_op {
            AluRegOp::Add | AluRegOp::Addu => rs_val.wrapping_add(rt_val),
            AluRegOp::Sub | AluRegOp::Subu => rs_val.wrapping_sub(rt_val),
            AluRegOp::Slt => (rs_val.cast_signed() < rt_val.cast_signed()) as u32,
            AluRegOp::Sltu => (rs_val < rt_val) as u32,
            AluRegOp::And => rs_val & rt_val,
            AluRegOp::Or => rs_val | rt_val,
            AluRegOp::Xor => rs_val ^ rt_val,
            AluRegOp::Nor => u32::MAX ^ (rs_val | rt_val),
        };

        // Overflow exception check
        match alu_op {
            AluRegOp::Add => {
                if (rs_val >> 31 == rt_val >> 31) && (res >> 31 != rs_val >> 31) {
                    self.throw_exception(instr_pc, ExceptionType::Overflow);
                    return;
                }
            }
            AluRegOp::Sub => {
                if (rs_val >> 31 != rt_val >> 31) && (res >> 31 != rs_val >> 31) {
                    self.throw_exception(instr_pc, ExceptionType::Overflow);
                    return;
                }
            }
            _ => (),
        }

        let rd_idx = instr.rd() as usize;
        self.core.set_gpr(rd_idx, res);
    }

    pub fn alu_imm(&mut self, instr: IType, instr_pc: u32, alu_op: AluImmOp) {
        let rs_idx = instr.rs() as usize;
        let rs_val = self.core.get_gpr(rs_idx);
        let imm_ex = instr.imm() as i16 as i32 as u32;

        let res: u32 = match alu_op {
            AluImmOp::Addi | AluImmOp::Addiu => rs_val.wrapping_add(imm_ex),
            AluImmOp::Slti => (rs_val.cast_signed() < imm_ex.cast_signed()) as u32,
            AluImmOp::Sltiu => (rs_val < imm_ex) as u32,
            AluImmOp::Andi => rs_val & (instr.imm() as u32),
            AluImmOp::Ori => rs_val | (instr.imm() as u32),
            AluImmOp::Xori => rs_val ^ (instr.imm() as u32),
            AluImmOp::Lui => (instr.imm() as u32) << 16,
        };

        if let AluImmOp::Addi = alu_op {
            if (rs_val >> 31 == imm_ex >> 31) && (res >> 31 != rs_val >> 31) {
                self.throw_exception(instr_pc, ExceptionType::Overflow);
                return;
            }
        }

        let rt_idx = instr.rt() as usize;
        self.core.set_gpr(rt_idx, res);
    }

    pub fn shift(&mut self, instr: RType, shift_op: ShiftOp) {
        let rs_idx = instr.rs() as usize;
        let rs_val = self.core.get_gpr(rs_idx);
        let rt_idx = instr.rt() as usize;
        let rt_val = self.core.get_gpr(rt_idx);

        let res: u32 = match shift_op {
            ShiftOp::Sll => rt_val << instr.shamt(),
            ShiftOp::Srl => rt_val >> instr.shamt(),
            ShiftOp::Sra => (rt_val.cast_signed() >> instr.shamt()) as u32,
            ShiftOp::Sllv => rt_val << (rs_val & 0x1f),
            ShiftOp::Srlv => rt_val >> (rs_val & 0x1f),
            ShiftOp::Srav => (rt_val.cast_signed() >> (rs_val & 0x1f)) as u32,
        };

        let rd_idx = instr.rd() as usize;
        self.core.set_gpr(rd_idx, res);
    }

    pub fn muldiv(&mut self, instr: RType, muldiv_op: MulDivOp) {
        let rs_idx = instr.rs() as usize;
        let rs_val = self.core.get_gpr(rs_idx);
        let rt_idx = instr.rt() as usize;
        let rt_val = self.core.get_gpr(rt_idx);

        // TODO: handle muldiv delay
        let res: u64 = match muldiv_op {
            MulDivOp::Mult => (rs_val.cast_signed() as i64 * rt_val.cast_signed() as i64) as u64,
            MulDivOp::Multu => (rs_val * rt_val) as u64,
            MulDivOp::Div => {
                if rt_val == 0 {
                    (rs_val as u64) << 32 | u32::MAX as u64
                } else {
                    match rt_val {
                        0 => {
                            if (rs_val as i32) >= 0 {
                                (rs_val as u64) << 32 | u32::MAX as u64
                            } else {
                                (rs_val as u64) << 32 | 1
                            }
                        }
                        u32::MAX if rs_val == 0x8000_0000 => 0x8000_0000 as u64,
                        _ => (rs_val.cast_signed() / rt_val.cast_signed()) as u64,
                    }
                }
            }
            MulDivOp::Divu => {
                if rt_val == 0 {
                    (rs_val as u64) << 32 | u32::MAX as u64
                } else {
                    (rs_val / rt_val) as u64
                }
            }
        };

        self.core.hilo.lo = res as u32;
        self.core.hilo.hi = (res >> 32) as u32;
    }

    pub fn move_hilo(&mut self, instr: RType, hilo_op: HiLoOp) {
        let rs_idx = instr.rs() as usize;
        let rs_val = self.core.get_gpr(rs_idx);
        let rd_idx = instr.rd() as usize;

        match hilo_op {
            HiLoOp::Mfhi => self.core.set_gpr(rd_idx, self.core.hilo.hi),
            HiLoOp::Mthi => self.core.hilo.hi = rs_val,
            HiLoOp::Mflo => self.core.set_gpr(rd_idx, self.core.hilo.lo),
            HiLoOp::Mtlo => self.core.hilo.lo = rs_val,
        }
    }
}
