use crate::data::common::Instrument;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct InstrumentFuture {
    exchange: String,
    name: String,
    kind: String,
    spec: String,
}

impl Instrument for InstrumentFuture {}
