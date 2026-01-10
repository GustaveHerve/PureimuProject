pub mod alu;
pub mod copro;
pub mod jmp;
pub mod load_store;
pub mod special;

#[allow(dead_code)]
pub enum Opcode {
    // Load
    LB { rs: u8, rt: u8, imm16: u16 },
    LBU { rs: u8, rt: u8, imm16: u16 },
    LH { rs: u8, rt: u8, imm16: u16 },
    LHU { rs: u8, rt: u8, imm16: u16 },
    LW { rs: u8, rt: u8, imm16: u16 },
    // Store
    SB { rs: u8, rt: u8, imm16: u16 },
    SH { rs: u8, rt: u8, imm16: u16 },
    SW { rs: u8, rt: u8, imm16: u16 },
}
