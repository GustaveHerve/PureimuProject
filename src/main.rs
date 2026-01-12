mod psx;

fn main() {
    let mut psx = psx::PSX::new();
    let running = true;
    while running {
        psx.next_instruction();
        break;
    }
}
