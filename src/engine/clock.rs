use chrono::{DateTime, Utc};

pub trait EngineClock {}

#[derive(Debug)]
pub struct SimulatedClock {
    inner: std::sync::Arc<parking_lot::RwLock<SimulatedClockInner>>,
}

#[derive(Debug)]
struct SimulatedClockInner {
    time_exchange_last: DateTime<Utc>,
    time_live_last_event: DateTime<Utc>,
}

impl EngineClock for SimulatedClock {}

#[derive(Debug)]
pub struct LiveClock {}

impl EngineClock for LiveClock {}
