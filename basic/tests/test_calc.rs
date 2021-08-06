#[test]
fn test_calc() {
    let mut num: i8 = 100;
    num = num.wrapping_add(100);
    println!("num: {}", num);
}
