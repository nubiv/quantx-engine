use crate::data::instrument::{future::FutureDetail, option::OptionDetail, perp::PerpetualDetail, spec::InstrumentSpec};

mod future;
mod option;
mod perp;
mod spec;

pub trait Instrument {}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct InstrumentCrypto<ExchangeSlot, AssetSlot> {
    exchange: ExchangeSlot,
    name: InstrumentName,
    underlying: Underlying<AssetSlot>,
    quote: InstrumentQuoteKind,
    kind: InstrumentKind<AssetSlot>,
    spec: InstrumentSpec<AssetSlot>,
}

impl<ExchangeSlot, AssetSlot> Instrument for InstrumentCrypto<ExchangeSlot, AssetSlot> {}

#[derive(Debug)]
pub struct InstrumentFuture<ExchangeSlot, AssetSlot, IN> {
    exchange: ExchangeSlot,
    name: IN,
    underlying: Underlying<AssetSlot>,
    kind: InstrumentKind<AssetSlot>,
    spec: InstrumentSpec<AssetSlot>,
}

impl<ExchangeSlot, AssetSlot, IN> Instrument for InstrumentFuture<ExchangeSlot, AssetSlot, IN> {}

#[derive(Debug)]
pub struct InstrumentName {
    name_internal: InstrumentNameInternal,
    name_exchange: InstrumentNameExchange,
}

#[derive(Debug)]
pub struct InstrumentNameInternal(smol_str::SmolStr);

#[derive(Debug)]
pub struct InstrumentNameExchange(smol_str::SmolStr);

#[derive(Debug)]
pub struct Underlying<AssetSlot> {
    pub base: AssetSlot,
    pub quote: AssetSlot,
}

#[derive(Debug)]
pub enum InstrumentQuoteKind {
    UnderlyingBase,
    UnderlyingQuote,
}

#[derive(Debug)]
pub enum InstrumentKind<AssetSlot> {
    Spot,
    Perpetual(PerpetualDetail<AssetSlot>),
    Future(FutureDetail<AssetSlot>),
    Option(OptionDetail<AssetSlot>),
}
