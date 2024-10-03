use std::sync::mpsc;

pub mod long_only_btc_candle_exchange;

/// Exchanges act like a fake exchange while backtesting
/// They iterate over time-series data and handle messages from strategies
/// An exchange is designed to only serve a single client (the strategy)
/// Numbered message channels can be used to represent different websockets/requests
pub trait Exchange {

    /// Initializes exchange from data source
    fn init(&mut self);

    /// Handles a single message from the client
    fn handle_message(&mut self, channel: u8, message: String);

    /// Processes all messages in the channel
    fn process_messages(&mut self, rx: &mpsc::Receiver<(u8, String)>) {
        loop {
            match rx.try_recv() {
                Ok((channel, message)) => {self.handle_message(channel, message)}
                Err(_) => {break;}
            }
        }
    }

    /// Indicates if the exchange has more time-series data
    fn can_update(&self) -> bool;

    /// Updates the exchange to the next time step and all necessary work
    fn update(&mut self);

}