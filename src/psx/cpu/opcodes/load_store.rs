use crate::psx::cpu::CPU;
use crate::psx::mem::MemBus;

#[repr(u8)]
enum LoadOp {
    // Aligned load
    LB = 0b000,
    LBU = 0b100,
    LH = 0b001,
    LHU = 0b101,
    LW = 0b011,
    // Unaligned load
    LWL = 0b010,
    LWR = 0b110,
}

impl TryFrom<u8> for LoadOp {
    type Error = &'static str;

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            0b000 => Ok(LoadOp::LB),
            0b100 => Ok(LoadOp::LBU),
            0b001 => Ok(LoadOp::LH),
            0b101 => Ok(LoadOp::LHU),
            0b011 => Ok(LoadOp::LW),
            _ => Err("Invalid Load opcode"),
        }
    }
}

enum StoreOp {
    SB,
    SH,
    SW,
}

impl CPU {
    pub fn load(&mut self, bus: &MemBus, instr: &super::IType) {
        let load_op = match LoadOp::try_from(instr.op) {
            Ok(op) => op,
            _ => panic!("Invalid load opcode"),
        };
        let rs_val = self.core.gpr[instr.rs as usize];
        let addr = rs_val.wrapping_add_signed(instr.imm.cast_signed() as i32);
        let val = bus.read_mem(addr);
        self.core.gpr[instr.rt as usize] = match load_op {
            LoadOp::LB => (val & 0xFF) as i8 as i32 as u32,
            LoadOp::LBU => val & 0xFF,
            LoadOp::LH => (val & 0xFFFF) as i16 as i32 as u32,
            LoadOp::LHU => val & 0xFFFF,
            LoadOp::LW => val,
            LoadOp::LWL => todo!(),
            LoadOp::LWR => todo!(),
        };
        self.core.gpr[instr.rt as usize] = val;
    }
}
