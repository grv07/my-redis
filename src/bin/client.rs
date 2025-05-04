use bytes::Bytes;
use mini_redis::{Result, client::connect};
use tokio::sync::{mpsc, oneshot};

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Cmd {
    Set {
        key: String,
        value: Bytes,
        resp: Responder<()>,
    },

    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:6363".to_string();
    let mut client = connect(addr).await?;

    let (tx, mut rx) = mpsc::channel(32);

    let tx1 = tx.clone();

    let t1 = tokio::spawn(async move {
        let (res_tx, res_rx) = oneshot::channel();

        tx1.send(Cmd::Set {
            key: "hello".to_string(),
            value: "world!".into(),
            resp: res_tx,
        })
        .await
        .unwrap();

        println!("Send Set");

        let res = res_rx.await;
        println!("Got: {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (res_tx, res_rx) = oneshot::channel();

        tx.send(Cmd::Get {
            key: "hello".to_string(),
            resp: res_tx,
        })
        .await
        .unwrap();

        println!("Send Get");
        let res = res_rx.await;
        println!("Got: {:?}", res);
    });

    let task_manager = tokio::spawn(async move {
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Cmd::Set { key, value, resp } => {
                    let res = client.set(&key, value).await;
                    resp.send(res).unwrap();
                }
                Cmd::Get { key, resp } => {
                    let res = client.get(&key).await;
                    resp.send(res).unwrap();
                }
            }
        }
    });

    task_manager.await.unwrap();
    t1.await.unwrap();
    t2.await.unwrap();

    Ok(())
}
