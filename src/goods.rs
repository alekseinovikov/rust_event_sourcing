#[derive(Debug, Clone, PartialEq)]
pub enum GoodCommand {
    Created { name: String, price: f64, quantity: i32 },
    NameUpdated { name: String },
    PriceUpdated { price: f64 },
    QuantityUpdated { quantity: i32 },
    Deleted,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GoodEvent {
    Created { name: String, price: f64, quantity: i32 },
    NameUpdated { name: String },
    PriceUpdated { price: f64 },
    QuantityUpdated { quantity: i32 },
    Deleted,
}

pub struct Good {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub quantity: i32,
}

use crate::common::{Aggregate, CommandError, EventError};
use async_trait::async_trait;
#[async_trait]
impl Aggregate<GoodCommand, GoodEvent> for Good {

    async fn handle(&self, command: GoodCommand) -> Result<Vec<GoodEvent>, CommandError> {
        match command {
            GoodCommand::Created { name, price, quantity } => {
                Ok(vec![GoodEvent::Created { name, price, quantity }])
            },
            GoodCommand::NameUpdated { name } => {
                Ok(vec![GoodEvent::NameUpdated { name }])
            },
            GoodCommand::PriceUpdated { price } => {
                Ok(vec![GoodEvent::PriceUpdated { price }])
            },
            GoodCommand::QuantityUpdated { quantity } => {
                Ok(vec![GoodEvent::QuantityUpdated { quantity }])
            },
            GoodCommand::Deleted => {
                Ok(vec![GoodEvent::Deleted])
            },
        }
    }

    async fn apply(&self, event: GoodEvent) -> Result<Self, EventError> where Self: Sized {
        match event {
            GoodEvent::Created { name, price, quantity } => {
                Ok(Good {
                    id: self.id.clone(),
                    name,
                    price,
                    quantity,
                })
            },
            GoodEvent::NameUpdated { name } => {
                Ok(Good {
                    id: self.id.clone(),
                    name,
                    price: self.price,
                    quantity: self.quantity,
                })
            },
            GoodEvent::PriceUpdated { price } => {
                Ok(Good {
                    id: self.id.clone(),
                    name: self.name.clone(),
                    price,
                    quantity: self.quantity,
                })
            },
            GoodEvent::QuantityUpdated { quantity } => {
                Ok(Good {
                    id: self.id.clone(),
                    name: self.name.clone(),
                    price: self.price,
                    quantity,
                })
            },
            GoodEvent::Deleted => {
                Err(EventError::AggregateError)
            },
        }
    }

}
