use crate::entities::actor::AsyncActor;
use tokio::sync::mpsc::{self, Sender};

pub mod game;

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
