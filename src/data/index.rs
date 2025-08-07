use derive_more::Constructor;

use crate::data::common::{
    Asset,
    AssetIndex,
    AssetMode,
    Exchange,
    ExchangeIndex,
    ExchangeMode,
    Instrument,
    InstrumentIndex,
    MultiAssetsMode,
    MultiExchangeMode,
    SingleAssetMode,
    SingleExchangeMode,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Constructor)]
pub struct IndexSlot<K, V> {
    index: K,
    value: V,
}

pub trait Indexer<EM: ExchangeMode, AM: AssetMode> {
    type Exchange: Exchange;
    type Asset: Asset;
    type Instrument: Instrument;
    type UnindexedInstrument: Instrument;

    type IndexerBuilder: IndexerBuilder<EM, AM>;

    fn new<Iter, I>(instruments: Iter) -> Self
    where
        Iter: IntoIterator<Item = I>,
        I: Into<Self::UnindexedInstrument>;

    fn builder() -> Self::IndexerBuilder {
        Self::IndexerBuilder::default()
    }

    fn instruments(&self) -> &[IndexSlot<InstrumentIndex, Self::Instrument>];
}

pub trait IndexerBuilder<EM: ExchangeMode, AM: AssetMode>: Default {
    type UnindexedInstrument: Instrument;

    type Indexer: Indexer<EM, AM>;

    fn new() -> Self {
        Self::default()
    }

    fn add_instrument(self, instrument: Self::UnindexedInstrument) -> Self;
    fn build(self) -> Self::Indexer;
}

pub trait IndexerSESA: Indexer<SingleExchangeMode, SingleAssetMode> {
    fn exchange(&self) -> &Self::Exchange;
    fn asset(&self) -> &Self::Asset;
}

pub trait IndexerSEMA: Indexer<SingleExchangeMode, MultiAssetsMode> {
    fn exchange(&self) -> &Self::Exchange;
    fn assets(&self) -> &[IndexSlot<AssetIndex, Self::Asset>];
}

pub trait IndexerMESA: Indexer<MultiExchangeMode, SingleAssetMode> {
    fn exchanges(&self) -> &[IndexSlot<ExchangeIndex, Self::Exchange>];
    fn asset(&self) -> &Self::Asset;
}

pub trait IndexerMEMA: Indexer<MultiExchangeMode, MultiAssetsMode> {
    fn exchanges(&self) -> &[IndexSlot<ExchangeIndex, Self::Exchange>];
    fn assets(&self) -> &[IndexSlot<AssetIndex, Self::Asset>];
}
