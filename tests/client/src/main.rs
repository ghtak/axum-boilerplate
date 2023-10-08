#![allow(unused)]

use std::{future::Future, time::{Instant, Duration}, task::Poll};

use tokio;
use pin_project;

#[pin_project::pin_project]
pub struct TimedWrapper<Fut: Future> {
    start: Option<Instant>,
    #[pin]
    inner: Fut,
}

impl<Fut: Future> TimedWrapper<Fut> {
    pub fn new(future: Fut) -> Self {
        Self {
            start: None,
            inner: future,
        }
    }
}

impl<Fut: Future> Future for TimedWrapper<Fut> {
    type Output = (Fut::Output, Duration);

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut this = self.project();
        let start = this.start.get_or_insert_with(Instant::now);
        let inner_poll = this.inner.as_mut().poll(cx);
        let elapsed = start.elapsed();
        match inner_poll {
            Poll::Pending => Poll::Pending,
            Poll::Ready(v) => Poll::Ready((v, elapsed)),
        }
    }
}

#[tokio::test]
async fn test_main() -> anyhow::Result<()> {
    let hc = httpc_test::new_client("http://localhost:18089")?;
    hc.do_get("/basic").await?.print().await?;
    hc.do_get("/basic/error").await?.print().await?;
    hc.do_get("/basic/state").await?.print().await?;
    let async_fn = reqwest::get("http://adamchalmers.com");
    let timed_async_fn = TimedWrapper::new(async_fn);
    print!("{:?}", timed_async_fn.await);
    Ok(())  
}


#[tokio::main]
async fn main() {}