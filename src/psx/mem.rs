const MEMSEG_MASK: u32 = 0xe000_0000;

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

pub struct MemBus {}

impl MemAddr {
    pub fn from_u32(addr: u32) -> MemAddr {
        MemAddr {
            seg: match (addr & MEMSEG_MASK) >> 29 {
                0x000..=0x011 => MemSegment::KUSEG,
                0x100 => MemSegment::KSEG0,
                0x101 => MemSegment::KSEG1,
                0x110..=0x111 => MemSegment::KSEG2,
                _ => unreachable!("Invalid segment value in memory address"),
            },
            offset: addr & !MEMSEG_MASK,
        }
    }
}

impl MemBus {
    pub fn read_mem(&self, addr: u32) -> u32 {
        let addr: MemAddr = MemAddr::from_u32(addr);
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
}
