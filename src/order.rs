enum OrderType {
    BTO,
    STC,
    STO,
    BTC
}

#[derive(strum_macros::Display)]
pub enum Coin {
    ADA,
    BCH,
    BNB,
    BTC,
    DASH,
    EOS,
    ETH,
    LTC,
    NEO,
    TRX,
    XRP,
    XTZ,
    ZEC
}

pub struct OpenOrder {
    order_type: OrderType,
    symbol: Coin,
    limit: Option<f64>,
    quantity: f64,
}

pub struct FilledOrder {
    order_type: OrderType,
    symbol: Coin,
    price: f64,
    quantity: f64,
}