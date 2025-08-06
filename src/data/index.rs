use derive_more::Constructor;

use crate::data::common::{Asset, Exchange, Instrument};

pub trait Indexer {
    type Exchange: Exchange;
    type Asset: Asset;
    type Instrument: Instrument;

    type IndexerBuilder: IndexerBuilder;

    fn builder() -> Self::IndexerBuilder {
        Self::IndexerBuilder::default()
    }
}

pub trait IndexerBuilder: Default {
    type Indexer: Indexer;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Constructor)]
pub struct IndexSlot<K, V> {
    index: K,
    value: V,
}
