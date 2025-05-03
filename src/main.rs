use bytes::Bytes;
use mini_redis::{Command, Connection, Frame};
use std::{collections::HashMap, sync::Arc, sync::Mutex};
use tokio::net::TcpListener;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let port = "127.0.0.1:6363";
    let listen = TcpListener::bind(port).await.unwrap();
    println!("Server Strating at {port}");

    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let db = db.clone();
        let (stream, _addr) = listen.accept().await.unwrap();

        tokio::spawn(async move {
            process(stream, db).await;
        });
    }
}

async fn process(socket: tokio::net::TcpStream, db: Db) {
    // let mut db = HashMap::new();
    let mut conn = Connection::new(socket);

    while let Some(frame) = conn.read_frame().await.unwrap() {
        let frame = match Command::from_frame(frame).unwrap() {
            Command::Set(cmd) => {
                {
                    let mut db = db.lock().unwrap();
                    let value = Bytes::copy_from_slice(cmd.value());
                    db.insert(cmd.key().to_string(), value);
                }
                Frame::Simple("OK".to_string())
            }
            Command::Get(cmd) => {
                if let Some(value) = db.lock().unwrap().get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            _ => Frame::Error("not implemented".to_string()),
        };

        let _ = conn.write_frame(&frame).await;
    }
}
