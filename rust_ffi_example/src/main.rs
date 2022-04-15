extern "C" {
    fn add_in_static(a: i32, b: i32) -> i32;
    fn add_in_dyn(a: i32, b: i32) -> i32;
    fn minus_in_static(a: i32, b: i32) -> i32;
    fn minus_in_dyn(a: i32, b: i32) -> i32;
}

fn add_in_static_wrap(a: i32, b: i32) -> i32 {
    unsafe { add_in_static(a, b) }
}

fn add_in_dyn_wrap(a: i32, b: i32) -> i32 {
    unsafe { add_in_dyn(a, b) }
}

fn minus_in_static_wrap(a: i32, b: i32) -> i32 {
    unsafe { minus_in_static(a, b) }
}

fn minus_in_dyn_wrap(a: i32, b: i32) -> i32 {
    unsafe { minus_in_dyn(a, b) }
}

fn main() {
    println!("add_in_static_wrap: {}", add_in_static_wrap(1, 2));
    println!("add_in_dyn_wrap: {}", add_in_dyn_wrap(1, 2));
    println!("minus_in_static_wrap: {}", minus_in_static_wrap(1, 2));
    println!("minus_in_dyn_wrap: {}", minus_in_dyn_wrap(1, 2));
}
