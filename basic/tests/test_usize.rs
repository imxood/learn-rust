#[test]
fn test_usize() {
    let u1 = 2usize;
    let u2 = 5usize;

    // let u = u1 - u2; // 报错, 溢出
    // println!("u: {}", u);

    let u = u2 / u1; // 下取整
    println!("u: {}", u);

    let u = (u2 + u1 - 1) / u1; // 上取整
    println!("u: {}", u);

    let u = 1.2 as u8;
    println!("u: {}", u);

    let u = -3.2 as i8;
    println!("u: {}", u);

    let u = -3 as u8;
    println!("u: {}", u);

    let u = -1i8 as u8;
    println!("u: {}", u);
}
