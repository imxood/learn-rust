#![feature(vec_into_raw_parts)]

use std::{io::Read, mem::ManuallyDrop, time::Instant};

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
    let count = datas.iter().filter(|x| **x > 5).count();
    println!("大于5的数量是: {}", count);
}

#[test]
fn test_vec4() {
    let _data1 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]; // 4 * 4 行列数据

    let _width = 4;
    let _height = 4;

    let (x, y, w, h) = (1, 1, 2, 2);

    for i in 0..h {
        let y1 = y + i;
        for j in 0..w {
            let x1 = x + j;
            println!("({},{})", x1, y1);
        }
    }
}

#[test]
fn test_vec5() {
    let len = 32;
    let mut data = Vec::<u8>::with_capacity(len);
    unsafe {
        data.set_len(len);
    }
    for d in data.iter() {
        println!("d: {}", d);
    }
    println!("d.capacity(): {}, d.len(): {}", data.capacity(), data.len());
}

#[test]
fn test_vec6() {
    use std::time::Instant;

    pub struct Color32(pub [u8; 4]);

    impl Color32 {
        pub fn to_tuple(&self) -> (u8, u8, u8, u8) {
            (self.0[0], self.0[1], self.0[2], self.0[3])
        }
    }

    let size = (1280, 720);
    let len = size.0 * size.1;

    // 不初始化数据, 直接使用
    let mut pixels = Vec::<Color32>::with_capacity(len);
    unsafe {
        pixels.set_len(len);
    }

    let now = Instant::now();

    let time1 = now.elapsed();

    // 测试 生成 _pixels 的时间, 寻找更好的计算法师

    let _pixels: Vec<Vec<(u8, u8, u8, u8)>> = pixels
        .chunks(size.0 as usize)
        .map(|row| row.iter().map(|srgba| srgba.to_tuple()).collect())
        .collect();

    let time2 = now.elapsed();

    println!("elapsed: {:?}", time2 - time1);
}

#[test]
fn test_vec7() {
    use std::mem::ManuallyDrop;
    use std::time::Instant;

    #[derive(Clone)]
    pub struct Color32(pub [u8; 4]);

    impl Color32 {
        pub fn to_tuple(&self) -> (u8, u8, u8, u8) {
            (self.0[0], self.0[1], self.0[2], self.0[3])
        }
    }

    let size = (1280, 720);
    let width = size.0;
    let height = size.1;
    let len = width * height;

    // 不初始化数据, 直接使用
    let mut pixels = Vec::<Color32>::with_capacity(len);
    unsafe {
        pixels.set_len(len);
    }

    let now = Instant::now();

    // 测试 生成 _pixels 的时间, 寻找更好的生成方式

    {
        let pixels = pixels.clone();

        // 方法1
        let time1 = now.elapsed();

        let _pixels: Vec<Vec<(u8, u8, u8, u8)>> = pixels
            .chunks(width)
            .map(|row| row.iter().map(|srgba| srgba.to_tuple()).collect())
            .collect();

        let time2 = now.elapsed();

        println!("elapsed: {:?}", time2 - time1);
    }

    {
        let pixels = pixels.clone();

        // 方法2
        let time1 = now.elapsed();

        let (ptr, len, cap) = pixels.into_raw_parts();

        let mut old_pixels = unsafe { Vec::from_raw_parts(ptr as *mut (u8, u8, u8, u8), len, cap) };
        // let mut old_pixels = ManuallyDrop::new(old_pixels);

        let pixels: Vec<Vec<(u8, u8, u8, u8)>> = old_pixels
            .chunks_mut(width)
            .map(|row| {
                // let mut row = std::mem::ManuallyDrop::new(row);
                let len = row.len();
                unsafe { Vec::from_raw_parts(row.as_mut_ptr(), len, len) }
            })
            .collect();

        unsafe {
            old_pixels.set_len(0);
        }

        std::mem::forget(old_pixels);

        // drop(pixels);
        // ManuallyDrop::into_inner(old_pixels);

        let time2 = now.elapsed();

        println!("len: {:?}", pixels.len());
        println!("elapsed: {:?}", time2 - time1);
    }
}

#[test]
fn test_vect8() {
    let raw_data = vec![01u8; 20];
    let mut data = std::mem::ManuallyDrop::new(raw_data);
    let new_data = unsafe { Vec::from_raw_parts(data.as_mut_ptr().add(8), 8, 8) };
    drop(new_data);
    println!("ok");
}

#[test]
fn test_mem_vec() {
    let a = test_mem_vec1();
    println!("new a: {:?}", &a);
}

fn test_mem_vec1() -> Vec<u8> {
    let mut a = vec![1u8, 2, 3, 4];
    let b = unsafe { Vec::from_raw_parts(a.as_mut_ptr(), a.len(), a.capacity()) };
    println!("a: {:?}", &a);
    println!("b: {:?}", &b);
    std::mem::forget(a);
    b
}
