use crate::{
    define_repr_enum,
    psx::cpu::{COP0, CPU},
};

pub const SR_IDX: usize = 12;
pub const CAUSE_IDX: usize = 13;
pub const EPC_IDX: usize = 14;

define_repr_enum! {
    pub enum MoveFromCoproRs : u8 {
        MFC = 0b00000,
        CFC = 0b00010,
        MTC = 0b00100,
        CTC = 0b00110,
    }
}

impl CPU {
    // TODO: handle coprocessor unusable exceptions
    pub fn move_copro(&mut self, move_op: MoveFromCoproRs, op: u8, rt: u8, rd: u8) {
        let rt_idx = rt as usize;
        let rt_val = self.core.get_gpr(rt_idx);
        let rd_idx = rd as usize;
        let rd_val = self.core.get_gpr(rd_idx);

        match move_op {
            MoveFromCoproRs::MFC => self.core.get_gpr(rt_idx)
            MoveFromCoproRs::CFC => todo!(),
            MoveFromCoproRs::MTC => todo!(),
            MoveFromCoproRs::CTC => todo!(),
        }
    }
}
