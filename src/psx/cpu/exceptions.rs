use super::Cpu;
use super::cop0::{BADA_IDX, COP0, EPC_IDX, SR};

pub enum ExceptionType {
    Reset,
    MemoryLoad,
    MemoryStore,
    MemoryIFetch,
    BusError,
    BusErrorIFetch,
    Overflow,
    Interrupt,
    Syscall,
    Breakpoint,
    ReservedInstruction,
    CopUnusable0,
    CopUnusable1,
    CopUnusable2,
    CopUnusable3,
}

#[repr(u8)]
enum ExCode {
    Int = 0x00,
    Adel = 0x04,
    Ades = 0x05,
    Ibe = 0x06,
    Dbe = 0x07,
    Syscall = 0x08,
    Bp = 0x09,
    Ri = 0x0A,
    Cpu = 0x0B,
    Ovf = 0x0C,
}

const EXCEPTION_VECTORS: [u32; 4] = [0xbfc0_0000, 0x8000_0000, 0x8000_0040, 0x8000_0080];
const EXCEPTION_VECTORS_BEV: [u32; 4] = [0xbfc0_0000, 0xbfc0_0100, 0xbfc0_0140, 0xbfc0_0180];

const USER_MODE: u8 = 0;
const KERNEL_MODE: u8 = 1;

const INTERRUPT_DISABLE: u8 = 0;
const INTERRUPT_ENABLE: u8 = 1;

impl COP0 {
    pub fn rfe(&mut self) {
        let mut sr = self.get_sr();

        // Restore User/Kernel mode and Interrupt Enable flags
        sr.set_iec(sr.iep());
        sr.set_iep(sr.ieo());

        sr.set_kuc(sr.kup());
        sr.set_kup(sr.kuo());

        // Update SR
        self.set_sr(&sr);
    }
}

impl Cpu {
    pub fn throw_exception(&mut self, instr_pc: u32, exception_type: ExceptionType) {
        let mut cause = self.cop0.get_cause();
        let mut sr = self.cop0.get_sr();

        // Set EPC
        self.cop0.set_reg(EPC_IDX, instr_pc);

        let vector_table: &[u32; 4] = if sr.bev() == 1 {
            &EXCEPTION_VECTORS
        } else {
            &EXCEPTION_VECTORS_BEV
        };

        let handler_addr = match exception_type {
            ExceptionType::Reset => vector_table[0],
            ExceptionType::Breakpoint => vector_table[2],
            _ => vector_table[3],
        };

        // Update SR

        // User/Kernel mode and Interrupt Enable flags pushed
        // in 3-entry stack of SR
        sr.set_ieo(sr.iep());
        sr.set_iep(sr.iec());

        sr.set_kuo(sr.kup());
        sr.set_kup(sr.kuc());

        // Set current mode to kernel and disable interrupts
        sr.set_iec(INTERRUPT_DISABLE);
        sr.set_kuc(KERNEL_MODE);

        self.cop0.set_sr(&sr);

        // Update CAUSE
        let excode: ExCode = match exception_type {
            ExceptionType::Reset => ExCode::Int, // TODO: check reset exception behaviour
            ExceptionType::MemoryLoad | ExceptionType::MemoryIFetch => ExCode::Adel,
            ExceptionType::MemoryStore => ExCode::Ades,
            ExceptionType::BusError => ExCode::Dbe,
            ExceptionType::BusErrorIFetch => ExCode::Ibe,
            ExceptionType::Overflow => ExCode::Ovf,
            ExceptionType::Interrupt => ExCode::Int,
            ExceptionType::Syscall => ExCode::Syscall,
            ExceptionType::Breakpoint => ExCode::Bp,
            ExceptionType::ReservedInstruction => ExCode::Ri,
            ExceptionType::CopUnusable0
            | ExceptionType::CopUnusable1
            | ExceptionType::CopUnusable2
            | ExceptionType::CopUnusable3 => ExCode::Cpu,
        };

        // Set BadVaddr in case of Address exception
        match exception_type {
            ExceptionType::MemoryLoad
            | ExceptionType::MemoryIFetch
            | ExceptionType::MemoryStore => self.cop0.set_reg(BADA_IDX, instr_pc),
            _ => (),
        };

        // Set CE in case of CopUnusable exception
        match exception_type {
            ExceptionType::CopUnusable0 => cause.set_ce(0),
            ExceptionType::CopUnusable1 => cause.set_ce(1),
            ExceptionType::CopUnusable2 => cause.set_ce(2),
            ExceptionType::CopUnusable3 => cause.set_ce(3),
            _ => (),
        };

        cause.set_exc_code(excode as u8);
        self.cop0.set_cause(&cause);

        // Set PC to exception handler
        self.core.pc = handler_addr;
    }
}
