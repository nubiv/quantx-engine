#[derive(Debug)]
pub struct MarketEvent {}

#[derive(Debug)]
pub struct AccountEvent {}

#[derive(Debug)]
pub enum EgnineEvent {
    Shutdown(String),
    Command(String),
    FeatureSwitch(String),
    Account(String),
    Market(String),
}
