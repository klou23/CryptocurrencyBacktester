use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time;
use crate::strategy::Strategy;

pub struct DoNothing{}

impl Strategy for DoNothing {
    fn run(&mut self, tx: &Sender<(u8, String)>) -> ! {
        loop {
            sleep(time::Duration::from_millis(100));
        }
    }
}