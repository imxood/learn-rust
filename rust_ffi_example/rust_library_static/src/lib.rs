#[no_mangle]
pub extern "C" fn minus_in_static(a: i32, b: i32) -> i32 {
    a - b
}
