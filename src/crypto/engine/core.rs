use crate::engine::clock::EngineClock;

#[derive(Debug)]
pub struct EngineCoreCrypto<Clock> {
    meta: String,
    clock: Clock,
    state: String,
    strategy: String,
    risk: String,
    exe_txs: String,
}

impl<Clock> EngineCoreCrypto<Clock> where Clock: EngineClock {}
