use anyhow::Result;
use tokio::sync::mpsc::unbounded_channel;

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, mut rx) = unbounded_channel::<i32>();
    let tx1 = tx.clone();

    let jh1 = tokio::spawn(async move {
        loop {
            let i = 1;
            tx.send(i).unwrap();
            // println!("tx: {:?}", i);
            // tokio::time::sleep(Duration::from_millis(10)).await;
        }
    });

    let jh2 = tokio::spawn(async move {
        loop {
            let i = 2;
            tx1.send(i).unwrap();
        }
    });

    let jh3 = tokio::spawn(async move {
        while let Some(i) = rx.recv().await {
            println!("rx: {:?}", i);
        }
    });

    jh1.await.unwrap();
    jh2.await.unwrap();
    jh3.await.unwrap();

    Ok(())
}
