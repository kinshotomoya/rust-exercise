use tokio::io;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()>{

    fn convert_u8_to_str(buffer: &[u8]) {
        match std::str::from_utf8(&buffer) {
            Ok(value) => println!("{}", value),
            Err(e) => println!("error")
        }
    }

    {
        let mut file = File::open("aaa.txt").await?;
        let mut buffer = [0; 10];
        let n = file.read(&mut buffer[..]).await?;
        convert_u8_to_str(&buffer);
    }

    {
        let mut file = File::open("aaa.txt").await?;
        let mut buffer = Vec::new();
        let n = file.read_to_end(&mut buffer).await?;
        convert_u8_to_str(&buffer);
    }

    {
        let mut file = File::create("file.txt").await?;
        file.write(b"sddcddcdc").await;

    }

    Ok(())
}
