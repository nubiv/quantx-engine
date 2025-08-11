use crate::{
    crypto::engine::{component::EngineComponentBundleCrypto, core::EngineCoreCrypto},
    engine::Engine,
};

mod component;
mod core;

#[derive(Debug)]
pub struct EngineCrypto<Clock> {
    core: EngineCoreCrypto<Clock>,
    components: EngineComponentBundleCrypto,
    feed_tx: String,
    // inspector: Option<Inspect>,
}

impl<Clock> Engine for EngineCrypto<Clock> {
    type Core = EngineCoreCrypto<Clock>;
    type Components = EngineComponentBundleCrypto;
    type FeedTx = String;
    type Inspect = ();

    fn shutdown() -> Self {
        todo!()
    }

    fn send(&self) {
        todo!()
    }

    fn update_state(&self) {
        todo!()
    }
}

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
