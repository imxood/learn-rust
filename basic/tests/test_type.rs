use std::any::type_name_of_val;

#[test]
fn test_type() {
    let a = 1;
    println!("{}", type_name_of_val(&a));
}
