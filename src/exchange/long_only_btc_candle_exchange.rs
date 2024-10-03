use crate::exchange::Exchange;

pub struct LongOnlyBtcCandleExchange {
}

impl Exchange for LongOnlyBtcCandleExchange {
    fn init(&mut self) {
        todo!()
    }

    fn handle_message(&mut self, channel: u8, message: String) {
        todo!()
    }

    fn can_update(&self) -> bool {
        true
    }

    fn update(&mut self) {
        todo!()
    }
}