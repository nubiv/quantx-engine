use crate::data::{
    common::{AssetIndex, ExchangeIndex, InstrumentIndex},
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

    type IndexerBuilder = IndexerCryptoBuilder;
}

#[derive(Debug, Default)]
pub struct IndexerCryptoBuilder {
    exchanges: Vec<ExchangeIdCrypto>,
    assets: Vec<AssetByExchange<AssetCrypto>>,
    instruments: Vec<InstrumentCrypto<IndexSlot<ExchangeIndex, ExchangeIdCrypto>, AssetIndex>>,
}

impl IndexerBuilder for IndexerCryptoBuilder {
    type Indexer = IndexerCrypto;
}
