#[test]
fn test_slice() {
    let mut dest: [u8; 10] = [0; 10];
    println!("dest: {:?}", &dest);
    let src: [u8; 5] = [1, 2, 3, 4, 5];
    dest.copy_from_slice(&src);
    println!("dest: {:?}", &dest);

    let a = &src[..3];
    println!("a: {:?}", a);

    // let a = &src[-2];
    // println!("a: {:?}", a);
}
