use std::ffi::c_void;

#[test]
fn test_ptr() {
    let s: &str = "ace";
    let ptr: *const u8 = s.as_ptr();
    unsafe {
        println!("{}", *ptr.add(1) as char);
        println!("{}", *ptr.add(2) as char);
    }
}

#[test]
fn test_ptr1() {
    #[repr(C)]
    pub struct Color32(pub [u8; 4]);

    let color = Color32([0xff, 0xff, 0xff, 0xff]);
    let color = &color as *const Color32 as *const u32;

    println!("color: {:x?}", unsafe { color.as_ref() });
    println!("color: {:x?}", unsafe { &*color });
}
