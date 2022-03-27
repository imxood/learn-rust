use anyhow::Result;
use tokio::runtime::Runtime;

#[derive(Default)]
struct Client {}

impl Client {
    async fn read(&self) -> Result<()> {
        println!("read");
        Ok(())
    }

    async fn write(&self) -> Result<()> {
        println!("write");
        Ok(())
    }
}

pub struct BlockingClient {
    inner: Client,
    rt: Runtime,
}

impl BlockingClient {
    pub fn new() -> Result<Self> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        Ok(Self {
            rt,
            inner: Client::default(),
        })
    }

    pub fn read(&self) -> Result<()> {
        self.rt.block_on(self.inner.read())
    }

    pub fn write(&self) -> crate::Result<()> {
        self.rt.block_on(self.inner.write())
    }
}

fn main() -> Result<()> {
    let client = BlockingClient::new()?;
    client.read()?;
    client.write()?;
    Ok(())
}
