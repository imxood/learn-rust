use std::{
    sync::{Arc, Mutex},
    thread::spawn,
};

#[derive(Debug)]
struct Config {
    a: i32,
}

#[test]
fn test_mutex() {
    let a = Arc::new(Mutex::new(Config { a: 1 }));
    let a1 = a.clone();

    let task1 = spawn(move || {
        let a1 = a1.lock().unwrap();
        let p_a1 = &*a1 as *const Config;
        println!("p_a1: {:?}", p_a1);
    });

    let task2 = spawn(move || {
        let a = a.lock().unwrap();
        let p_a = &*a as *const Config;
        println!("p_a: {:?}", p_a);
    });

    task1.join().unwrap();
    task2.join().unwrap();
}

#[test]
fn test_parking_mutex() {
    let a = Arc::new(parking_lot::Mutex::new(Config { a: 1 }));
    let a1 = a.clone();

    let task1 = spawn(move || {
        let a1 = a1.lock();
        let p_a1 = &*a1 as *const Config;
        println!("p_a1: {:?}", p_a1);
    });

    let task2 = spawn(move || {
        let a = a.lock();
        let p_a = &*a as *const Config;
        println!("p_a: {:?}", p_a);
    });

    task1.join().unwrap();
    task2.join().unwrap();
}
