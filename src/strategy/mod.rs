use std::sync::mpsc;

pub mod do_nothing;

pub trait Strategy {

    /// Run the strategy, this should be an infinite loop
    /// tx is used to send messages to the exchange, message formats are exchange dependent
    fn run(&mut self, tx: &mpsc::Sender<(u8, String)>) -> !;

}