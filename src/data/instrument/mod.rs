use derive_more::Constructor;

use crate::data::instrument::{future::FutureDetail, option::OptionDetail, perp::PerpetualDetail, spec::InstrumentSpec};

mod future;
mod option;
mod perp;
mod spec;

pub trait Instrument: PartialEq + Eq + PartialOrd + Ord {}

#[derive(Debug, Constructor)]
pub struct InstrumentIndex(usize);

impl InstrumentIndex {
    pub fn index(&self) -> usize {
        self.0
    }
}

impl std::fmt::Display for InstrumentIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExchangeIndex({})", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct InstrumentCrypto<ExchangeSlot, AssetSlot> {
    exchange: ExchangeSlot,
    name: InstrumentName,
    underlying: Underlying<AssetSlot>,
    quote: InstrumentQuoteKind,
    kind: InstrumentKind<AssetSlot>,
    spec: InstrumentSpec<AssetSlot>,
}

impl<ExchangeSlot, AssetSlot> Instrument for InstrumentCrypto<ExchangeSlot, AssetSlot>
where
    ExchangeSlot: PartialEq + Eq + PartialOrd + Ord,
    AssetSlot: PartialEq + Eq + PartialOrd + Ord,
{
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct InstrumentFuture<ExchangeSlot, AssetSlot, IN> {
    exchange: ExchangeSlot,
    name: IN,
    kind: InstrumentKind<AssetSlot>,
    spec: InstrumentSpec<AssetSlot>,
}

impl<ExchangeSlot, AssetSlot, IN> Instrument for InstrumentFuture<ExchangeSlot, AssetSlot, IN>
where
    ExchangeSlot: PartialEq + Eq + PartialOrd + Ord,
    AssetSlot: PartialEq + Eq + PartialOrd + Ord,
    IN: PartialEq + Eq + PartialOrd + Ord,
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
