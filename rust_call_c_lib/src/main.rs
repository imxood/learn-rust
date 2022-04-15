#[link(name = "hello", kind = "dylib")]
extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

fn add_wrap(a: i32, b: i32) -> i32 {
    unsafe { add(a, b) }
}

fn main() {
    println!("sum: {}", add_wrap(1, 2));
}
