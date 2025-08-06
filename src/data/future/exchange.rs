use crate::data::common::Exchange;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExchangeIdFuture {
    SHFE,
    INE,
    DCE,
    CZCE,
    CFFEX,
    Unknown,
}

impl Exchange for ExchangeIdFuture {}
