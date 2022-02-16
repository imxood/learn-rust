use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_blocking_task() {
    let blocking_task = tokio::task::spawn_blocking(|| {
        println!("blocking task begin...");
        std::thread::sleep(Duration::from_millis(5000));
        println!("blocking task end");
    });

    tokio::spawn(async {
        println!("async task begin...");
        tokio::time::sleep(Duration::from_millis(2000)).await;
        println!("async task end");
    })
    .await
    .unwrap();

    blocking_task.await.unwrap();
}

#[tokio::test]
async fn test_poll_result() {
    let mut tasks = Vec::new();
    tasks.push(tokio::spawn(async {
        sleep(Duration::from_millis(10000)).await;
        println!("sleep exit");
    }));
    loop {
        for task in tasks.iter() {
            // let ret = task.poll_unpin();
        }
    }
    sleep(Duration::from_millis(1000)).await;
    println!("abort exit");
}
