use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;

pub mod sma_strategy;

pub trait Strategy {
    /// Does all work before the strategy begins running (market data subscriptions, etc)
    fn init(&mut self, tx: &mpsc::Sender<(u8, String)>);

    /// Handles a single message from the exchange
    fn handle_message(&mut self, channel: u8, message: String);

    /// Processes all messages in the receiver channel
    fn process_messages(&mut self, rx: &mpsc::Receiver<(u8, String)>) {
        loop {
            match rx.try_recv() {
                Ok((channel, message)) => self.handle_message(channel, message),
                Err(_) => {
                    break;
                }
            }
        }
    }

    /// Computes strategy and sends orders through tx
    fn compute(&mut self, tx: &mpsc::Sender<(u8, String)>);
}
