use crate::coin::Coin;

pub enum OrderType {
    BTO,
    STC,
    STO,
    BTC
}

pub struct OpenOrder {
    pub order_type: OrderType,
    pub symbol: Coin,
    pub limit: Option<f64>,
    pub quantity: f64,
}

pub struct FilledOrder {
    pub order_type: OrderType,
    pub symbol: Coin,
    pub price: f64,
    pub quantity: f64,
}