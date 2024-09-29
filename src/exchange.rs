use std::collections::HashMap;
use strum::IntoEnumIterator;
use crate::candle::Candle;
use crate::coin::Coin;
use crate::order::{FilledOrder, OpenOrder, OrderType};

pub struct Exchange {
    current_candles: HashMap<Coin, Candle>,
    open_orders: Vec<OpenOrder>,
    cash: f64,
    positions: HashMap<Coin, f64>,
    nlv: f64,
    curr_time: u32,
    end_time: u32,
    market_data: HashMap<Coin, HashMap<u32, Candle>>,
}

impl Exchange {
    pub fn new(start_cash: f64, start_time: u32, end_time: u32) -> Self {
        Self {
            current_candles: HashMap::new(),
            open_orders: Vec::new(),
            cash: start_cash,
            positions: HashMap::new(),
            nlv: start_cash,
            curr_time: start_time,
            end_time,
            market_data: HashMap::new()
        }
    }

    /// Sends market data over websocket
    pub fn send_market_data(&self) {
        //TODO
    }

    /// Polls market data from the queue and appends it to open_orders
    pub fn poll_order_queue(&self) {
        //TODO
    }

    pub fn increment_time(&mut self) {
        self.curr_time += 60;
        for c in Coin::iter() {
            let candle: Option<Candle> = self.market_data.get(&c).get(self.curr_time);
            if let Some(candle) = candle {
                self.current_candles.insert(c, candle);
            }
        }
    }

    pub fn attempt_orders(&mut self) {
        let new_open_orders = Vec::new();
        for order in self.open_orders {
            let (fill, reject) = match order.order_type {
                OrderType::BTO => Self::attempt_bto(&order),
                OrderType::STC => Self::attempt_stc(&order),
                OrderType::STO => Self::attempt_sto(&order),
                OrderType::BTC => Self::attempt_btc(&order)
            };
        }
    }

    fn attempt_bto(order: &OpenOrder) -> (Option<FilledOrder>, bool) {
        (None, false)
    }

    fn attempt_stc(order: &OpenOrder) -> (Option<FilledOrder>, bool) {
        (None, false)
    }

    fn attempt_sto(order: &OpenOrder) -> (Option<FilledOrder>, bool) {
        (None, true)
    }

    fn attempt_btc(order: &OpenOrder) -> (Option<FilledOrder>, bool) {
        (None, true)
    }

    pub fn compute_nlv(&mut self) {
        self.nlv = self.cash;
        for c in Coin::iter() {
            let quantity = self.positions.get(&c);
            let candle = self.current_candles.get(&c);
            match (quantity, candle) {
                (Some(quantity), Some(candle)) => {
                    self.nlv += quantity * candle.close
                },
                _ => {}
            }
        }
    }
}

