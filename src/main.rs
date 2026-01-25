#![allow(dead_code)]
#![allow(unused_variables)]

mod psx;

fn main() {
    let mut psx = psx::PSX::new();
    let running = true;
    for _ in 0..50 {
        psx.exec_next_instr();
    }
}
