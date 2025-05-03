use mini_redis::{Result, client::connect};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = connect("127.0.0.1:6363").await?;

    // client.set("foo", "value".into()).await?;

    let res = client.get("foo").await?;

    println!("Res: {res:?}");

    Ok(())
}
