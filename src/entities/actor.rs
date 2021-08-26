use anyhow::Result;
use async_trait::async_trait;
pub trait Actor<T> {
    type Output;

    fn handle(&mut self, cmd: T) -> Result<Self::Output>;
}

#[async_trait]
pub trait AsyncActor<T> {
    type Output;

    async fn handle(&mut self, cmd: T) -> Result<Self::Output>;
}
