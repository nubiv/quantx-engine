use crate::data::{
    common::{ExchangeIndex, InstrumentIndex},
    future::{asset::AssetUnified, exchange::ExchangeIdFuture, instrument::InstrumentFuture},
    index::{IndexSlot, Indexer, IndexerBuilder},
};

mod asset;
mod exchange;
mod instrument;

#[derive(Debug)]
pub struct IndexerFuture {
    exchanges: Vec<IndexSlot<ExchangeIndex, ExchangeIdFuture>>,
    assets: AssetUnified,
    instruments: Vec<IndexSlot<InstrumentIndex, InstrumentFuture>>,
}

impl Indexer for IndexerFuture {
    type Exchange = ExchangeIdFuture;
    type Asset = AssetUnified;
    type Instrument = InstrumentFuture;

    type IndexerBuilder = IndexerFutureBuilder;
}

#[derive(Debug, Default)]
pub struct IndexerFutureBuilder {
    exchanges: Vec<ExchangeIdFuture>,
    assets: AssetUnified,
    instruments: Vec<InstrumentFuture>,
}

impl IndexerBuilder for IndexerFutureBuilder {
    type Indexer = IndexerFuture;
}
