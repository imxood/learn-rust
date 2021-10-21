use std::{
    thread,
    time::{self, Duration},
};

use crossbeam_channel::unbounded;

#[test]
fn test_channel() {
    let (tx, rx) = unbounded::<usize>();
    let (cmd_tx, cmd_rx) = unbounded::<usize>();

    let jh = thread::spawn(move || {
        let now = time::Instant::now();
        while let Ok(d) = rx.recv() {
            match cmd_rx.try_recv() {
                Ok(cmd) => {
                    println!("cmd: {}", cmd);
                    if cmd == 0 {
                        break;
                    }
                }
                Err(e) => {
                    println!("e: {:?}", &e);
                }
            }
            println!("recv: {}, delta: {:?}", d, now.elapsed());
            spin_sleep::sleep(Duration::from_secs(1));
        }
    });
    let jh1 = thread::spawn(move || {
        let mut count = 1000;
        while count > 0 {
            let now = time::Instant::now();
            tx.send(count).unwrap();
            count -= 1;
            spin_sleep::sleep(Duration::from_millis(1));
        }
        // println!("send thread, delta: {:?}", now.elapsed());
        println!("done");
        cmd_tx.send(0).unwrap();
    });

    jh.join().unwrap();
    jh1.join().unwrap();
}

#[test]
fn test_thread() {
    let mut count = 1000;
    while count > 0 {
        count -= 1;

        let now = time::Instant::now();
        // thread::sleep(Duration::from_millis(1));
        spin_sleep::sleep(Duration::from_millis(1));
        println!("send thread, delta: {:?}", now.elapsed());
    }
}
