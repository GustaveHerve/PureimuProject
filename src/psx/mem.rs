mod io;

const MEMSEG_MASK: u32 = 0xe000_0000;

pub(super) const MAIN_RAM_SIZE: usize = 2048 << 10;
pub(super) const EXPANSION_1_SIZE: usize = 8192 << 10;
pub(super) const SCRATCHPAD_SIZE: usize = 1 << 10;
pub(super) const IO_SIZE: usize = 8 << 10;
pub(super) const EXPANSION_2_SIZE: usize = 8 << 10;
pub(super) const EXPANSION_3_SIZE: usize = 2048 << 10;
pub(super) const BIOS_SIZE: usize = 512 << 10;
pub(super) const IO_CACHE_SIZE: usize = 512;

enum MemSegment {
    KUSEG,
    KSEG0,
    KSEG1,
    KSEG2,
}

struct MemAddr {
    pub seg: MemSegment,
    pub offset: u32,
}

pub struct MemBus {
    pub main_ram: [u8; MAIN_RAM_SIZE],
    pub expansion_1: [u8; EXPANSION_1_SIZE],
    pub scratchpad: [u8; SCRATCHPAD_SIZE],
    pub io: [u8; IO_SIZE],
    pub expansion_2: [u8; EXPANSION_2_SIZE],
    pub expansion_3: [u8; EXPANSION_3_SIZE],
    pub bios: [u8; BIOS_SIZE],
    pub io_cache: [u8; IO_CACHE_SIZE],
}

impl From<u32> for MemAddr {
    fn from(value: u32) -> Self {
        MemAddr {
            seg: match (value & MEMSEG_MASK) >> 29 {
                0b000..=0b011 => MemSegment::KUSEG,
                0b100 => MemSegment::KSEG0,
                0b101 => MemSegment::KSEG1,
                0b110..=0b111 => MemSegment::KSEG2,
                _ => unreachable!("Invalid segment value in memory address"),
            },
            offset: value & !MEMSEG_MASK,
        }
    }
}

impl MemBus {
    // TODO: reading from memory (except Scratchpad) should have a 6 cycles delay
    fn internal_read(&self, addr: u32) -> u32 {
        let addr: MemAddr = addr.into();
        match addr.offset {
            0x0000_0000..0x1f00_0000 => todo!(), // Main RAM
            0x1f00_0000..0x1f80_0000 => todo!(), // Expansion Region 1
            0x1f80_0000..0x1f80_1000 => todo!(), // Scratchpad
            0x1f80_1000..0x1f80_2000 => todo!(), // IO ports
            0x1f80_2000..0x1fa0_0000 => todo!(), // Expansion Region 2
            0x1fa0_0000..0x1fc0_0000 => todo!(), // Expansion Region 3
            0x1fc0_0000..0x1fc8_0000 => todo!(), // BIOS ROM
            _ => (),
        };
        0b100000_00001_00010_0000000000000000 // lb 2,[r1+0]
    }

    pub fn read_u8(&self, addr: u32) -> u8 {
        self.internal_read(addr) as u8
    }

    pub fn read_u16(&self, addr: u32) -> u16 {
        self.internal_read(addr) as u16
    }

    pub fn read_u32(&self, addr: u32) -> u32 {
        self.internal_read(addr) as u32
    }

    pub fn write_u8(&mut self, addr: u32, val: u8) {
        todo!()
    }

    pub fn write_u16(&mut self, addr: u32, val: u16) {
        todo!()
    }

    pub fn write_u32(&mut self, addr: u32, val: u32) {
        todo!()
    }
}
