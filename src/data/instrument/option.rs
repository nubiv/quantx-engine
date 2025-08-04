use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug)]
pub struct OptionDetail<AssetSlot> {
    contract_size: Decimal,
    settlement_asset: AssetSlot,
    kind: String,
    exercise: String,
    expiry: DateTime<Utc>,
    strike: Decimal,
}
