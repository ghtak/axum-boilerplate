#![allow(unused)]

use anyhow::{Result, Ok};
use tokio;

#[tokio::test]
async fn test_main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:18089")?;
    hc.do_get("/basic").await?.print().await?;
    Ok(())   
}