#![feature(thread_is_running)]

use std::time::Duration;

#[test]
fn test_thread_return() {
    let ret = std::thread::spawn(|| {
        return 1;
    })
    .join()
    .unwrap();
    println!("ret: {}", ret);
}

#[test]
fn test_thread_some() {
    let thread_some = Some(std::thread::spawn(|| {
        println!("new thread");
    }));
    std::thread::sleep(Duration::from_millis(100));
    if let Some(thread) = thread_some.as_ref() {
        println!("thread_some: {:?}, is_running: {}", &thread_some, thread.is_running());
    }
}
