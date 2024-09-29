use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(strum_macros::Display, EnumIter, PartialEq, Eq, Hash)]
pub enum Coin {
    ADA,
    BCH,
    BNB,
    BTC,
    DASH,
    EOS,
    ETH,
    LTC,
    NEO,
    TRX,
    XRP,
    XTZ,
    ZEC
}