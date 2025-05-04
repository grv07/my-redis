use tokio::{io, net::TcpListener};

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "localhost:6363".to_string();
    let listener = TcpListener::bind(&addr).await?;

    println!("Server started at: {addr}");

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let (mut rh, mut wh) = socket.split();
            if let Ok(n) = io::copy(&mut rh, &mut wh).await {
                println!("Copy {n} bytes successfully");
            }
        });
    }

    #[allow(unreachable_code)]
    Ok(())
}
