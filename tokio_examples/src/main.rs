use anyhow::Result;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let jh1 = tokio::spawn(async {
        let cur = std::time::Instant::now();
        while cur.elapsed() < Duration::from_secs(5) {
            println!("jh1");
        }
    });
    let jh2 = tokio::spawn(async {
        let cur = std::time::Instant::now();
        while cur.elapsed() < Duration::from_secs(10) {
            println!("jh2");
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });
    tokio::spawn(async {
        tokio::time::sleep(Duration::from_secs(1)).await;
    });

    jh1.await?;
    jh2.await?;

    Ok(())
}
