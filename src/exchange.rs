use crate::candle::Candle;
use crate::coin::Coin;
use crate::order::{FilledOrder, OpenOrder, OrderType};
use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;

pub struct Exchange {
    current_candles: HashMap<Coin, Candle>,
    open_orders: Vec<OpenOrder>,
    cash: f64,
    long_positions: HashMap<Coin, f64>,
    short_positions: HashMap<Coin, f64>,
    nlv: f64,
    curr_time: u32,
    end_time: u32,
    market_data: HashMap<Coin, HashMap<u32, Candle>>,
    subscriptions: HashSet<Coin>
}

impl Exchange {
    pub fn new(start_cash: f64, start_time: u32, end_time: u32) -> Self {
        Self {
            current_candles: HashMap::new(),
            open_orders: Vec::new(),
            cash: start_cash,
            long_positions: HashMap::new(),
            short_positions: HashMap::new(),
            nlv: start_cash,
            curr_time: start_time,
            end_time,
            market_data: HashMap::new(),
            subscriptions: HashSet::new()
        }
    }

    pub fn add_subscription(&mut self, coins: &[Coin]) {
        for coin in coins {
            self.subscriptions.insert((*coin).clone());
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

    /// Increments time and updates the values of current_candles
    pub fn increment_time(&mut self) {
        self.curr_time += 60;
        for c in Coin::iter() {
            if let Some(coin_candles) = self.market_data.get(&c) {
                if let Some(candle) = coin_candles.get(&self.curr_time) {
                    self.current_candles.insert(c, (*candle).clone());
                }
            }
        }
    }

    /// Iterates through open_orders and attempts to fill them
    pub fn attempt_orders(&mut self) -> Vec<FilledOrder> {
        let mut new_open_orders: Vec<OpenOrder> = Vec::new();
        let mut filled_orders: Vec<FilledOrder> = Vec::new();
        for order in self.open_orders.clone() {
            let (fill, reject) = match order.order_type {
                OrderType::BTO => self.attempt_bto(&order),
                OrderType::STC => self.attempt_stc(&order),
                OrderType::STO => self.attempt_sto(&order),
                OrderType::BTC => self.attempt_btc(&order),
            };
            if reject {
                continue;
            }
            match fill {
                None => {new_open_orders.push(order)}
                Some(fill) => {filled_orders.push(fill)}
            }
        }
        filled_orders
    }

    /// Attempts to fill a BTO order. If filled, will return (Some, false)
    /// Returns (None, true) if order cancelled
    /// Returns (None, false) otherwise
    fn attempt_bto(&mut self, order: &OpenOrder) -> (Option<FilledOrder>, bool) {
        if order.order_type != OrderType::BTO {
            // Wrong order type
            return (None, false);
        }
        let candle;
        if let Some(c) = self.current_candles.get(&order.symbol) {
            candle = c;
        } else {
            // No market data, can't fill order
            return (None, false);
        }

        let limit = match order.limit {
            None => {candle.open}
            Some(limit) => {limit}
        };

        if limit < candle.low {
            // Can't fill
            return (None, false);
        }

        let fill_price = f64::min(candle.high, limit);
        if self.cash < fill_price * order.quantity {
            // Not enough money, cancel order
            return (None, true);
        }

        // Order filled
        match self.long_positions.get(&order.symbol) {
            None => {self.long_positions.insert(order.symbol.clone(), order.quantity);}
            Some(curr) => {self.long_positions.insert(order.symbol.clone(), curr+order.quantity);}
        }
        self.cash -= fill_price * order.quantity;
        (Some(FilledOrder::new(order, fill_price)), false)
    }

    fn attempt_stc(&mut self, order: &OpenOrder) -> (Option<FilledOrder>, bool) {
        if order.order_type != OrderType::STC {
            // Wrong order type
            return (None, false);
        }
        let candle;
        if let Some(c) = self.current_candles.get(&order.symbol) {
            candle = c;
        } else {
            // No market data, can't fill order
            return (None, false);
        }

        let limit = match order.limit {
            None => {candle.open}
            Some(limit) => {limit}
        };

        if limit > candle.high {
            // Can't fill
            return (None, false);
        }

        let fill_price = f64::max(candle.low, limit);
        let long_position = self.long_positions.get(&order.symbol).cloned().unwrap_or(0f64);
        if long_position < order.quantity {
            // Not enough owned to STC
            return (None, true);
        }

        // Order filled
        match self.long_positions.get(&order.symbol) {
            None => {return (None, true);}
            Some(curr) => {self.long_positions.insert(order.symbol.clone(), curr-order.quantity);}
        }
        self.cash += fill_price * order.quantity;
        (Some(FilledOrder::new(order, fill_price)), false)
    }

    fn attempt_sto(&self, order: &OpenOrder) -> (Option<FilledOrder>, bool) {
        (None, true)
    }

    fn attempt_btc(&self, order: &OpenOrder) -> (Option<FilledOrder>, bool) {
        (None, true)
    }

    pub fn compute_nlv(&mut self) {
        self.nlv = self.cash;
        for c in Coin::iter() {
            if let (Some(quantity), Some(candle)) =
                (self.long_positions.get(&c), self.current_candles.get(&c))
            {
                self.nlv += quantity * candle.close;
            }
            if let (Some(quantity), Some(candle)) =
                (self.short_positions.get(&c), self.current_candles.get(&c))
            {
                self.nlv -= quantity * candle.close;
            }
        }
    }
}
