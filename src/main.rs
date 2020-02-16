use std::fs;
use std::path::Path;

use futures::StreamExt;
use tokio;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::{UnixDatagram, UnixListener, UnixStream};

async fn handle_client(mut stream: UnixStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = [0; 1024];

    let n = stream.read(&mut buf).await?;
    let s = String::from_utf8_lossy(&buf[..n]);
    println!("{}", s);

    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let sock_path = Path::new("/var/run/docker.sock");
    let mut client = UnixStream::connect(&sock_path).await.unwrap();
    let request = format!("GET /info HTTP/1.1\r\nContent-Type: application/json\r\nHost: localhost\r\n\r\n");

    client.write_all(request.as_bytes()).await?;
    
    const BUFFER_SIZE: usize = 3000;
    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut raw: Vec<u8> = Vec::new();
    let mut ret;
    loop {
        ret = client.read(&mut buffer).await.map(|len|{
            println!("{}", len);
            for i in 0..len { raw.push(buffer[i]); }
        });
        if let Err(ref err) = ret {
            if err.kind() == std::io::ErrorKind::WouldBlock {
                break;
            }
            panic!("{:?}", err);
        }
        if ret.is_ok() { break }
    }
    let ret = std::str::from_utf8(&raw).unwrap();
    dbg!(&ret);

    Ok(())
}

#[tokio::main]
async fn unix_socket_server() -> Result<(), Box<dyn std::error::Error>> {
    let sockfile = Path::new("/tmp/uds.sock");
    if sockfile.exists() {
        fs::remove_file(&sockfile)?;
    }

    let mut listner = UnixListener::bind(sockfile)?;
    let mut incoming = listner.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        tokio::spawn(async move {
            handle_client(stream).await.unwrap();
        });
    }

    Ok(())
}