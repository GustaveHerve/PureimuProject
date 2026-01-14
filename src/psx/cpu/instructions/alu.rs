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
    }
}

define_repr_enum! {
    pub enum ShiftFunct : u8 {
        SLLV = 0b000100,
        SRLV = 0b000110,
        SRAV = 0b000111,
        SLL = 0b000000,
        SRL = 0b000010,
        SRA = 0b000011,
    }
}

impl CPU {
    pub fn alu_reg(bus: &MemBus, instr: &super::RType) {
        let alu_op = match AluRegFunct::try_from(instr.op()) {
            Ok(op) => op,
            _ => panic!("Invalid load opcode"),
        };

        if let Ok(e) = AluRegFunct::try_from(instr.funct()) {}
        if let Ok(e) = AluImmOp::try_from(instr.op()) {}
        if let Ok(e) = AluRegFunct::try_from(instr.funct()) {}

        panic!("Invalid ALU opcode")
    }

    pub fn alu_imm(bus: &MemBus, instr: &super::IType) {
        todo!()
    }

    pub fn shift(bus: &MemBus, instr: &super::RType) {
        todo!()
    }

    pub fn lui(bus: &MemBus, instr: &super::IType) {
        todo!()
    }

    pub fn muldiv(bus: &MemBus, instr: &super::RType) {
        todo!()
    }
}
