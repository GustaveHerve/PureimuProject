use crate::{
    define_repr_enum,
    psx::{
        cpu::{
            CPU,
            instructions::copro::{CAUSE_IDX, EPC_IDX, SR_IDX},
        },
        mem::MemBus,
    },
};

use bitfield::bitfield;

bitfield! {
    struct SR(u32);
    impl Debug;
    impl new;

    u8;
    pub(super) iec, set_iec : 0, 0;
    pub(super) kuc, set_kuc : 1, 1;
    pub(super) iep, set_iep : 2, 2;
    pub(super) kup, set_kup : 3, 3;
    pub(super) ieo, set_ieo : 4, 4;
    pub(super) kuo, set_kuo : 5, 5;
    pub(super) _, _ : 7, 6;
    pub(super) im, set_im : 15, 8;
    pub(super) isc, set_isc : 16, 16;
    pub(super) swc, set_swc : 17, 17;
    pub(super) pz, set_pz : 18, 18;
    pub(super) cm, set_cm : 19, 19;
    pub(super) pe, set_pe : 20, 20;
    pub(super) ts, set_ts : 21, 21;
    pub(super) bev, set_bev : 22, 22;
    pub(super) _, _ : 24, 23;
    pub(super) re, set_re : 25, 25;
    pub(super) _, _ : 27, 26;
    pub(super) cu0, set_cu0 : 28, 28;
    pub(super) cu1, set_cu1 : 29, 29;
    pub(super) cu2, set_cu2 : 30, 30;
    pub(super) cu3, set_cu3 : 31, 31;
}

bitfield! {
    struct CAUSE(u32);
    impl Debug;
    impl new;

    u8;
    pub(super) _, _ : 1, 0;
    pub(super) exc_code, set_exc_code : 6, 2;
    pub(super) _, _ : 7, 7;
    pub(super) ip, set_ip : 15, 8;
    u16;
    pub(super) _, _ : 27, 16;
    u8;
    pub(super) ce, set_ce : 29, 28;
    pub(super) _, _ : 30, 30;
    pub(super) bd, set_bd : 31, 31;
}

pub enum ExceptionType {
    Reset,
    MemoryLoad,
    MemoryStore,
    BusError,
    Overflow,
    Interrupt,
    Syscall,
    Breakpoint,
    ReservedInstr,
    CopUnusable,
    MemoryIFetch,
    BusErrorIFetch,
}

#[repr(u8)]
enum ExCode {
    INT = 0x00,
    ADEL = 0x04,
    ADES = 0x05,
    IBE = 0x06,
    DBE = 0x07,
    SYSCALL = 0x08,
    BP = 0x09,
    RI = 0x0A,
    CpU = 0x0B,
    OVF = 0x0C,
}

const EXCEPTION_VECTORS: [u32; 4] = [0xbfc0_0000, 0x8000_0000, 0x8000_0040, 0x8000_0080];
const EXCEPTION_VECTORS_BEV: [u32; 4] = [0xbfc0_0000, 0xbfc0_0100, 0xbfc0_0140, 0xbfc0_0180];

const KERNEL_MODE: u8 = 0;
const USER_MODE: u8 = 1;

const INTERRUPT_DISABLE: u8 = 0;
const INTERRUPT_ENABLE: u8 = 1;

impl CPU {
    pub fn throw_exception(&mut self, bus: &MemBus, exception_type: ExceptionType) {
        let mut cause = CAUSE(self.cop0.get_reg(CAUSE_IDX));
        let mut sr = SR(self.cop0.get_reg(SR_IDX));

        // Set EPC
        // TODO: pass each instruction address as argument instead
        self.cop0.set_reg(EPC_IDX, self.core.pc);

        let vector_table: &[u32; 4] = if sr.bev() == 1 {
            &EXCEPTION_VECTORS
        } else {
            &EXCEPTION_VECTORS_BEV
        };

        let handler_addr = match exception_type {
            ExceptionType::Reset => vector_table[0],
            ExceptionType::Breakpoint => vector_table[1],
            _ => vector_table[2],
        };

        // User/Kernel mode and Interrupt Enable flags pushed
        // in 3-entry stack of SR
        sr.set_ieo(sr.iep());
        sr.set_iep(sr.iec());

        sr.set_kuo(sr.kup());
        sr.set_kup(sr.kuc());

        // Set current mode to kernel and disable interrupts
        sr.set_iec(INTERRUPT_DISABLE);
        sr.set_kuc(KERNEL_MODE);

        // Update SR
        self.cop0.set_reg(SR_IDX, sr.0);
    }
}
