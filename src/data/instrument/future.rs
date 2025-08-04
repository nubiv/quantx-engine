use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug)]
pub struct FutureDetail<AssetSlot> {
    contract_size: Decimal,
    settlement_asset: AssetSlot,
    expiry: DateTime<Utc>,
}
