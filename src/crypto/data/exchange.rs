use crate::data::common::Exchange;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExchangeIdCrypto {
    Bybit,
    Binance,
    Okx,
    Coinbase,
    Kraken,
    Unknown,
}

impl Exchange for ExchangeIdCrypto {}
