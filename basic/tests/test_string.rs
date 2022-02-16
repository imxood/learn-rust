#[test]
fn test_string1() {
    let a = "明天, 你好";
    println!("size: {}", a.len());
    for c in a.chars() {
        print!("{}", &c);
    }
    println!();
}
