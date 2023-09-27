#![allow(unused)]

use tokio;

#[tokio::test]
async fn test_main() -> anyhow::Result<()> {
    let hc = httpc_test::new_client("http://localhost:18089")?;
    hc.do_get("/basic").await?.print().await?;
    hc.do_get("/basic/error").await?.print().await?;
    hc.do_get("/basic/state").await?.print().await?;
    Ok(())  
}


#[tokio::main]
async fn main(){

}