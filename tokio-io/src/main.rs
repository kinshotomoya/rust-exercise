use std::fs::read_to_string;
use tokio::io;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:8000").await?;

    loop {
        let (mut tcpStream, socket) = listener.accept().await?;

        tokio::spawn(async move {
            // メモリ上にバッファを確保する
            let mut buffer = vec![0; 1];
            // socketからバイト読み込む
            match tcpStream.read(&mut buffer).await {
                Ok(0) => return,
                Ok(v) => {
                    println!("{:?}", buffer);
                    // socketに書き出す
                    if tcpStream.write_all(&buffer).await.is_err() {
                        return;
                    }
                },
                Err(e) => {
                    println!("error: {:?}", e);
                    return;
                }
            }


        });
    }


    // fn convert_u8_to_str(buffer: &[u8]) {
    //     match std::str::from_utf8(&buffer) {
    //         Ok(value) => println!("{}", value),
    //         Err(e) => println!("error")
    //     }
    // }
    //
    // {
    //     let mut file = File::open("aaa.txt").await?;
    //     let mut buffer = [0; 10];
    //     let n = file.read(&mut buffer[..]).await?;
    //     convert_u8_to_str(&buffer);
    // }
    //
    // {
    //     let mut file = File::open("aaa.txt").await?;
    //     let mut buffer = Vec::new();
    //     let n = file.read_to_end(&mut buffer).await?;
    //     convert_u8_to_str(&buffer);
    // }
    //
    // {
    //     let mut file = File::create("file.txt").await?;
    //     file.write(b"sddcddcdc").await;
    //
    // }
    //
    // Ok(())
}
