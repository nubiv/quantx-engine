use crate::engine::clock::EngineClock;

#[derive(Debug)]
pub struct EngineCore<Clock> {
    meta: String,
    clock: Clock,
    state: String,
    strategy: String,
    risk: String,
    exe_txs: String,
}

impl<Clock> EngineCore<Clock> where Clock: EngineClock {}
