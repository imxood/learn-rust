#[test]
fn test_f32_compare() {
    let a = 1.0f32.min(f32::INFINITY);
    println!("a: {:?}", a);
    let a = 1.0f32.max(f32::INFINITY);
    println!("a: {:?}", a);
}
