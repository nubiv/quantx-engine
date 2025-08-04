use crate::data::{
    asset::{Asset, AssetByExchange, AssetIndex},
    exchange::{Exchange, ExchangeIndex},
    instrument::{Instrument, InstrumentIndex},
};

mod asset;
mod exchange;
mod instrument;

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

#[derive(Debug)]
pub struct IndexSlot<K, V> {
    index: K,
    value: V,
}
