use bitfield::bitfield;

pub const BPC_IDX: usize = 3;
pub const BDA_IDX: usize = 5;
pub const TAR_IDX: usize = 6;
pub const DCIC_IDX: usize = 7;
pub const BADA_IDX: usize = 8;
pub const BDAM_IDX: usize = 9;
pub const BPCM_IDX: usize = 11;
pub const SR_IDX: usize = 12;
pub const CAUSE_IDX: usize = 13;
pub const EPC_IDX: usize = 14;
pub const PRID_IDX: usize = 15;

pub enum CpuMode {
    KernelMode,
    UserMode,
}

#[derive(Debug)]
pub(super) struct COP0 {
    pub regs: [u32; 16],
}

impl COP0 {
    pub fn get_reg(&self, idx: usize) -> u32 {
        assert!(idx < 64);
        self.regs[idx]
    }

    pub fn set_reg(&mut self, idx: usize, value: u32) {
        assert!(idx < 64);
        self.regs[idx] = value;
    }

    pub fn get_sr(&self) -> SR {
        SR(self.get_reg(SR_IDX))
    }

    pub fn set_sr(&mut self, value: &SR) {
        self.set_reg(SR_IDX, value.0);
    }

    pub fn get_cause(&self) -> CAUSE {
        CAUSE(self.get_reg(CAUSE_IDX))
    }

    pub fn set_cause(&mut self, value: &CAUSE) {
        self.set_reg(CAUSE_IDX, value.0);
    }

    pub fn get_current_mode(&self) -> CpuMode {
        match self.get_sr().kuc() {
            0 => CpuMode::UserMode,
            1 => CpuMode::KernelMode,
            _ => unreachable!("Bitfield not working properly"),
        }
    }
}

bitfield! {
    pub(super) struct SR(u32);
    impl Debug;
    impl new;

    u8;
    pub iec, set_iec : 0, 0;
    pub kuc, set_kuc : 1, 1;
    pub iep, set_iep : 2, 2;
    pub kup, set_kup : 3, 3;
    pub ieo, set_ieo : 4, 4;
    pub kuo, set_kuo : 5, 5;
    pub _, _ : 7, 6;
    pub im, set_im : 15, 8;
    pub isc, set_isc : 16, 16;
    pub swc, set_swc : 17, 17;
    pub pz, set_pz : 18, 18;
    pub cm, set_cm : 19, 19;
    pub pe, set_pe : 20, 20;
    pub ts, set_ts : 21, 21;
    pub bev, set_bev : 22, 22;
    pub _, _ : 24, 23;
    pub re, set_re : 25, 25;
    pub _, _ : 27, 26;
    pub cu0, set_cu0 : 28, 28;
    pub cu1, set_cu1 : 29, 29;
    pub cu2, set_cu2 : 30, 30;
    pub cu3, set_cu3 : 31, 31;
}

bitfield! {
    pub struct CAUSE(u32);
    impl Debug;
    impl new;

    u8;
    pub _, _ : 1, 0;
    pub exc_code, set_exc_code : 6, 2;
    pub _, _ : 7, 7;
    pub ip, set_ip : 15, 8;
    u16;
    pub _, _ : 27, 16;
    u8;
    pub ce, set_ce : 29, 28;
    pub _, _ : 30, 30;
    pub bd, set_bd : 31, 31;
}
