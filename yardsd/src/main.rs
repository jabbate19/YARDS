mod dhcp;
mod dns;

use anyhow::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let host = env::var("HOST")?;
    let client = reqwest::Client::new();
    loop {
        dhcp::main(&host).await?;
        dns::main(&host).await?;
        client
            .post(format!("{}/api/agent/0/success", host))
            .send()
            .await?;
        // Sleep for 30 seconds
        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    }
}
