use anyhow::Result;
use tokio::time::{sleep, Duration};

pub struct UI {}

impl UI {
    pub async fn new() -> Result<Self> {
        Ok(UI {})
    }

    pub async fn run(&self) -> Result<()> {
        loop {
            println!(".");
            sleep(Duration::from_secs(1)).await;
        }
        Ok(())
    }
}
