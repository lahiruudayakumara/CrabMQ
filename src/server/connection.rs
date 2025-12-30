use crate::errors::Result;
use bytes::BytesMut;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

pub async fn handle(mut stream: TcpStream) -> Result<()> {
    let mut buf = BytesMut::with_capacity(1024);
    loop {
        let mut temp = [0u8; 256];
        let n = stream.read(&mut temp).await?;
        if n == 0 { break; }
        buf.extend_from_slice(&temp[..n]);

        // Echo back as simple placeholder
        stream.write_all(&temp[..n]).await?;
    }
    Ok(())
}
