const MAIN_RAM_SIZE: usize = 2048 << 10;
const EXPANSION_1_SIZE: usize = 8192 << 10;
const SCRATCHPAD_SIZE: usize = 1 << 10;
const IO_SIZE: usize = 8 << 10;
const EXPANSION_2_SIZE: usize = 8 << 10;
const EXPANSION_3_SIZE: usize = 2048 << 10;
const BIOS_SIZE: usize = 512 << 10;
const IO_CACHE_SIZE: usize = 512;

const MAIN_RAM_WORDS: usize = MAIN_RAM_SIZE / 4;
const EXPANSION_1_WORDS: usize = EXPANSION_1_SIZE / 4;
const SCRATCHPAD_WORDS: usize = SCRATCHPAD_SIZE / 4;
const IO_WORDS: usize = IO_SIZE / 4;
const EXPANSION_2_WORDS: usize = EXPANSION_2_SIZE / 4;
const EXPANSION_3_WORDS: usize = EXPANSION_3_SIZE / 4;
const BIOS_WORDS: usize = BIOS_SIZE / 4;
const IO_CACHE_WORDS: usize = IO_CACHE_SIZE / 4;

const MEMSEG_MASK: u32 = 0xe000_0000;

pub enum MemSegment {
    KUSEG,
    KSEG0,
    KSEG1,
    KSEG2,
}

#[derive(Debug)]
pub enum MemException {
    UnauthorizedAccess,
    AddressError,
    BusError,
}

pub struct MemAddr {
    pub seg: MemSegment,
    pub offset: u32,
}

pub struct MemBus {
    main_ram: [u32; MAIN_RAM_WORDS],
    expansion_1: [u32; EXPANSION_1_WORDS],
    scratchpad: [u32; SCRATCHPAD_WORDS],
    io: [u32; IO_WORDS],
    expansion_2: [u32; EXPANSION_2_WORDS],
    expansion_3: [u32; EXPANSION_3_WORDS],
    bios: [u32; BIOS_WORDS],
    io_cache: [u32; IO_CACHE_WORDS],
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
    pub fn new() -> MemBus {
        MemBus {
            main_ram: [0; MAIN_RAM_WORDS],
            expansion_1: [0; EXPANSION_1_WORDS],
            scratchpad: [0; SCRATCHPAD_WORDS],
            io: [0; IO_WORDS],
            expansion_2: [0; EXPANSION_2_WORDS],
            expansion_3: [0; EXPANSION_3_WORDS],
            bios: [0; BIOS_WORDS],
            io_cache: [0; IO_CACHE_WORDS],
        }
    }

    // TODO: reading from memory (except Scratchpad) should have a 6 cycles delay
    pub fn read_bus(&self, addr: u32) -> Result<u32, MemException> {
        let addr_aligned = addr & !0b11;
        let addr: MemAddr = addr_aligned.into();
        let idx: usize = (addr.offset >> 2) as usize;
        match addr.offset {
            0x0000_0000..0x1f00_0000 => Ok(self.main_ram[idx & (MAIN_RAM_WORDS - 1)]), // Main RAM
            0x1f00_0000..0x1f80_0000 => Ok(self.expansion_1[idx & (EXPANSION_1_WORDS - 1)]), // Expansion Region 1
            0x1f80_0000..0x1f80_1000 => Ok(self.scratchpad[idx & (SCRATCHPAD_WORDS - 1)]), // Scratchpad
            0x1f80_1000..0x1f80_2000 => Ok(self.io[idx & (IO_WORDS - 1)]), // IO ports
            0x1f80_2000..0x1fa0_0000 => Ok(self.expansion_2[idx & (EXPANSION_2_WORDS - 1)]), // Expansion Region 2
            0x1fa0_0000..0x1fc0_0000 => Ok(self.expansion_3[idx & (EXPANSION_3_WORDS - 1)]), // Expansion Region 3
            0x1fc0_0000..0x1fc8_0000 => Ok(self.bios[idx & (BIOS_WORDS - 1)]), // BIOS ROM
            _ => Err(MemException::BusError),
        }
    }
}
