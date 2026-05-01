use super::Cpu;

use crate::psx::{
    cpu::cop0::CpuMode,
    mem::{MemAddr, MemBus, MemException, MemSegment},
};

impl Cpu {
    /// Checks if the accessed memory can be done in the current privilege level (User/Kernel mode).
    ///
    /// * `addr`: Memory address to check privlege for.
    fn check_privilege(&self, addr: u32) -> Result<(), MemException> {
        if let CpuMode::UserMode = self.cop0.get_current_mode() {
            let mem_addr: MemAddr = addr.into();
            match mem_addr.seg {
                MemSegment::Kuseg => Ok(()),
                _ => Err(MemException::UnauthorizedAccess),
            }
        } else {
            // Kernel mode can access every memory segments
            Ok(())
        }
    }

    /// Reads a byte at address addr from memory bus while checking for privilege level.
    /// Returns Ok(u8) in case of success.
    /// Returns Err(UnauthorizedAccess) in case of wrong privilege level.
    ///
    /// * `bus`: Reference to the MemBus.
    /// * `addr`: Memory address to read.
    pub fn read_u8_protected(&self, bus: &MemBus, addr: u32) -> Result<u8, MemException> {
        self.check_privilege(addr)?;
        bus.read_u8(addr)
    }

    /// Reads a half-word at address addr from memory bus while checking for privilege level.
    /// Returns Ok(u16) in case of success.
    /// Returns Err(UnauthorizedAccess) in case of wrong privilege level.
    ///
    /// * `bus`: Reference to the MemBus.
    /// * `addr`: Memory address to read.
    pub fn read_u16_protected(&self, bus: &MemBus, addr: u32) -> Result<u16, MemException> {
        self.check_privilege(addr)?;
        bus.read_u16(addr)
    }

    /// Reads a word at address addr from memory bus while checking for privilege level.
    /// Returns Ok(u32) in case of success.
    /// Returns Err(UnauthorizedAccess) in case of wrong privilege level.
    ///
    /// * `bus`: Reference to the MemBus.
    /// * `addr`: Memory address to read.
    pub fn read_u32_protected(&self, bus: &MemBus, addr: u32) -> Result<u32, MemException> {
        self.check_privilege(addr)?;
        bus.read_u32(addr)
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
