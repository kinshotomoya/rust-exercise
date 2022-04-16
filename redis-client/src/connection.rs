use std::io::Cursor;
use bytes::{Buf, BytesMut};
use mini_redis::{Frame, Result};
use mini_redis::frame::Error::Incomplete;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;

#[derive(Debug)]
struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut
}

impl Connection {

    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(stream),
            buffer: BytesMut::with_capacity(4096)
        }
    }


    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame))
            }

            // parseするにはまだbufferが足りない場合には、さらにstreamから読み込む
            // ↓読み込み終わるまで待つ
            // 0は読み込み終了ということ

            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                return if self.buffer.is_empty() {
                    Ok(None)
                } else {
                    Err("コネクションはリセットされた".into())
                }

            }
        }
    }

    fn parse_frame(&mut self) -> Result<Option<Frame>> {
        // NOTE: Cursorはバイト列を便利に扱うための構造体
        let mut buf = Cursor::new(&self.buffer[..]);

        match Frame::check(&mut buf) {
            Ok(_) => {
                // ↓こんな感じにでバイト配列のポジションを動かしたり、現在のポジションを取得したり便利にできる
                // Frameを認識できるほどの十分な情報がバッファされているなら
                let size = buf.position() as usize;
                // bufferの先頭にポジションを戻す
                buf.set_position(0);
                let frame = Frame::parse(&mut buf)?;
                // bufferからframe分のバイト列を削除する
                self.buffer.advance(size);
                Ok(Some(frame))

            },
            Err(Incomplete) => Ok(None),
            Err(e) => Err(e.into())
        }


    }

    pub async fn write_frame(&mut self, frame: &Frame) -> std::io::Result<()> {
        match frame {
            // 複数のFrameの場合
            Frame::Array(val) => {
                self.stream.write_u8(b'*').await?;
                self.write_decimal(val.len() as u64).await?;

                for entry in val {
                    self.write_value(entry).await?
                }
            },
            _ => self.write_value(frame).await?
        }
        self.stream.flush().await
    }

    async fn write_value(&mut self, frame: &Frame) -> std::io::Result<()> {
        // コピーした
        match frame {
            Frame::Simple(val) => {
                self.stream.write_u8(b'+').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Error(val) => {
                self.stream.write_u8(b'-').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Integer(val) => {
                self.stream.write_u8(b':').await?;
                self.write_decimal(*val).await?;
            }
            Frame::Null => {
                self.stream.write_all(b"$-1\r\n").await?;
            }
            Frame::Bulk(val) => {
                let len = val.len();

                self.stream.write_u8(b'$').await?;
                self.write_decimal(len as u64).await?;
                self.stream.write_all(val).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Array(_val) => unreachable!(),
        }

        Ok(())
    }

    async fn write_decimal(&mut self, val: u64) -> std::io::Result<()> {
        // コピーした
        use std::io::Write;

        // Convert the value to a string
        let mut buf = [0u8; 12];
        let mut buf = Cursor::new(&mut buf[..]);
        write!(&mut buf, "{}", val)?;

        let pos = buf.position() as usize;
        self.stream.write_all(&buf.get_ref()[..pos]).await?;
        self.stream.write_all(b"\r\n").await?;

        Ok(())
    }
}
