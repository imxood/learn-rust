#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Target {
    A,
    B,
}

#[test]
fn test_enum() {
    let a = Target::A;
    let a1 = Target::A;
    if a == a1 {
        println!("a == a1");
    } else {
        println!("a != a1");
    }
}
