fn main() {
    #[repr(C)]
    pub struct Color32(pub [u8; 4]);

    let color = Color32([0xff, 0xff, 0xff, 0xff]);
    let color = &color as *const Color32 as *const u32;

    println!("color: {:x?}", unsafe { color.as_ref() });
    println!("color: {:x?}", unsafe { *color });
}
