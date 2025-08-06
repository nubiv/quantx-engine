use derive_more::Constructor;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Constructor)]
pub struct ExchangeIndex(usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Constructor)]
pub struct AssetIndex(pub usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Constructor)]
pub struct InstrumentIndex(usize);

pub trait Exchange: PartialEq + Eq + PartialOrd + Ord {}
pub trait Asset: PartialEq + Eq + PartialOrd + Ord {}
pub trait Instrument: PartialEq + Eq + PartialOrd + Ord {}
