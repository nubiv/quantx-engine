use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use crate::data::{common::Instrument, crypto::instrument::spec::InstrumentSpecCrypto};

mod spec;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct InstrumentCrypto<ExchangeSlot, AssetSlot> {
    exchange: ExchangeSlot,
    name: InstrumentName,
    underlying: Underlying<AssetSlot>,
    quote: InstrumentQuoteKind,
    kind: InstrumentKind<AssetSlot>,
    spec: InstrumentSpecCrypto<AssetSlot>,
}

impl<ExchangeSlot, AssetSlot> Instrument for InstrumentCrypto<ExchangeSlot, AssetSlot>
where
    ExchangeSlot: PartialEq + Eq + PartialOrd + Ord,
    AssetSlot: PartialEq + Eq + PartialOrd + Ord,
{
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct InstrumentName {
    name_internal: InstrumentNameInternal,
    name_exchange: InstrumentNameExchange,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct InstrumentNameInternal(smol_str::SmolStr);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct InstrumentNameExchange(smol_str::SmolStr);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Underlying<AssetSlot> {
    pub base: AssetSlot,
    pub quote: AssetSlot,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum InstrumentQuoteKind {
    UnderlyingBase,
    UnderlyingQuote,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum InstrumentKind<AssetSlot> {
    Spot,
    Perpetual(PerpetualDetail<AssetSlot>),
    Future(FutureDetail<AssetSlot>),
    Option(OptionDetail<AssetSlot>),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PerpetualDetail<AssetSlot> {
    contract_size: Decimal,
    settlement_asset: AssetSlot,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FutureDetail<AssetSlot> {
    contract_size: Decimal,
    settlement_asset: AssetSlot,
    expiry: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OptionDetail<AssetSlot> {
    contract_size: Decimal,
    settlement_asset: AssetSlot,
    kind: String,
    exercise: String,
    expiry: DateTime<Utc>,
    strike: Decimal,
}
