use derive_more::Constructor;

use crate::data::common::{Asset, AssetIndex, Exchange, ExchangeIndex, Instrument, InstrumentIndex};

pub trait Indexer {
    type Exchange: Exchange;
    type Asset: Asset;
    type Instrument: Instrument;
    type UnindexedInstrument: Instrument;

    type IndexerBuilder: IndexerBuilder;

    fn new<Iter, I>(instruments: Iter) -> Self
    where
        Iter: IntoIterator<Item = I>,
        I: Into<Self::UnindexedInstrument>;

    fn builder() -> Self::IndexerBuilder {
        Self::IndexerBuilder::default()
    }

    fn exchanges(&self) -> &[IndexSlot<ExchangeIndex, Self::Exchange>];

    fn assets(&self) -> &[IndexSlot<AssetIndex, Self::Asset>];

    fn instruments(&self) -> &[IndexSlot<InstrumentIndex, Self::Instrument>];
}

pub trait IndexerBuilder: Default {
    type UnindexedInstrument: Instrument;

    type Indexer: Indexer;

    fn new() -> Self {
        Self::default()
    }

    fn add_instrument(self, instrument: Self::UnindexedInstrument) -> Self;
    fn build(self) -> Self::Indexer;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Constructor)]
pub struct IndexSlot<K, V> {
    index: K,
    value: V,
}
