pub mod alu;
pub mod copro;
pub mod jmp;
pub mod load_store;
pub mod special;

use bitfield::bitfield;

bitfield! {
    pub struct IType(u32);
    impl Debug;
    impl new;

    u16;
    pub(super) imm, set_imm : 15, 0;
    u8;
    pub(super) rt, set_rt : 20, 16;
    pub(super) rs, set_rs : 25, 21;
    pub(super) op, set_op : 31, 26;
}

bitfield! {
    pub struct JType(u32);
    impl Debug;
    impl new;

    u32;
    pub(super) imm, set_imm : 25, 0;
    u8;
    pub(super) op, set_op : 31, 26;
}

bitfield! {
    pub struct RType(u32);
    impl Debug;
    impl new;

    u8;
    pub(super) funct, set_funct : 5, 0;
    pub(super) shamt, set_shamt : 10, 6;
    pub(super) rd, set_rd : 15, 11;
    pub(super) rt, set_rt : 20, 16;
    pub(super) rs, set_rs : 25, 21;
    pub(super) op, set_op : 31, 26;
}

pub enum Instr {
    IType(IType),
    JType(JType),
    RType(RType),
}

impl Instr {
    pub fn as_itype(&self) -> Option<&IType> {
        if let Instr::IType(res) = self {
            Some(res)
        } else {
            None
        }
    }

    pub fn as_jtype(&self) -> Option<&JType> {
        if let Instr::JType(res) = self {
            Some(res)
        } else {
            None
        }
    }

    pub fn as_rtype(&self) -> Option<&RType> {
        if let Instr::RType(res) = self {
            Some(res)
        } else {
            None
        }
    }
}

impl From<u32> for Instr {
    fn from(value: u32) -> Self {
        match (value >> 26) & 0b11 {
            0b00 => Instr::RType(RType(value)),
            0b10 | 0b11 => Instr::JType(JType(value)),
            _ => Instr::IType(IType(value)),
        }
    }
}
