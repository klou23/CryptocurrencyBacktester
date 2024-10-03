use crate::exchange::long_only_btc_candle_exchange::Candle;
use crate::strategy::Strategy;
use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time;

pub struct SmaStrategy {}

impl Strategy for SmaStrategy {
    fn init(&mut self, tx: &Sender<(u8, String)>) {
        tx.send((1, "BTC".to_string())).expect("Send error");
    }

    fn handle_message(&mut self, channel: u8, message: String) {
        if (channel == 1) {
            let candle: Candle = serde_json::from_str(&message).expect("Deserialize error");
            println!("BTC: {}", candle.close);
        }
    }

    fn compute(&mut self, tx: &Sender<(u8, String)>) {
        // nothing
    }
}
