use std::time::Instant;

use crossbeam_channel::unbounded;

#[derive(Clone, Copy)]
struct TestData1 {
    pub a: i32,
}

fn main() {
    let (tx, rx) = unbounded::<TestData1>();

    let now1 = Instant::now();
    let now2 = Instant::now();

    let jh1 = std::thread::spawn(move || {
        while let Ok(_data) = rx.recv() {
            println!("");
        }
        println!("delta1: {:?}", now1.elapsed());
    });

    let jh2 = std::thread::spawn(move || {
        let mut i = 10000;
        let data = TestData1 { a: 1 };
        while i != 0 {
            tx.send(data).unwrap();
            i -= 1;
        }
        println!("delta1: {:?}", now2.elapsed());
    });

    jh1.join().unwrap();
    jh2.join().unwrap();
}
