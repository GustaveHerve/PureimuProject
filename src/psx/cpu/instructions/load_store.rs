use crate::psx::{
    cpu::{Cpu, instructions::IType},
    mem::{MemBus, MemException},
};

pub enum LoadOp {
    Lb,
    Lbu,
    Lh,
    Lhu,
    Lw,
    Lwl,
    Lwr,
}

pub enum StoreOp {
    Sb,
    Sh,
    Sw,
    Swl,
    Swr,
}

impl Cpu {
    fn lwl(&self, bus: &MemBus, addr: u32, val: u32) -> Result<u32, MemException> {
        let mut res = val.to_le_bytes();
        let aligned_addr = addr & !3;
        let word_arr = self.read_u32(bus, aligned_addr)?.to_le_bytes();

        let offset: usize = addr as usize % 4;
        let len: usize = offset + 1;
        res[(4 - len)..4].copy_from_slice(&word_arr[0..len]);

        Ok(u32::from_le_bytes(res))
    }

    fn lwr(&self, bus: &MemBus, addr: u32, val: u32) -> Result<u32, MemException> {
        let mut res = val.to_le_bytes();
        let aligned_addr = addr & !3;
        let word_arr = self.read_u32(bus, aligned_addr)?.to_le_bytes();

        let offset: usize = addr as usize % 4;
        let len: usize = 4 - offset;
        res[0..len].copy_from_slice(&word_arr[offset..4]);

        Ok(u32::from_le_bytes(res))
    }

    fn swl(&self, bus: &MemBus, addr: u32, val: u32) -> Result<u32, MemException> {
        let aligned_addr = addr & !3;
        let mut res = self.read_u32(bus, aligned_addr)?.to_le_bytes();
        let word_arr = val.to_le_bytes();

        let offset: usize = val as usize % 4;
        let len: usize = offset + 1;
        res[(4 - len)..4].copy_from_slice(&word_arr[0..len]);

        Ok(u32::from_le_bytes(res))
    }

    fn swr(&self, bus: &MemBus, addr: u32, val: u32) -> Result<u32, MemException> {
        let aligned_addr = addr & !3;
        let mut res = self.read_u32(bus, aligned_addr)?.to_le_bytes();
        let word_arr = val.to_le_bytes();

        let offset: usize = addr as usize % 4;
        let len: usize = 4 - offset;
        res[0..len].copy_from_slice(&word_arr[offset..4]);

        Ok(u32::from_le_bytes(res))
    }
    pub fn load(&mut self, bus: &MemBus, instr: IType, load_op: LoadOp) {
        let rs_idx: usize = instr.rs() as usize;
        let rs_val = self.core.get_gpr(rs_idx);

        let addr = rs_val.wrapping_add_signed(instr.imm().cast_signed() as i32);
        let res = match load_op {
            LoadOp::Lb => self.read_u8(bus, addr).unwrap().cast_signed() as i32 as u32,
            LoadOp::Lbu => self.read_u8(bus, addr).unwrap() as u32,
            LoadOp::Lh => self.read_u16(bus, addr).unwrap().cast_signed() as i32 as u32,
            LoadOp::Lhu => self.read_u16(bus, addr).unwrap() as u32,
            LoadOp::Lw => self.read_u32(bus, addr).unwrap(),
            LoadOp::Lwl => self.lwl(bus, addr, rs_val).unwrap(),
            LoadOp::Lwr => self.lwr(bus, addr, rs_val).unwrap(),
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
            StoreOp::Sb => self.write_u8(bus, addr, rt_val as u8),
            StoreOp::Sh => self.write_u16(bus, addr, rt_val as u16),
            StoreOp::Sw => self.write_u32(bus, addr, rt_val),
            StoreOp::Swl => self.write_u32(bus, addr, self.swl(bus, addr, rt_val).unwrap()),
            StoreOp::Swr => self.write_u32(bus, addr, self.swr(bus, addr, rt_val).unwrap()),
        };
    }
}
