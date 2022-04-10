use mini_redis::{client, Result};

#[tokio::main]
// このtokio::mainはmacroになっており、ランタイムの初期化と非同期処理の実行が行われる
//
pub async fn main() -> Result<()> {
    // mini-redis アドレスへのコネクションを開く
    let mut client = client::connect("127.0.0.1:6379").await?;

    // "hello" というキーに "world" という値をセット
    client.set("hello", "world".into()).await?;

    // "hello" の値を取得
    let result = client.get("hello").await?;
    //
    println!("got value from the server; result={:?}", result);

    Ok(())
}
