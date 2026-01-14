use crate::{
    define_repr_enum,
    psx::{cpu::CPU, mem::MemBus},
};

define_repr_enum! {
    pub enum LoadOp : u8 {
        LB  = 0b100000,
        LBU = 0b100100,
        LH  = 0b100001,
        LHU = 0b100101,
        LW  = 0b100011,
        LWL = 0b100010,
        LWR = 0b100110,
    }
}

define_repr_enum! {
    pub enum StoreOp : u8 {
        SB = 0b101000,
        SH = 0b101001,
        SW = 0b101011,
        SWL = 0b101010,
        SWR = 0b101110,
    }
}

fn lwl(bus: &MemBus, addr: u32, val: u32) -> u32 {
    let mut res = val.to_le_bytes();
    let word_arr = bus.read_u32(addr).to_le_bytes();
    let start_byte: usize = addr as usize % 4;
    let byte_count: usize = 4 - start_byte;
    for i in 0..byte_count {
        res[i] = word_arr[start_byte + i];
    }
    u32::from_le_bytes(res)
}

fn lwr(bus: &MemBus, addr: u32, val: u32) -> u32 {
    let mut res = val.to_le_bytes();
    let word_arr = bus.read_u32(addr).to_le_bytes();
    let start_byte: usize = addr as usize % 4;
    let byte_count: usize = start_byte + 1;
    for i in 0..byte_count {
        res[res.len() - i] = word_arr[start_byte - i];
    }
    u32::from_le_bytes(res)
}

fn swl(bus: &MemBus, addr: u32, val: u32) -> u32 {
    let mut res = bus.read_u32(addr).to_le_bytes();
    let word_arr = val.to_le_bytes();
    let start_byte: usize = val as usize % 4;
    let byte_count: usize = 4 - start_byte;
    for i in 0..byte_count {
        res[i] = word_arr[start_byte + i];
    }
    u32::from_le_bytes(res)
}

fn swr(bus: &MemBus, addr: u32, val: u32) -> u32 {
    let mut res = bus.read_u32(addr).to_le_bytes();
    let word_arr = val.to_le_bytes();
    let start_byte: usize = val as usize % 4;
    let byte_count: usize = start_byte + 1;
    for i in 0..byte_count {
        res[i] = word_arr[start_byte + i];
    }
    u32::from_le_bytes(res)
}

impl CPU {
    pub fn load(&mut self, bus: &MemBus, load_op: LoadOp, rs: u8, rt: u8, imm: u16) {
        let rs_idx: usize = rs as usize;
        let rs_val = self.core.get_reg(rs_idx);

        let addr = rs_val.wrapping_add_signed(imm.cast_signed() as i32);
        let res = match load_op {
            LoadOp::LB => bus.read_u8(addr).cast_signed() as i32 as u32,
            LoadOp::LBU => bus.read_u8(addr) as u32,
            LoadOp::LH => bus.read_u16(addr).cast_signed() as i32 as u32,
            LoadOp::LHU => bus.read_u16(addr) as u32,
            LoadOp::LW => bus.read_u32(addr),
            LoadOp::LWL => lwl(bus, addr, self.core.get_reg(rs_idx)),
            LoadOp::LWR => lwr(bus, addr, self.core.get_reg(rs_idx)),
        };
        self.core.set_reg(rt as usize, res);
    }

    pub fn store(&self, bus: &mut MemBus, store_op: StoreOp, rs: u8, rt: u8, imm: u16) {
        let rs_idx: usize = rs as usize;
        let rs_val = self.core.get_reg(rs_idx);
        let rt_idx: usize = rt as usize;
        let rt_val = self.core.get_reg(rt_idx);

        let addr = rs_val.wrapping_add_signed(imm.cast_signed() as i32);
        match store_op {
            StoreOp::SB => bus.write_u8(addr, rt_val as u8),
            StoreOp::SH => bus.write_u16(addr, rt_val as u16),
            StoreOp::SW => bus.write_u32(addr, rt_val),
            StoreOp::SWL => bus.write_u32(addr, swl(bus, addr, rt_val)),
            StoreOp::SWR => bus.write_u32(addr, swr(bus, addr, rt_val)),
        };
    }
}
