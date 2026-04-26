mod psx;

fn main() {
    let mut psx = Box::new(psx::Psx::new());
    psx.execution_loop();
}
