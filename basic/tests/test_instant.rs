use std::time::{Duration, Instant};

#[test]
fn test_instant() {
    let start = Instant::now();

    let start0 = start;
    std::thread::spawn(move || loop {
        println!("elapsed: {:?}", start0.elapsed());
        std::thread::sleep(Duration::from_millis(1000));
    });

    std::thread::spawn(move || loop {
        println!("elapsed: {:?}", start.elapsed());
        std::thread::sleep(Duration::from_millis(1000));
    });

    std::thread::sleep(Duration::from_millis(10000));
}
