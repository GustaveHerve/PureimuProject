#![allow(dead_code)]
#![allow(unused_variables)]

mod psx;

fn main() {
    let mut psx = psx::PSX::new();
    let running = true;
    while running {
        psx.exec_next_instr();
        break;
    }
}
