unsafe extern "C" {
    unsafe fn my_function() -> i64;
}

fn main() {
    println!("Hello from Rust!");
    let result = unsafe { my_function() };
    println!("Result from assembly: {}", result);
}
