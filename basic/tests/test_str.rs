#[test]
fn test_str1() {
    let mut a = String::from("a");
    a += "1";
    println!("a: {a:?}");
}
