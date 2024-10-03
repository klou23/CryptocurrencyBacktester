use std::sync::mpsc;
use std::thread;
use crate::exchange::Exchange;
use crate::exchange::long_only_btc_candle_exchange::LongOnlyBtcCandleExchange;
use crate::strategy::do_nothing::DoNothing;
use crate::strategy::Strategy;

pub mod exchange;
pub mod strategy;

fn run_do_nothing_on_long_only_btc_candle() {
    let (tx, rx) = mpsc::channel::<(u8, String)>();
    let mut exchange = LongOnlyBtcCandleExchange{};
    exchange.init();
    thread::spawn(move || {
        let mut strategy = DoNothing{};
        strategy.run(&tx);
    });
    while exchange.can_update() {
        exchange.process_messages(&rx);
        exchange.update();
    }
}

fn main() {
    run_do_nothing_on_long_only_btc_candle();
}
