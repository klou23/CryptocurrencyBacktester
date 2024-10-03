use crate::exchange::Exchange;
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::sync::mpsc::Sender;

#[derive(Clone, Serialize, Deserialize)]
pub struct Candle {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub symbol: String,
}

#[derive(Debug, Deserialize)]
struct KucoinMinuteCandleRow {
    pub unix: u32,
    pub date: String,
    pub symbol: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub vol1: f64,
    pub vol2: f64,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum BuySell {
    Buy,
    Sell,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OpenOrder {
    pub buy_sell: BuySell,
    pub quantity: f64,
    pub limit: Option<f64>,
    pub symbol: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FilledOrder {
    pub buy_sell: BuySell,
    pub quantity: f64,
    pub price: f64,
    pub symbol: String,
}

/// Candle-based BTC exchange, only allowing long positions
/// Market data on channel 1: Send symbol to subscribe, responses will be of type Candle
/// Orders on channel 2: Strategy should send OpenOrder type, exchange will send FilledOrder back
/// if order gets filled
pub struct LongOnlyBtcCandleExchange {
    pub market_data: HashMap<u32, Candle>,
    pub subscribed: bool,
    pub curr_candle: Option<Candle>,
    pub position: f64,
    pub cash: f64,
    pub curr_time: u32,
    pub end_time: u32,
    pub open_orders: Vec<OpenOrder>,
}

impl LongOnlyBtcCandleExchange {
    pub fn new(cash: f64) -> Self {
        Self {
            market_data: HashMap::new(),
            subscribed: false,
            curr_candle: None,
            position: 0.0,
            cash,
            curr_time: u32::MAX,
            end_time: u32::MIN,
            open_orders: Vec::new(),
        }
    }
}

impl Exchange for LongOnlyBtcCandleExchange {
    fn init(&mut self) {
        let file_path = "Data/Kucoin_BTCUSDT_2019_minute.csv".to_string();
        let file = File::open(file_path).expect("Failed to open file");
        let mut rdr = ReaderBuilder::new().from_reader(file);

        for result in rdr.deserialize() {
            let row: KucoinMinuteCandleRow = result.expect("Failed to deserialize csv");
            self.market_data.insert(
                row.unix,
                Candle {
                    open: row.open,
                    high: row.high,
                    low: row.low,
                    close: row.close,
                    symbol: "BTC".to_string(),
                },
            );
            self.curr_time = u32::min(self.curr_time, row.unix);
            self.end_time = u32::max(self.end_time, row.unix);
        }
    }

    fn handle_message(&mut self, channel: u8, message: String, tx: &Sender<(u8, String)>) {
        if (channel == 1) {
            // subscription channel
            if (message == "BTC") {
                self.subscribed = true;
            }
        } else if (channel == 2) {
            // order entry
            if let Ok(order) = serde_json::from_str(&message) {
                self.open_orders.push(order);
            }
        }
    }

    fn can_update(&self) -> bool {
        self.curr_time < self.end_time
    }

    fn update(&mut self, tx: &Sender<(u8, String)>) {
        self.curr_time += 60;
        self.curr_candle = self.market_data.get(&self.curr_time).cloned();
        self.attempt_orders(tx);
        if let Some(candle) = &self.curr_candle {
            if self.subscribed {
                let json = serde_json::to_string(candle).expect("Serialization Error");
                tx.send((1, json)).expect("Send error");
            }
        }
    }
}

impl LongOnlyBtcCandleExchange {
    /// Iterates through open_orders and attempts to fill them
    /// Sends all filled orders to strategy using tx
    pub fn attempt_orders(&mut self, tx: &Sender<(u8, String)>) {
        let mut new_open_orders: Vec<OpenOrder> = Vec::new();
        if let Some(candle) = &self.curr_candle {
            for order in self.open_orders.clone() {
                match order.buy_sell {
                    BuySell::Buy => {
                        let limit = match order.limit {
                            None => candle.open,
                            Some(limit) => limit,
                        };
                        if limit < candle.low {
                            // Can't fill
                            new_open_orders.push(order);
                            continue;
                        }
                        let fill_price = f64::min(candle.open, limit);
                        if self.cash < fill_price * order.quantity {
                            // Not enough cash to buy
                            continue;
                        }
                        self.position += order.quantity;
                        self.cash -= fill_price * order.quantity;
                        let filled_order = FilledOrder {
                            buy_sell: BuySell::Buy,
                            quantity: order.quantity,
                            price: fill_price,
                            symbol: "BTC".to_string(),
                        };
                        let json = serde_json::to_string(&filled_order).expect("JSON failed");

                        tx.send((2, json)).expect("Message send failed");
                    }
                    BuySell::Sell => {
                        let limit = match order.limit {
                            None => candle.open,
                            Some(limit) => limit,
                        };
                        if limit > candle.high {
                            // Can't fill
                            new_open_orders.push(order);
                            continue;
                        }
                        let fill_price = f64::max(candle.open, limit);
                        if self.position < order.quantity {
                            // Not enough owned to sell
                            continue;
                        }
                        self.position -= order.quantity;
                        self.cash += fill_price * order.quantity;
                        let filled_order = FilledOrder {
                            buy_sell: BuySell::Sell,
                            quantity: order.quantity,
                            price: fill_price,
                            symbol: "BTC".to_string(),
                        };
                        let json = serde_json::to_string(&filled_order).expect("JSON failed");
                        tx.send((2, json)).expect("Message send failed");
                    }
                }
            }
        }
    }
}
