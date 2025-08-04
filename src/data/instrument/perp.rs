use rust_decimal::Decimal;

#[derive(Debug)]
pub struct PerpetualDetail<AssetSlot> {
    contract_size: Decimal,
    settlement_asset: AssetSlot,
}
