use crate::psx::{
    cpu::{
        CPU,
        instructions::{IType, RType},
    },
    mem::MemBus,
};

pub const SR_IDX: usize = 12;
pub const CAUSE_IDX: usize = 13;
pub const EPC_IDX: usize = 14;

impl CPU {
    // TODO: handle coprocessor unusable exceptions

    pub fn move_from_cop0(&mut self, instr: RType) {
        let rt_idx = instr.rt() as usize;
        let rd_idx = instr.rd() as usize;

        self.core.set_gpr(rt_idx, self.cop0.get_reg(rd_idx));
    }

    pub fn move_to_cop0(&mut self, instr: RType) {
        let rt_idx = instr.rt() as usize;
        let rd_idx = instr.rd() as usize;

        self.cop0.set_reg(rd_idx, self.core.get_gpr(rt_idx));
    }

    pub fn lwc0(&mut self, bus: &MemBus, instr: IType) {
        let rs_idx = instr.rs() as usize;
        let rs_val = self.core.get_gpr(rs_idx);
        let rt_idx = instr.rt() as usize;

        let addr = rs_val.wrapping_add_signed(instr.imm().cast_signed() as i32);
        self.cop0.set_reg(rt_idx, bus.read_u32(addr));
    }

    pub fn swc0(&mut self, bus: &mut MemBus, instr: IType) {
        let rs_idx = instr.rs() as usize;
        let rs_val = self.core.get_gpr(rs_idx);
        let rt_idx = instr.rt() as usize;

        let addr = rs_val.wrapping_add_signed(instr.imm().cast_signed() as i32);
        bus.write_u32(addr, self.cop0.get_reg(rt_idx));
    }
}
