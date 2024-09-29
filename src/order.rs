use crate::coin::Coin;

#[derive(PartialEq, Eq, Clone)]
pub enum OrderType {
    BTO,
    STC,
    STO,
    BTC,
}

#[derive(Clone)]
pub struct OpenOrder {
    pub order_type: OrderType,
    pub symbol: Coin,
    pub limit: Option<f64>,
    pub quantity: f64,
}

#[derive(Clone)]
pub struct FilledOrder {
    pub order_type: OrderType,
    pub symbol: Coin,
    pub price: f64,
    pub quantity: f64,
}

impl FilledOrder {
    pub fn new(order: &OpenOrder, price: f64) -> Self {
        Self {
            order_type: order.order_type.clone(),
            symbol: order.symbol.clone(),
            price,
            quantity: order.quantity
        }
    }
}