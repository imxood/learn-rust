#[test]
fn test_vect() {
    let mut arr1 = vec![0; 5];
    let mut arr2: Vec<i32> = Vec::new();
    arr2.push(1);
    arr2.push(2);
    println!("arr1: {:?}, arr2: {:?}", &arr1, &arr2);
    arr1.append(&mut arr2);
    println!("arr1: {:?}, arr2: {:?}", &arr1, &arr2);
}