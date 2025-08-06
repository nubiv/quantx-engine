use crate::data::{
    common::{AssetIndex, ExchangeIndex, Instrument, InstrumentIndex},
    crypto::{
        asset::{AssetByExchange, AssetCrypto},
        exchange::ExchangeIdCrypto,
        instrument::InstrumentCrypto,
    },
    index::{IndexSlot, Indexer, IndexerBuilder},
};

mod asset;
mod exchange;
mod instrument;

#[derive(Debug)]
pub struct IndexerCrypto {
    exchanges: Vec<IndexSlot<ExchangeIndex, ExchangeIdCrypto>>,
    assets: Vec<IndexSlot<AssetIndex, AssetByExchange<AssetCrypto>>>,
    instruments: Vec<IndexSlot<InstrumentIndex, InstrumentCrypto<IndexSlot<ExchangeIndex, ExchangeIdCrypto>, AssetIndex>>>,
}

impl Indexer for IndexerCrypto {
    type Exchange = ExchangeIdCrypto;
    type Asset = AssetByExchange<AssetCrypto>;
    type Instrument = InstrumentCrypto<IndexSlot<ExchangeIndex, ExchangeIdCrypto>, AssetIndex>;
    type UnindexedInstrument = InstrumentCrypto<ExchangeIdCrypto, AssetCrypto>;

    type IndexerBuilder = IndexerCryptoBuilder;

    fn new<Iter, I>(instruments: Iter) -> Self
    where
        Iter: IntoIterator<Item = I>,
        I: Into<Self::UnindexedInstrument>,
    {
        todo!()
    }

    fn exchanges(&self) -> &[IndexSlot<ExchangeIndex, Self::Exchange>] {
        todo!()
    }

    fn assets(&self) -> &[IndexSlot<AssetIndex, Self::Asset>] {
        todo!()
    }

    fn instruments(&self) -> &[IndexSlot<InstrumentIndex, Self::Instrument>] {
        todo!()
    }
}

#[derive(Debug, Default)]
pub struct IndexerCryptoBuilder {
    exchanges: Vec<ExchangeIdCrypto>,
    assets: Vec<AssetByExchange<AssetCrypto>>,
    instruments: Vec<InstrumentCrypto<IndexSlot<ExchangeIndex, ExchangeIdCrypto>, AssetIndex>>,
}

impl IndexerBuilder for IndexerCryptoBuilder {
    type Indexer = IndexerCrypto;
    type UnindexedInstrument = InstrumentCrypto<ExchangeIdCrypto, AssetCrypto>;

    fn add_instrument(self, instrument: Self::UnindexedInstrument) -> Self {
        todo!()
    }

    fn build(self) -> Self::Indexer {
        todo!()
    }
}
