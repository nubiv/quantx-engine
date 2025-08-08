use crate::{crypto::data::exchange::ExchangeIdCrypto, data::common::Asset};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AssetByExchange<A> {
    exchange: ExchangeIdCrypto,
    asset: A,
}

impl<A> Asset for AssetByExchange<A> where A: PartialEq + Eq + PartialOrd + Ord {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AssetCrypto {
    name_internal: AssetNameInternal,
    name_exchange: AssetNameExchange,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AssetNameInternal(smol_str::SmolStr);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AssetNameExchange(smol_str::SmolStr);
