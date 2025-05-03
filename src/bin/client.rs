use mini_redis::{Result, client::connect};

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:6363".to_string();
    let mut client = connect(addr).await?;

    tokio::spawn(async move {
        if let Some(value) = client.get("hello").await.unwrap() {
            println!("Value: {value:?}");
        } else {
            println!("No data");
        }
    })
    .await;

    tokio::spawn(async move {
        let _ = client.set("hello", "world!".into()).await;
    })
    .await;

    Ok(())
}
