use bytes::Bytes;
use mini_redis::{Result, client::connect};
use tokio::sync::mpsc;

#[derive(Debug)]
enum Cmd {
    Set { key: String, value: Bytes },
    Get { key: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:6363".to_string();
    let mut client = connect(addr).await?;

    let (tx, mut rx) = mpsc::channel(32);

    let tx1 = tx.clone();

    let t1 = tokio::spawn(async move {
        tx1.send(Cmd::Set {
            key: "hello".to_string(),
            value: "world!".into(),
        })
        .await
        .unwrap();

        tx1.send(Cmd::Set {
            key: "ping".to_string(),
            value: "pong".into(),
        })
        .await
        .unwrap();

        tx1.send(Cmd::Set {
            key: "yin".to_string(),
            value: "yan".into(),
        })
        .await
        .unwrap();
    });

    let t2 = tokio::spawn(async move {
        tx.send(Cmd::Get {
            key: "hello".to_string(),
        })
        .await
        .unwrap();

        tx.send(Cmd::Get {
            key: "ping".to_string(),
        })
        .await
        .unwrap();

        tx.send(Cmd::Get {
            key: "yin".to_string(),
        })
        .await
        .unwrap();
    });

    t1.await.unwrap();
    t2.await.unwrap();

    let task_manager = tokio::spawn(async move {
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Cmd::Set { key, value } => client.set(&key, value).await.unwrap(),
                Cmd::Get { key } => {
                    if let Some(value) = client.get(&key).await.unwrap() {
                        println!("Value: {value:?}");
                    }
                }
            }
        }
    });

    task_manager.await.unwrap();

    Ok(())
}
