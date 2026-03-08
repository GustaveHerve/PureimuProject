use crate::psx::cpu::{
    CPU,
    exceptions::ExceptionType,
    instructions::{IType, RType},
};

pub enum AluRegOp {
    ADD,
    ADDU,
    SUB,
    SUBU,
    SLT,
    SLTU,
    AND,
    OR,
    XOR,
    NOR,
}

pub enum AluImmOp {
    ADDI,
    ADDIU,
    SLTI,
    SLTIU,
    ANDI,
    ORI,
    XORI,
    LUI,
}

pub enum ShiftOp {
    SLL,
    SRL,
    SRA,
    SLLV,
    SRLV,
    SRAV,
}

pub enum MulDivOp {
    MULT,
    MULTU,
    DIV,
    DIVU,
}

pub enum HiLoOp {
    MFHI,
    MTHI,
    MFLO,
    MTLO,
}

impl CPU {
    pub fn alu_reg(&mut self, instr: RType, instr_pc: u32, alu_op: AluRegOp) {
        let rs_idx = instr.rs() as usize;
        let rs_val = self.core.get_gpr(rs_idx);
        let rt_idx = instr.rt() as usize;
        let rt_val = self.core.get_gpr(rt_idx);

        let res: u32 = match alu_op {
            AluRegOp::ADD | AluRegOp::ADDU => rs_val.wrapping_add(rt_val),
            AluRegOp::SUB | AluRegOp::SUBU => rs_val.wrapping_sub(rt_val),
            AluRegOp::SLT => (rs_val.cast_signed() < rt_val.cast_signed()) as u32,
            AluRegOp::SLTU => (rs_val < rt_val) as u32,
            AluRegOp::AND => rs_val & rt_val,
            AluRegOp::OR => rs_val | rt_val,
            AluRegOp::XOR => rs_val ^ rt_val,
            AluRegOp::NOR => u32::MAX ^ (rs_val | rt_val),
        };

        // Overflow exception check
        match alu_op {
            AluRegOp::ADD => {
                if (rs_val >> 31 == rt_val >> 31) && (res >> 31 != rs_val >> 31) {
                    self.throw_exception(instr_pc, ExceptionType::Overflow);
                    return;
                }
            }
            AluRegOp::SUB => {
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
            AluImmOp::ADDI | AluImmOp::ADDIU => rs_val.wrapping_add(imm_ex),
            AluImmOp::SLTI => (rs_val.cast_signed() < imm_ex.cast_signed()) as u32,
            AluImmOp::SLTIU => (rs_val < imm_ex) as u32,
            AluImmOp::ANDI => rs_val & (instr.imm() as u32),
            AluImmOp::ORI => rs_val | (instr.imm() as u32),
            AluImmOp::XORI => rs_val ^ (instr.imm() as u32),
            AluImmOp::LUI => (instr.imm() as u32) << 16,
        };

        if let AluImmOp::ADDI = alu_op {
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
            ShiftOp::SLL => rt_val << instr.shamt(),
            ShiftOp::SRL => rt_val >> instr.shamt(),
            ShiftOp::SRA => (rt_val.cast_signed() >> instr.shamt()) as u32,
            ShiftOp::SLLV => rt_val << (rs_val & 0x1f),
            ShiftOp::SRLV => rt_val >> (rs_val & 0x1f),
            ShiftOp::SRAV => (rt_val.cast_signed() >> (rs_val & 0x1f)) as u32,
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
            MulDivOp::MULT => (rs_val.cast_signed() as i64 * rt_val.cast_signed() as i64) as u64,
            MulDivOp::MULTU => (rs_val * rt_val) as u64,
            MulDivOp::DIV => {
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
            MulDivOp::DIVU => {
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
            HiLoOp::MFHI => self.core.set_gpr(rd_idx, self.core.hilo.hi),
            HiLoOp::MTHI => self.core.hilo.hi = rs_val,
            HiLoOp::MFLO => self.core.set_gpr(rd_idx, self.core.hilo.lo),
            HiLoOp::MTLO => self.core.hilo.lo = rs_val,
        }
    }
}
