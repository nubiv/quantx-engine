use crate::data::exchange::Exchange;

pub trait Asset {}

#[derive(Debug)]
pub struct AssetIndex(pub usize);

impl AssetIndex {
    pub fn index(&self) -> usize {
        self.0
    }
}

impl std::fmt::Display for AssetIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExchangeIndex({})", self.0)
    }
}

#[derive(Debug)]
pub enum AssetUnified {
    CNY,
    USD,
    EUR,
    JPY,
    GBP,
    AUD,
    CAD,
    CHF,
    NZD,
}

impl Asset for AssetUnified {}

#[derive(Debug)]
pub struct AssetByExchange<E, AN>
where
    E: Exchange,
{
    exchange: E,
    name: AN,
}

impl<E, AN> Asset for AssetByExchange<E, AN> where E: Exchange {}

#[derive(Debug)]
pub struct AssetName {
    name_internal: AssetNameInternal,
    name_exchange: AssetNameExchange,
}

#[derive(Debug)]
pub struct AssetNameInternal(smol_str::SmolStr);

#[derive(Debug)]
pub struct AssetNameExchange(smol_str::SmolStr);
