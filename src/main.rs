mod psx;

fn main() {
    let mut psx = psx::Psx::new();
    for _ in 0..50 {
        psx.exec_next_instr();
    }
}
