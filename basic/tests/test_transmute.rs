use std::ffi::c_void;

#[test]
fn test_transmute() {
    let a: [u8; 4] = [0, 1, 2, 3];
    println!("a ptr: {:?}", &a as *const u8);
    let b = unsafe { std::mem::transmute::<[u8; 4], u32>(a) };
    println!("b ptr: {:?}", &b as *const u32 as *const c_void as *const u8);
}
