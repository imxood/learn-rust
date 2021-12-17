#[test]
fn test_ptr() {
    let s: &str = "ace";
    let ptr: *const u8 = s.as_ptr();
    unsafe {
        println!("{}", *ptr.add(1) as char);
        println!("{}", *ptr.add(2) as char);
    }
}
