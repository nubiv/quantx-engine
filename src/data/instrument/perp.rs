use rust_decimal::Decimal;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PerpetualDetail<AssetSlot> {
    contract_size: Decimal,
    settlement_asset: AssetSlot,
}
