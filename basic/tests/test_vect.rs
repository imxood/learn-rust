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

#[test]
fn test_vec4() {
    let data1 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]; // 4 * 4 行列数据

    let width = 4;
    let height = 4;

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
