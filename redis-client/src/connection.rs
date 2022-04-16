use std::io::Read;
use bytes::{Buf, BytesMut};
use mini_redis::{Frame, Result};
use mini_redis::frame::Error::Incomplete;
use tokio::io::{AsyncBufReadExt, AsyncReadExt};
use tokio::net::TcpStream;

struct Connection {
    stream: TcpStream,
    buffer: BytesMut
}

impl Connection {

    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream,
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
        let mut buf = std::io::Cursor::new(&self.buffer[..]);

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

    pub async fn write_frame(&self) -> Result<()> {
        todo!()
    }
}
