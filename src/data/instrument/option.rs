use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OptionDetail<AssetSlot> {
    contract_size: Decimal,
    settlement_asset: AssetSlot,
    kind: String,
    exercise: String,
    expiry: DateTime<Utc>,
    strike: Decimal,
}
