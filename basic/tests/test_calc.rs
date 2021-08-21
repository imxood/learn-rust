use std::ffi::c_void;

#[test]
fn test_calc() {
    let mut num: i8 = 100;
    num = num.wrapping_add(100);
    println!("num: {}", num);
}

#[test]
fn test_vect() {
    let data = vec![(1, 2, 3), (4, 5, 6), (7, 8, 9), (10, 11, 12), (13, 14, 15)];
    let len = data.len() * 3;
    let cap = data.capacity() * 3;
    let p = data.leak() as *mut _ as *mut i32;

    let data_slice = unsafe { std::slice::from_raw_parts(p, len) };
    println!("data slice: {:?}", data_slice);

    let data_vec = unsafe { Vec::from_raw_parts(p, len, cap) };
    println!("data vec: {:?}", &data_vec);

    println!("ok");
}
