use derive_more::Constructor;

use crate::data::{
    asset::{Asset, AssetIndex, AssetUnified},
    exchange::{Exchange, ExchangeIdFutureCN, ExchangeIndex},
    instrument::{Instrument, InstrumentFuture, InstrumentIndex, InstrumentName},
};

mod asset;
mod exchange;
mod instrument;

#[derive(Debug, Constructor)]
pub struct IndexSlot<K, V> {
    index: K,
    value: V,
}

#[derive(Debug)]
pub struct InternalIndexMap<E, A, I>
where
    E: Exchange,
    A: Asset,
    I: Instrument,
{
    exchanges: Vec<IndexSlot<ExchangeIndex, E>>,
    assets: Vec<IndexSlot<AssetIndex, A>>,
    instruments: Vec<IndexSlot<InstrumentIndex, I>>,
}

impl<E, A, I> InternalIndexMap<E, A, I>
where
    E: Exchange,
    A: Asset,
    I: Instrument,
{
    pub fn builder() -> InternalIndexMapBuilder<E, A, I> {
        InternalIndexMapBuilder::new()
    }

    pub fn new_future<Iter, Inst>(instruments: Iter) -> Self
    where
        Iter: IntoIterator<Item = Inst>,
        Inst: Into<InstrumentFuture<ExchangeIdFutureCN, AssetUnified, InstrumentName>>,
    {
        instruments
            .into_iter()
            .fold(Self::builder(), |builder, instrument| builder.add_instrument_future(instrument.into()))
            .build_unified_asset()
    }
}

#[derive(Debug)]
pub struct InternalIndexMapBuilder<E, A, I>
where
    E: Exchange,
    A: Asset,
    I: Instrument,
{
    exchanges: Vec<E>,
    assets: Vec<A>,
    instruments: Vec<I>,
}

impl<E, A, I> InternalIndexMapBuilder<E, A, I>
where
    E: Exchange,
    A: Asset,
    I: Instrument,
{
    pub fn new() -> Self {
        Self {
            exchanges: Vec::new(),
            assets: Vec::new(),
            instruments: Vec::new(),
        }
    }

    pub fn add_instrument_future(mut self, instrument: InstrumentFuture<ExchangeIdFutureCN, AssetUnified, InstrumentName>) -> Self {
        self
    }

    pub fn build_unified_asset(mut self) -> InternalIndexMap<E, A, I> {
        self.exchanges.sort();
        self.exchanges.dedup();
        self.assets.sort();
        self.assets.dedup();
        self.instruments.sort();
        self.instruments.dedup();

        let exchange_idxs = self
            .exchanges
            .into_iter()
            .enumerate()
            .map(|(index, exchange)| IndexSlot::new(ExchangeIndex::new(index), exchange))
            .collect::<Vec<_>>();

        let asset_idxs = self
            .assets
            .into_iter()
            .enumerate()
            .map(|(index, asset)| IndexSlot::new(AssetIndex::new(index), asset))
            .collect::<Vec<_>>();

        let instrument_idxs = self
            .instruments
            .into_iter()
            .enumerate()
            .map(|(index, instrument)| IndexSlot::new(InstrumentIndex::new(index), instrument))
            .collect::<Vec<_>>();

        InternalIndexMap {
            exchanges: exchange_idxs,
            assets: asset_idxs,
            instruments: instrument_idxs,
        }
    }

    pub fn build_cross_exchanges(mut self) -> InternalIndexMap<E, A, I> {
        self.exchanges.sort();
        self.exchanges.dedup();
        self.assets.sort();
        self.assets.dedup();
        self.instruments.sort();
        self.instruments.dedup();

        let exchange_idxs = self
            .exchanges
            .into_iter()
            .enumerate()
            .map(|(index, exchange)| IndexSlot::new(ExchangeIndex::new(index), exchange))
            .collect::<Vec<_>>();

        let asset_idxs = self
            .assets
            .into_iter()
            .enumerate()
            .map(|(index, asset)| IndexSlot::new(AssetIndex::new(index), asset))
            .collect::<Vec<_>>();

        let instrument_idxs = self
            .instruments
            .into_iter()
            .enumerate()
            .map(|(index, instrument)| IndexSlot::new(InstrumentIndex::new(index), instrument))
            .collect::<Vec<_>>();

        InternalIndexMap {
            exchanges: exchange_idxs,
            assets: asset_idxs,
            instruments: instrument_idxs,
        }
    }
}
