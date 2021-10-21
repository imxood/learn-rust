#[test]
fn test_thread_return() {
    let ret = std::thread::spawn(|| {
        return 1;
    })
    .join()
    .unwrap();
    println!("ret: {}", ret);
}
