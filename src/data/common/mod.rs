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

mod sealed {
    pub trait Sealed {}
}

pub trait ExchangeMode: sealed::Sealed {
    const IS_SINGLE: bool;
}

pub struct SingleExchangeMode;
pub struct MultiExchangeMode;

impl sealed::Sealed for SingleExchangeMode {}
impl sealed::Sealed for MultiExchangeMode {}

impl ExchangeMode for SingleExchangeMode {
    const IS_SINGLE: bool = true;
}
impl ExchangeMode for MultiExchangeMode {
    const IS_SINGLE: bool = false;
}

pub trait AssetMode: sealed::Sealed {
    const IS_SINGLE: bool;
}

pub struct SingleAssetMode;
pub struct MultiAssetsMode;

impl sealed::Sealed for SingleAssetMode {}
impl sealed::Sealed for MultiAssetsMode {}

impl AssetMode for SingleAssetMode {
    const IS_SINGLE: bool = true;
}
impl AssetMode for MultiAssetsMode {
    const IS_SINGLE: bool = false;
}
