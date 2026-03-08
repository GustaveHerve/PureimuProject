use crate::psx::{
    cpu::{
        CPU,
        exceptions::ExceptionType,
        instructions::{IType, RType},
    },
    mem::MemBus,
};

impl CPU {
    // TODO: handle coprocessor unusable exceptions

    pub fn decode_cop0(&mut self, instr: RType, instr_pc: u32) {
        match instr.rs() {
            0b00000 | 0b00010 => self.move_from_cop0(instr),
            0b00100 | 0b00110 => self.move_to_cop0(instr),
            0b10000 if instr.funct() == 0b010000 => self.cop0.rfe(),
            _ => self.throw_exception(instr_pc, ExceptionType::CopUnusable0),
        }
    }

    fn move_from_cop0(&mut self, instr: RType) {
        let rt_idx = instr.rt() as usize;
        let rd_idx = instr.rd() as usize;

        self.core.set_gpr(rt_idx, self.cop0.get_reg(rd_idx));
    }

    fn move_to_cop0(&mut self, instr: RType) {
        let rt_idx = instr.rt() as usize;
        let rd_idx = instr.rd() as usize;

        self.cop0.set_reg(rd_idx, self.core.get_gpr(rt_idx));
    }

    pub fn lwc0(&mut self, bus: &MemBus, instr: IType) {
        let rs_idx = instr.rs() as usize;
        let rs_val = self.core.get_gpr(rs_idx);
        let rt_idx = instr.rt() as usize;

        let addr = rs_val.wrapping_add_signed(instr.imm().cast_signed() as i32);
        self.cop0.set_reg(rt_idx, self.read_u32(bus, addr).unwrap());
    }

    pub fn swc0(&mut self, bus: &mut MemBus, instr: IType) {
        let rs_idx = instr.rs() as usize;
        let rs_val = self.core.get_gpr(rs_idx);
        let rt_idx = instr.rt() as usize;

        let addr = rs_val.wrapping_add_signed(instr.imm().cast_signed() as i32);
        self.write_u32(bus, addr, self.cop0.get_reg(rt_idx));
    }

    pub fn decode_cop2(&mut self, instr: RType) {
        todo!("COP2 instructions not yet implemented")
    }
}
