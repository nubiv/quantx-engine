use crate::data::common::Asset;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct AssetUnified(smol_str::SmolStr);

impl Asset for AssetUnified {}
