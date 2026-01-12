pub mod alu;
pub mod copro;
pub mod jmp;
pub mod load_store;
pub mod special;

#[derive(Clone, Copy)]
pub struct IType {
    pub op: u8,
    pub rs: u8,
    pub rt: u8,
    pub imm: u16,
}

#[derive(Clone, Copy)]
pub struct JType {
    op: u8,
    imm: u32,
}

#[derive(Clone, Copy)]
pub struct RType {
    op: u8,
    rs: u8,
    rt: u8,
    shamt: u8,
    funct: u8,
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
