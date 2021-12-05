use std::{
    thread::{self, sleep, spawn},
    time::{self, Duration},
};

use crossbeam_channel::{unbounded, bounded};

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


/* 
    这里的 mpsc 只是， 多个发送 多个接收, 但 数据只能被 一个 rx接收后, 这个数据就没了， 不会存在其它的 rx 中
*/
#[test]
fn test_mpmc() {
    let (tx, rx) = bounded::<u32>(10);
    let rx0 = rx.clone();

    spawn(move || loop {
        if rx0.is_empty() {
            println!("rx0 -- is empty");
        } else {
            println!("rx0 -- is not empty");
        }
        // if let Ok(data) = rx0.try_recv() {
        //     println!("rx0 -- recved data: {}", data);
        // }
        sleep(Duration::from_millis(1000));
    });

    spawn(move || loop {
        if rx.is_empty() {
            println!("rx -- is empty");
        } else {
            println!("rx -- is not empty");
        }
        if let Ok(data) = rx.try_recv() {
            println!("rx -- recved data: {}", data);
        }
        sleep(Duration::from_millis(1000));
    });

    let mut i = 0u32;
    loop {
        sleep(Duration::from_millis(3000));
        if tx.send(i).is_err() {
            println!("tx -- send error");
        }
        i += 1;
    }
}
