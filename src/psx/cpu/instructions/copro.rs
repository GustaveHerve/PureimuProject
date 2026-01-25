use crate::psx::cpu::COP0;

pub const SR_IDX: usize = 12;
pub const CAUSE_IDX: usize = 13;
pub const EPC_IDX: usize = 14;

impl COP0 {
    pub fn get_reg(&self, idx: usize) -> u32 {
        assert!(idx < 64);
        self.regs[idx]
    }
    pub fn set_reg(&mut self, idx: usize, value: u32) {
        assert!(idx < 64);
        self.regs[idx] = value;
    }
}
