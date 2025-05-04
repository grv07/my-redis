use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "localhost:6363".to_string();
    let socket = TcpStream::connect(addr).await?;

    let (mut rh, mut wh) = io::split(socket);

    tokio::spawn(async move {
        wh.write_all(b"hello\r\n").await?;
        wh.write_all(b"world\r\n").await?;

        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0; 10];

    loop {
        let n = rh.read(&mut buf).await?;

        if n == 0 {
            break;
        }

        let msg = String::from_utf8(buf[..n].to_vec());
        println!("GOT: {:?}", msg);
    }

    Ok(())
}
