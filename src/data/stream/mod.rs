use fnv::FnvHashMap;
use kanal::Receiver;

use crate::data::common::Exchange;

#[derive(Debug)]
pub struct StreamAggregated<E, T>
where
    E: Exchange,
{
    streams: FnvHashMap<E, Receiver<T>>,
}

#[derive(Debug)]
pub struct StreamSingle<T> {
    front: String,
    stream: Receiver<T>,
}
