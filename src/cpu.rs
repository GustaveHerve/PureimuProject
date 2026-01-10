#[derive(Debug)]
pub struct R3000 {
    gpr: [u32; 32],
    pc: u32,
    sp: u32,
    hi: u32,
    lo: u32,
}

#[derive(Debug)]
pub struct CP0 {}

impl R3000 {
    pub fn new() -> R3000 {
        R3000 {
            gpr: [0; 32],
            pc: 0,
            sp: 0,
            hi: 0,
            lo: 0,
        }
    }
    pub fn fetch_decode(&self) {}
}
