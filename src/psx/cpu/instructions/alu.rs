use crate::{
    define_repr_enum,
    psx::{cpu::CPU, mem::MemBus},
};

define_repr_enum! {
    pub enum AluRegFunct : u8 {
        ADD = 0b100000,
        ADDU = 0b100001,
        SUB = 0b100010,
        SUBU = 0b100011,
        SLT = 0b101010,
        SLTU = 0b101011,
        AND = 0b100100,
        OR = 0b100101,
        XOR = 0b100110,
        NOR = 0b100111,
    }
}

define_repr_enum! {
    pub enum AluImmOp : u8 {
        ADDI = 0b001000,
        ADDIU = 0b001001,
        SLTI = 0b001010,
        SLTIU = 0b001011,
        ANDI = 0b001100,
        ORI = 0b001101,
        XORI = 0b001110,
        LUI = 0b001111
    }
}

define_repr_enum! {
    pub enum ShiftFunct : u8 {
        SLL = 0b000000,
        SRL = 0b000010,
        SRA = 0b000011,
        SLLV = 0b000100,
        SRLV = 0b000110,
        SRAV = 0b000111,
    }
}

impl CPU {
    pub fn alu_reg(&mut self, bus: &MemBus, alu_op: AluRegFunct, rs: u8, rt: u8, rd: u8) {
        let rs_idx = rs as usize;
        let rs_val = self.core.get_gpr(rs_idx);
        let rt_idx = rt as usize;
        let rt_val = self.core.get_gpr(rt_idx);

        let res = match alu_op {
            AluRegFunct::ADD | AluRegFunct::ADDU => rs_val.wrapping_add(rt_val),
            AluRegFunct::SUB | AluRegFunct::SUBU => rs_val.wrapping_sub(rt_val),
            AluRegFunct::SLT => (rs_val.cast_signed() < rt_val.cast_signed()) as u32,
            AluRegFunct::SLTU => (rs_val < rt_val) as u32,
            AluRegFunct::AND => rs_val & rt_val,
            AluRegFunct::OR => rs_val | rt_val,
            AluRegFunct::XOR => rs_val ^ rt_val,
            AluRegFunct::NOR => u32::MAX ^ (rs_val | rt_val),
        };
        // TODO: handle overflow traps for ADDU and SUBU

        let rd_idx = rd as usize;
        self.core.set_gpr(rd_idx, res);
    }

    pub fn alu_imm(&mut self, bus: &MemBus, alu_op: AluImmOp, rs: u8, rt: u8, imm: u16) {
        let rs_idx = rs as usize;
        let rs_val = self.core.get_gpr(rs_idx);
        let imm_ex = imm as i16 as i32 as u32;

        let res = match alu_op {
            AluImmOp::ADDI | AluImmOp::ADDIU => rs_val.wrapping_add(imm_ex),
            AluImmOp::SLTI => (rs_val.cast_signed() < imm_ex.cast_signed()) as u32,
            AluImmOp::SLTIU => (rs_val < imm_ex) as u32,
            AluImmOp::ANDI => rs_val & (imm as u32),
            AluImmOp::ORI => rs_val | (imm as u32),
            AluImmOp::XORI => rs_val ^ (imm as u32),
            AluImmOp::LUI => (imm as u32) << 16,
        };
        // TODO: handle overflow traps for ADDIU and SUBIU

        let rt_idx = rt as usize;
        self.core.set_gpr(rt_idx, res);
    }

    pub fn shift(&mut self, bus: &MemBus, shift_op: ShiftFunct, rs: u8, rt: u8, rd: u8, shamt: u8) {
        let rs_idx = rs as usize;
        let rs_val = self.core.get_gpr(rs_idx);
        let rt_idx = rt as usize;
        let rt_val = self.core.get_gpr(rt_idx);

        let res = match shift_op {
            ShiftFunct::SLL => rt_val << shamt,
            ShiftFunct::SRL => rt_val >> shamt,
            ShiftFunct::SRA => (rt_val.cast_signed() >> shamt) as u32,
            ShiftFunct::SLLV => rt_val << (rs_val & 0x1f),
            ShiftFunct::SRLV => rt_val >> (rs_val & 0x1f),
            ShiftFunct::SRAV => (rt_val.cast_signed() >> (rs_val & 0x1f)) as u32,
        };

        let rd_idx = rd as usize;
        self.core.set_gpr(rd_idx, res);
    }

    pub fn muldiv(&mut self, bus: &MemBus, instr: &super::RType) {
        todo!()
    }
}
