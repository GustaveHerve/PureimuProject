const MAIN_RAM_SIZE: usize = 2048 << 10;
const EXPANSION_1_SIZE: usize = 8192 << 10;
const SCRATCHPAD_SIZE: usize = 1 << 10;
const IO_SIZE: usize = 8 << 10;
const EXPANSION_2_SIZE: usize = 8 << 10;
const EXPANSION_3_SIZE: usize = 2048 << 10;
const BIOS_SIZE: usize = 512 << 10;
const IO_CACHE_SIZE: usize = 512;

const MEMSEG_MASK: u32 = 0xe000_0000;

pub enum MemSegment {
    Kuseg,
    Kseg0,
    Kseg1,
    Kseg2,
}

#[derive(Debug)]
pub enum MemException {
    UnauthorizedAccess,
    AddressError,
    BusError,
}

pub struct MemAddr {
    pub seg: MemSegment,
    pub phys_addr: u32,
}

pub struct MemBus {
    main_ram: [u8; MAIN_RAM_SIZE],
    scratchpad: [u8; SCRATCHPAD_SIZE],
    io: [u8; IO_SIZE],
}

impl From<u32> for MemAddr {
    fn from(value: u32) -> Self {
        MemAddr {
            seg: match (value & MEMSEG_MASK) >> 29 {
                0b000..=0b011 => MemSegment::Kuseg,
                0b100 => MemSegment::Kseg0,
                0b101 => MemSegment::Kseg1,
                0b110..=0b111 => MemSegment::Kseg2,
                _ => unreachable!("Invalid segment value in memory address"),
            },
            phys_addr: value & !MEMSEG_MASK,
        }
    }
}

impl MemBus {
    pub fn new() -> MemBus {
        MemBus {
            main_ram: [0; MAIN_RAM_SIZE],
            scratchpad: [0; SCRATCHPAD_SIZE],
            io: [0; IO_SIZE],
        }
    }

    // TODO: reading from memory (except Scratchpad) should have a 6 cycles delay
    pub fn read_u8(&self, addr: u32) -> Result<u8, MemException> {
        let addr = MemAddr::from(addr);
        let idx: usize = addr.phys_addr as usize;
        match addr.phys_addr {
            0x0000_0000..0x1f00_0000 => Ok(self.main_ram[idx]), // Main RAM
            0x1f00_0000..0x1f80_0000 => todo!(),                // Expansion Region 1
            0x1f80_0000..0x1f80_1000 => Ok(self.scratchpad[idx]), // Scratchpad
            0x1f80_1000..0x1f80_2000 => Ok(self.io[idx]),       // IO ports
            0x1f80_2000..0x1fa0_0000 => todo!(),                // Expansion Region 2
            0x1fa0_0000..0x1fc0_0000 => todo!(),                // Expansion Region 3
            0x1fc0_0000..0x1fc8_0000 => todo!(),                // BIOS ROM
            _ => Err(MemException::BusError),
        }
    }

    pub fn read_u16(&self, addr: u32) -> Result<u16, MemException> {
        // Address must be halfword aligned
        if addr & 0b1 != 0 {
            return Err(MemException::AddressError);
        }

        let bytes = [self.read_u8(addr)?, self.read_u8(addr + 1)?];

        Ok(u16::from_le_bytes(bytes))
    }

    pub fn read_u32(&self, addr: u32) -> Result<u32, MemException> {
        // Address must be word aligned
        if addr & 0b11 != 0 {
            return Err(MemException::AddressError);
        }

        let bytes = [
            self.read_u8(addr)?,
            self.read_u8(addr + 1)?,
            self.read_u8(addr + 2)?,
            self.read_u8(addr + 3)?,
        ];

        Ok(u32::from_le_bytes(bytes))
    }
}
