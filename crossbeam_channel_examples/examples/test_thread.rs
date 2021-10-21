use std::time::Duration;

fn main() {
    let mut count = 1000;
    while count > 0 {
        count -= 1;

        let now = std::time::Instant::now();
        std::thread::sleep(Duration::from_micros(1));
        println!("send thread, delta: {:?}", now.elapsed());
    }
}
