use crate::d400::CameraService;
use std::{
    env,
    time::{Duration, Instant},
};

mod d400;

fn test_fetch() {
    env::set_var("RUST_LOG", "DEBUG");
    env_logger::init();

    // let a = [1u8, 2, 3, 4, 5, 6];

    // let rgb = a.as_ptr() as *const u8 as *const ColorRGB;
    // log::info!("rgb: {:?}", unsafe { &*rgb });

    let depth_min_distance = 0.2;
    let depth_max_distance = 3.0;

    let mut cs = CameraService::new("realsense-config.json".to_string());
    cs.open().unwrap();

    log::info!("camera open success");

    let now = Instant::now();
    let mut count = 0;

    let screen_width = 3 * 16;
    let screen_height = 5 * 16;

    // 1秒钟采集30帧图片
    loop {
        let mut start = now.elapsed();
        let mut camera_frame = cs.fetch_frame(Some(Duration::from_millis(5000))).unwrap();
        log::info!("fetch camera frame: {:?}", now.elapsed() - start);
        let time1 = now.elapsed();
        let camera_rgb_data =
            camera_frame.to_rgb(depth_min_distance, depth_max_distance, (0, 0, 0, 0), None);
        log::info!("time1: {:?}", now.elapsed() - time1);

        count += 1;
        if count == 30 {
            log::info!("elapsed: {:?}", now.elapsed() - start);
            start = now.elapsed();
            count = 0;
        }
    }
}

fn test_poll() {
    env::set_var("RUST_LOG", "DEBUG");
    env_logger::init();

    let mut cs = CameraService::new("realsense-config.json".to_string());
    cs.open().unwrap();

    log::info!("camera open success");

    let now = Instant::now();

    // 1秒钟采集30帧图片
    loop {
        let start = now.elapsed();
        spin_sleep::sleep(Duration::from_millis(10));
        let camera_frame = cs.poll_frame().unwrap();
        if camera_frame.is_some() {
            log::info!("poll camera frame: {:?}", now.elapsed() - start);
        }
    }
}

fn main() {
    // test_fetch();
    test_poll();
}
