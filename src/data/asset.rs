use derive_more::Constructor;

use crate::data::exchange::Exchange;

pub trait Asset: PartialEq + Eq + PartialOrd + Ord {}

#[derive(Debug, Constructor)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AssetUnified(smol_str::SmolStr);

impl Asset for AssetUnified {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AssetCentralized {
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

impl Asset for AssetCentralized {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AssetByExchange<E, AN>
where
    E: Exchange,
    AN: PartialEq + Eq + PartialOrd + Ord,
{
    exchange: E,
    name: AN,
}

impl<E, AN> Asset for AssetByExchange<E, AN>
where
    E: Exchange,
    AN: PartialEq + Eq + PartialOrd + Ord,
{
}

#[derive(Debug)]
pub struct AssetName {
    name_internal: AssetNameInternal,
    name_exchange: AssetNameExchange,
}

#[derive(Debug)]
pub struct AssetNameInternal(smol_str::SmolStr);

#[derive(Debug)]
pub struct AssetNameExchange(smol_str::SmolStr);
