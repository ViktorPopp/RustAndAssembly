unsafe extern "C" {
    unsafe fn print() -> i64;
}

fn main() {
    println!("Hello from Rust!");
    unsafe { print() };
}
