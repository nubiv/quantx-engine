use rust_decimal::Decimal;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct InstrumentSpec<AssetSlot> {
    price: InstrumentSpecPrice,
    quantity: InstrumentSpecQuantity<AssetSlot>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct InstrumentSpecPrice {
    pub min: Decimal,
    pub tick_size: Decimal,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct InstrumentSpecQuantity<AssetSlot> {
    pub unit: OrderQuantityUnit<AssetSlot>,
    pub min: Decimal,
    pub increment: Decimal,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum OrderQuantityUnit<AssetSlot> {
    Asset(AssetSlot),
    Contract,
    Quote,
}
