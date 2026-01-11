use crate::psx::cpu::CPU;
use crate::psx::mem::MemBus;

enum Opcode {
    LB { rs: u8, rt: u8, imm: u16 },
    LBU { rs: u8, rt: u8, imm: u16 },
    LH { rs: u8, rt: u8, imm: u16 },
    LHU { rs: u8, rt: u8, imm: u16 },
    LW { rs: u8, rt: u8, imm: u16 },
}

impl CPU {
    pub fn load(&self, bus: MemBus) {}
}
