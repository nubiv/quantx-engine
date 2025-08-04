use rust_decimal::Decimal;

#[derive(Debug)]
pub struct InstrumentSpec<AssetSlot> {
    price: InstrumentSpecPrice,
    quantity: InstrumentSpecQuantity<AssetSlot>,
}

#[derive(Debug)]
pub struct InstrumentSpecPrice {
    pub min: Decimal,
    pub tick_size: Decimal,
}

#[derive(Debug)]
pub struct InstrumentSpecQuantity<AssetSlot> {
    pub unit: OrderQuantityUnit<AssetSlot>,
    pub min: Decimal,
    pub increment: Decimal,
}

#[derive(Debug)]
pub enum OrderQuantityUnit<AssetSlot> {
    Asset(AssetSlot),
    Contract,
    Quote,
}
