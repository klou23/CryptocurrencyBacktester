use crate::exchange::long_only_btc_candle_exchange::LongOnlyBtcCandleExchange;
use crate::exchange::Exchange;
use crate::strategy::sma_strategy::SmaStrategy;
use crate::strategy::Strategy;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub mod exchange;
pub mod strategy;

fn run_sma_on_long_only_btc_candle() {
    let (e_to_s_tx, e_to_s_rx) = mpsc::channel::<(u8, String)>();
    let (s_to_e_tx, s_to_e_rx) = mpsc::channel::<(u8, String)>();
    let (stop_tx, stop_rx) = mpsc::channel::<bool>();

    let exchange_thread = thread::spawn(move || {
        let mut exchange = LongOnlyBtcCandleExchange::new(1000.0);
        exchange.init();
        while exchange.can_update() {
            exchange.process_messages(&s_to_e_rx, &e_to_s_tx);
            exchange.update(&e_to_s_tx);
            // thread::sleep(Duration::from_millis(1));
        }
        stop_tx.send(true).expect("Send error");
    });

    let strategy_thread = thread::spawn(move || {
        let mut strategy = SmaStrategy {};
        strategy.init(&s_to_e_tx);
        loop {
            match stop_rx.try_recv() {
                Ok(_) => {
                    break;
                }
                Err(_) => {}
            }
            strategy.process_messages(&e_to_s_rx);
            strategy.compute(&s_to_e_tx);
        }
    });

    exchange_thread.join().unwrap();
    strategy_thread.join().unwrap();
}

fn main() {
    run_sma_on_long_only_btc_candle();
}
