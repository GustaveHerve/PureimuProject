use crate::psx::cpu::CPU;
use crate::psx::mem::MemBus;

#[repr(u8)]
pub enum LoadOp {
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

#[repr(u8)]
pub enum StoreOp {
    SB = 0b000,
    SH = 0b001,
    SW = 0b011,
    SWL = 0b010,
    SWR = 0b110,
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
            _ => Err("Invalid load opcode"),
        }
    }
}

impl TryFrom<u8> for StoreOp {
    type Error = &'static str;

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            0b000 => Ok(StoreOp::SB),
            0b001 => Ok(StoreOp::SH),
            0b011 => Ok(StoreOp::SW),
            0b010 => Ok(StoreOp::SWL),
            0b110 => Ok(StoreOp::SWR),
            _ => Err("Invalid store opcode"),
        }
    }
}

fn lwl(bus: &MemBus, addr: u32, reg: u32) -> u32 {
    let mut res = reg.to_le_bytes();
    let word_arr = bus.read_u32(addr).to_le_bytes();
    let start_byte: usize = addr as usize % 4;
    let byte_count: usize = 4 - start_byte;
    for i in 0..byte_count {
        res[i] = word_arr[start_byte + i];
    }
    u32::from_le_bytes(res)
}

fn lwr(bus: &MemBus, addr: u32, reg: u32) -> u32 {
    let mut res = reg.to_le_bytes();
    let word_arr = bus.read_u32(addr).to_le_bytes();
    let start_byte: usize = addr as usize % 4;
    let byte_count: usize = start_byte + 1;
    for i in 0..byte_count {
        res[res.len() - i] = word_arr[start_byte - i];
    }
    u32::from_le_bytes(res)
}

impl CPU {
    pub fn load(&mut self, bus: &MemBus, instr: &super::IType) {
        let load_op = match LoadOp::try_from(instr.op) {
            Ok(op) => op,
            _ => panic!("Invalid load opcode"),
        };
        let rs_idx: usize = instr.rs as usize;
        let rs_val = self.core.gpr[rs_idx];
        let addr = rs_val.wrapping_add_signed(instr.imm.cast_signed() as i32);
        self.core.gpr[instr.rt as usize] = match load_op {
            LoadOp::LB => bus.read_u8(addr).cast_signed() as i32 as u32,
            LoadOp::LBU => bus.read_u8(addr) as u32,
            LoadOp::LH => bus.read_u16(addr).cast_signed() as i32 as u32,
            LoadOp::LHU => bus.read_u16(addr) as u32,
            LoadOp::LW => bus.read_u32(addr),
            LoadOp::LWL => lwl(bus, addr, self.core.gpr[rs_idx]),
            LoadOp::LWR => lwr(bus, addr, self.core.gpr[rs_idx]),
        };
    }

    pub fn store(&self, bus: &mut MemBus, instr: &super::IType) {
        let store_op = match StoreOp::try_from(instr.op) {
            Ok(op) => op,
            _ => panic!("Invalid store opcode"),
        };
        let rs_idx: usize = instr.rs as usize;
        let rs_val = self.core.gpr[rs_idx];
        let addr = rs_val.wrapping_add_signed(instr.imm.cast_signed() as i32);
        let val = self.core.gpr[rs_idx];
        match store_op {
            StoreOp::SB => bus.write_u8(addr, val as u8),
            StoreOp::SH => bus.write_u16(addr, val as u16),
            StoreOp::SW => bus.write_u32(addr, val),
            StoreOp::SWL => todo!(),
            StoreOp::SWR => todo!(),
        };
    }
}
