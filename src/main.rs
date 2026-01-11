#![allow(dead_code)]

mod psx;

fn main() {
    let psx = psx::PSX::new();
    let running = true;
    while running {
        psx.next_instruction();
        break;
    }
}
