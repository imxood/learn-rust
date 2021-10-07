#[test]
fn test_vec() {
    let mut arr1 = vec![0; 5];
    let mut arr2: Vec<i32> = Vec::new();
    arr2.push(1);
    arr2.push(2);
    println!("arr1: {:?}, arr2: {:?}", &arr1, &arr2);
    arr1.append(&mut arr2);
    println!("arr1: {:?}, arr2: {:?}", &arr1, &arr2);
}

#[test]
fn test_vec2() {
    let datas = [1, 3, 5, 7, 2, 4, 6, 8];
    let mut ret: Vec<bool> = datas.iter().map(|x| *x > 5).collect();
    ret.retain(|x| *x);
    println!("大于5的数量是: {}", ret.len());
}

#[test]
fn test_vec3() {
    let datas = [1, 3, 5, 7, 2, 4, 6, 8];
    let mut count = datas.iter().filter(|x| **x > 5).count();
    println!("大于5的数量是: {}", count);
}
