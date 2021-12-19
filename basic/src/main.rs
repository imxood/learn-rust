#![feature(vec_into_raw_parts)]
fn main() {
    use std::mem::ManuallyDrop;
    use std::time::Instant;

    #[repr(C)]
    pub struct Rgb {
        pub r: u8,
        pub g: u8,
        pub b: u8,
        pub a: u8,
    };

    impl Rgb {
        pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
            Self { r, g, b, a }
        }
    }

    #[repr(C)]
    #[derive(Clone)]
    pub struct Color32(pub [u8; 4]);

    impl Color32 {
        pub fn to_tuple(&self) -> (u8, u8, u8, u8) {
            (self.0[0], self.0[1], self.0[2], self.0[3])
        }
    }

    impl Default for Color32 {
        fn default() -> Self {
            Self([0, 1, 2, 3])
        }
    }

    impl Drop for Color32 {
        fn drop(&mut self) {
            println!("now: {:?}", Instant::now());
        }
    }

    let size = (2, 2);
    let width = size.0;
    let height = size.1;
    let len = width * height;

    // 不初始化数据, 直接使用
    let mut pixels = vec![Color32::default(); len];

    let now = Instant::now();

    // 测试 生成 _pixels 的时间, 寻找更好的生成方式

    // {
    //     let pixels = pixels.clone();

    //     // 方法1
    //     let time1 = now.elapsed();

    //     let _pixels: Vec<Vec<[u8; 4]>> = pixels
    //         .chunks(width)
    //         .map(|row| row.iter().map(|srgba| srgba.to_tuple()).collect())
    //         .collect();

    //     let time2 = now.elapsed();

    //     println!("elapsed: {:?}", time2 - time1);
    // }

    {
        let pixels = pixels.clone();

        // 方法2
        let time1 = now.elapsed();

        let (ptr, len, cap) = pixels.into_raw_parts();

        let mut old_pixels = unsafe { Vec::from_raw_parts(ptr as *mut Rgb, len, cap) };
        let mut old_pixels = std::mem::ManuallyDrop::new(old_pixels);
        let (ptr, len, cap) = (
            old_pixels.as_mut_ptr(),
            old_pixels.len(),
            old_pixels.capacity(),
        );

        // let pixels: Vec<Rgb> = unsafe { Vec::from_raw_parts(ptr, len, cap) };

        let mut pixels: Vec<Vec<Rgb>> = (0..height)
            .map(|i| unsafe {
                println!("i * width: {}, width: {}", i * width, width);
                Vec::from_raw_parts(ptr.add(i * width), width, width)
            })
            .collect();

        // let pixels: Vec<Vec<Rgb>> = old_pixels
        //     .chunks_mut(width)
        //     .map(|row| {
        //         // let mut row = std::mem::ManuallyDrop::new(row);
        //         let len = row.len();
        //         unsafe { Vec::from_raw_parts(row.as_mut_ptr(), len, len) }
        //     })
        //     .collect();

        let time2 = now.elapsed();

        // println!("len: {:?}", pixels.len());
        println!("elapsed: {:?}", time2 - time1);
        println!("pixels.len(): {:?}", &pixels.len());
        let mut color1 = pixels.pop().unwrap();
        let color0 = pixels.pop().unwrap();

        // let color1_1 = color1.pop().unwrap();
        // let color1_0 = color1.pop().unwrap();

        // drop(color1_1);
        // drop(color1_0);
        drop(color0);
        drop(color1);
        println!("ok");
    }
}
