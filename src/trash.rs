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


struct Hub {}

pub trait SvcA {}
impl SvcA for Hub {}

pub trait IsSvcA {
    fn a(&self) -> String;
}
impl<T: SvcA> IsSvcA for T {
    fn a(&self) -> String {
        "svc-a".to_string()
    }
}

pub trait HaveSvcA {
    type A: IsSvcA;
    fn get_svc_a(&self) -> &Self::A;
}
impl HaveSvcA for Hub {
    type A = Self;
    fn get_svc_a(&self) -> &Self::A {
        &self
    }
}
