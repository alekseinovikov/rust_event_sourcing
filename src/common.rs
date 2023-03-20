use async_trait::async_trait;

#[async_trait]
pub trait Aggregate<C, E> {
    async fn handle(&self, command: C) -> Result<Vec<E>, CommandError>;
    async fn apply(&self, event: E) -> Result<Self, EventError>
    where
        Self: Sized;
}

pub enum CommandError {
    AggregateError,
    EventError,
}

pub enum EventError {
    AggregateError,
}