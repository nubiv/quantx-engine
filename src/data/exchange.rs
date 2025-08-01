#[derive(Debug)]
pub struct ExchangeIndex(usize);

impl ExchangeIndex {
    pub fn index(&self) -> usize {
        self.0
    }
}

impl std::fmt::Display for ExchangeIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExchangeIndex({})", self.0)
    }
}

pub trait ExchangeId {}

#[derive(Debug)]
pub enum ExchangeIdFuturesCN {
    SHFE,
    INE,
    DCE,
    CZCE,
    CFFEX,
    Unknown,
}

impl ExchangeId for ExchangeIdFuturesCN {}

#[derive(Debug)]
pub enum ExchangeIdCrypto {
    Bybit,
    Binance,
    Okx,
    Coinbase,
    Kraken,
    Unknown,
}

impl ExchangeId for ExchangeIdCrypto {}
