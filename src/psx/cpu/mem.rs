use super::CPU;

use crate::psx::{
    cpu::cop0::CpuMode,
    mem::{MemAddr, MemBus, MemException, MemSegment},
};

impl CPU {
    fn check_privilege(&self, addr: u32) -> Result<(), MemException> {
        if let CpuMode::UserMode = self.cop0.get_current_mode() {
            let mem_addr: MemAddr = addr.into();
            match mem_addr.seg {
                MemSegment::KUSEG => Err(MemException::UnauthorizedAccess),
                _ => Ok(()),
            }
        } else {
            Ok(())
        }
    }

    pub fn read_u8(&self, bus: &MemBus, addr: u32) -> Result<u8, MemException> {
        self.check_privilege(addr)?;
        let shift = (addr & 0b11) * 8;
        Ok((bus.read_bus(addr)? >> shift) as u8)
    }

    pub fn read_u16(&self, bus: &MemBus, addr: u32) -> Result<u16, MemException> {
        self.check_privilege(addr)?;
        // Address must be halfword aligned
        if addr & 1 != 0 {
            Err(MemException::AddressError)
        } else {
            let shift = (addr & 0b1) * 8;
            Ok((bus.read_bus(addr)? >> shift) as u16)
        }
    }

    pub fn read_u32(&self, bus: &MemBus, addr: u32) -> Result<u32, MemException> {
        self.check_privilege(addr)?;
        // Address must be word aligned
        if addr & 0b11 != 0 {
            Err(MemException::AddressError)
        } else {
            bus.read_bus(addr)
        }
    }

    pub fn write_u8(&mut self, bus: &mut MemBus, addr: u32, val: u8) {
        todo!()
    }

    pub fn write_u16(&mut self, bus: &mut MemBus, addr: u32, val: u16) {
        todo!()
    }

    pub fn write_u32(&mut self, bus: &mut MemBus, addr: u32, val: u32) {
        todo!()
    }
}
