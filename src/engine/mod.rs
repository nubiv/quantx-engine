use crate::{
    data::{
        common::{AssetMode, ExchangeMode},
        index::Indexer,
    },
    engine::{component::EngineComponents, core::EngineCore, inspect::Inspect},
};

mod clock;
mod component;
mod core;
mod execution;
mod inspect;
mod state;

pub trait Engine {
    fn shutdown() -> Self;

    fn send(&self);

    fn update_state(&self);

    fn wire_inspector(&mut self) -> Option<&Inspect>;
}

// #[derive(Debug)]
// pub struct Engine<Clock> {
//     core: EngineCore<Clock>,
//     components: EngineComponents,
//     feed_tx: String,
//     inspector: Option<Inspect>,
// }

// #[derive(Debug)]
// pub struct EngineBuild {
//     args: SystemArgs<'a, Clock, Strategy, Risk, MarketStream, GlobalData, FnInstrumentData>,
//     engine_feed_mode: Option<EngineFeedMode>,
//     audit_mode: Option<AuditMode>,
//     trading_state: Option<TradingState>,
//     balances: FnvHashMap<ExchangeAsset<AssetNameInternal>, Balance>,
// }

// #[derive(Debug)]
// pub struct EngineBuilder {}

// #[derive(Debug)]
// pub struct EngineBuildArgs<'i, I, EM, AM, C, S, MS>
// where
//     I: Indexer<EM, AM>,
//     EM: ExchangeMode,
//     AM: AssetMode,
// {
//     pub instruments: &'i I,
//     pub clock: C,
//     pub strategy: S,
//     pub market_stream: MS,
//     pub executions: Vec<ExecutionConfig>,
//     pub risk: Risk,
//     pub global_data: GlobalData,
//     pub instrument_data_init: FnInstrumentData,
// }
