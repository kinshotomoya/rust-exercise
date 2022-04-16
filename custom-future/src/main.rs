use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_secs(10);
    let delay = Delay {
        when
    };

    // Poll::Readyになった時点で処理が終わる
    let out = delay.await;
    println!("{}", out)

}

struct Delay {
    when: Instant
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("hogehogehoge");
            Poll::Ready("done")
        } else {
            // ?????
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
