extern "C" {
    fn add(a: i32, b: i32) -> i32;
    fn minus(a: i32, b: i32) -> i32;
}

fn add_wrap(a: i32, b: i32) -> i32 {
    unsafe { add(a, b) }
}

fn minus_wrap(a: i32, b: i32) -> i32 {
    unsafe { minus(a, b) }
}

fn main() {
    println!("add: {}", add_wrap(1, 2));
    println!("minus: {}", minus_wrap(1, 2));
}
