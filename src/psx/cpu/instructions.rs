pub mod alu;
pub mod copro;
pub mod jump;
pub mod load_store;
pub mod special;

use bitfield::bitfield;

bitfield! {
    pub struct IType(u32);
    impl Debug;
    impl new;

    u16;
    pub(super) imm, set_imm : 15, 0;
    u8;
    pub(super) rt, set_rt : 20, 16;
    pub(super) rs, set_rs : 25, 21;
    pub(super) op, set_op : 31, 26;
}

bitfield! {
    pub struct JType(u32);
    impl Debug;
    impl new;

    u32;
    pub(super) imm, set_imm : 25, 0;
    u8;
    pub(super) op, set_op : 31, 26;
}

bitfield! {
    pub struct RType(u32);
    impl Debug;
    impl new;

    u8;
    pub(super) funct, set_funct : 5, 0;
    pub(super) shamt, set_shamt : 10, 6;
    pub(super) rd, set_rd : 15, 11;
    pub(super) rt, set_rt : 20, 16;
    pub(super) rs, set_rs : 25, 21;
    pub(super) op, set_op : 31, 26;
}
