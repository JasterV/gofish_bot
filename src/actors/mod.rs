pub mod game;

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc::{self, Sender};

#[async_trait]
pub trait AsyncActor<T> {
    type Output;

    async fn handle(&mut self, cmd: T) -> Result<Self::Output>;
}

pub fn run_async_actor<T, E>(mut actor: E) -> Sender<T>
where
    T: 'static + Send,
    E: 'static + AsyncActor<T> + Send,
{
    let (tx, mut rx) = mpsc::channel(32);
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let _ = actor.handle(msg).await;
        }
    });
    tx
}
