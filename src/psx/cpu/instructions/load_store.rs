use crate::psx::{
    cpu::{CPU, instructions::IType},
    mem::{MemBus, MemException},
};

pub enum LoadOp {
    LB,
    LBU,
    LH,
    LHU,
    LW,
    LWL,
    LWR,
}

pub enum StoreOp {
    SB,
    SH,
    SW,
    SWL,
    SWR,
}

impl CPU {
    fn lwl(&self, bus: &MemBus, addr: u32, val: u32) -> Result<u32, MemException> {
        let mut res = val.to_le_bytes();
        let word_arr = self.read_u32(bus, addr).unwrap().to_le_bytes();
        let start_byte: usize = addr as usize % 4;
        let byte_count: usize = 4 - start_byte;
        for i in 0..byte_count {
            res[i] = word_arr[start_byte + i];
        }
        Ok(u32::from_le_bytes(res))
    }

    fn lwr(&self, bus: &MemBus, addr: u32, val: u32) -> Result<u32, MemException> {
        let mut res = val.to_le_bytes();
        let word_arr = self.read_u32(bus, addr).unwrap().to_le_bytes();
        let start_byte: usize = addr as usize % 4;
        let byte_count: usize = start_byte + 1;
        for i in 0..byte_count {
            res[res.len() - i] = word_arr[start_byte - i];
        }
        Ok(u32::from_le_bytes(res))
    }

    fn swl(&self, bus: &MemBus, addr: u32, val: u32) -> Result<u32, MemException> {
        let mut res = self.read_u32(bus, addr).unwrap().to_le_bytes();
        let word_arr = val.to_le_bytes();
        let start_byte: usize = val as usize % 4;
        let byte_count: usize = 4 - start_byte;
        for i in 0..byte_count {
            res[i] = word_arr[start_byte + i];
        }
        Ok(u32::from_le_bytes(res))
    }

    fn swr(&self, bus: &MemBus, addr: u32, val: u32) -> Result<u32, MemException> {
        let mut res = self.read_u32(bus, addr).unwrap().to_le_bytes();
        let word_arr = val.to_le_bytes();
        let start_byte: usize = val as usize % 4;
        let byte_count: usize = start_byte + 1;
        for i in 0..byte_count {
            res[i] = word_arr[start_byte + i];
        }
        Ok(u32::from_le_bytes(res))
    }
    pub fn load(&mut self, bus: &MemBus, instr: IType, load_op: LoadOp) {
        let rs_idx: usize = instr.rs() as usize;
        let rs_val = self.core.get_gpr(rs_idx);

        let addr = rs_val.wrapping_add_signed(instr.imm().cast_signed() as i32);
        let res = match load_op {
            LoadOp::LB => self.read_u8(bus, addr).unwrap().cast_signed() as i32 as u32,
            LoadOp::LBU => self.read_u8(bus, addr).unwrap() as u32,
            LoadOp::LH => self.read_u16(bus, addr).unwrap().cast_signed() as i32 as u32,
            LoadOp::LHU => self.read_u16(bus, addr).unwrap() as u32,
            LoadOp::LW => self.read_u32(bus, addr).unwrap(),
            LoadOp::LWL => self.lwl(bus, addr, rs_val).unwrap(),
            LoadOp::LWR => self.lwr(bus, addr, rs_val).unwrap(),
        };
        self.core.set_gpr(instr.rt() as usize, res);
    }

    pub fn store(&mut self, bus: &mut MemBus, instr: IType, store_op: StoreOp) {
        let rs_idx: usize = instr.rs() as usize;
        let rs_val = self.core.get_gpr(rs_idx);
        let rt_idx: usize = instr.rt() as usize;
        let rt_val = self.core.get_gpr(rt_idx);

        let addr = rs_val.wrapping_add_signed(instr.imm().cast_signed() as i32);
        match store_op {
            StoreOp::SB => self.write_u8(bus, addr, rt_val as u8),
            StoreOp::SH => self.write_u16(bus, addr, rt_val as u16),
            StoreOp::SW => self.write_u32(bus, addr, rt_val),
            StoreOp::SWL => self.write_u32(bus, addr, self.swl(bus, addr, rt_val).unwrap()),
            StoreOp::SWR => self.write_u32(bus, addr, self.swr(bus, addr, rt_val).unwrap()),
        };
    }
}
