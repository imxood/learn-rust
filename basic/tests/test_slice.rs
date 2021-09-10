use core::slice;

#[test]
fn test_slice() {
    let mut dest: [u8; 10] = [0; 10];
    println!("dest: {:?}", &dest);
    let mut src: [u8; 5] = [1, 2, 3, 4, 5];
    dest.copy_from_slice(&src);
    println!("dest: {:?}", &dest);
}
