#[derive(Debug)]
pub enum MarketDataKind {
    Trade(String),
    OrderBookL1(String),
    OrderBook(String),
    Candle(String),
    Liquidation(String),
}
