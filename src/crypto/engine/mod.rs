use crate::{
    crypto::engine::{component::EngineComponentBundleCrypto, core::EngineCoreCrypto},
    engine::{Engine, EngineAuditMode, EngineBuild, EngineBuildArgs, EngineBuilder, EngineFeedMode, inspect::Inspect},
};

mod component;
mod core;

#[derive(Debug)]
pub struct EngineCrypto<Clock> {
    core: EngineCoreCrypto<Clock>,
    components: EngineComponentBundleCrypto,
    feed_tx: String,
    inspector: Option<Inspect>,
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

#[derive(Debug)]
pub struct EngineBuildCrypto<E> {
    pub engine: E,
    pub engine_feed_mode: EngineFeedMode,
    pub audit_mode: EngineAuditMode,
    // pub market_stream: String,
    // pub account_channel: String,
    // pub execution_build_futures: String,
}

impl EngineBuild for EngineBuildCrypto<EngineCrypto<String>> {
    type Engine = EngineCrypto<String>;

    fn new() -> Self {
        todo!()
    }

    async fn init_internal(self, runtime: tokio::runtime::Handle) -> Self::Engine {
        let Self {
            mut engine,
            engine_feed_mode,
            audit_mode,
            // market_stream,
            // account_channel,
            // execution_build_futures,
            // phantom_event: _,
        } = self;

        // Initialise all execution components
        // let execution = execution_build_futures.init_with_runtime(runtime.clone()).await?;

        // Initialise central Engine channel
        // let (feed_tx, mut feed_rx) = mpsc_unbounded();

        // Forward MarketStreamEvents to Engine feed
        // let market_to_engine = runtime.clone().spawn(market_stream.forward_to(feed_tx.clone()));

        // Forward AccountStreamEvents to Engine feed
        // let account_stream = account_channel.rx.into_stream();
        // let account_to_engine = runtime.spawn(account_stream.forward_to(feed_tx.clone()));

        // Run Engine in configured mode
        let _ = match (engine_feed_mode, audit_mode) {
            (EngineFeedMode::Iterator, EngineAuditMode::Enabled) => {
                todo!()
            },
            (EngineFeedMode::Iterator, EngineAuditMode::Disabled) => {
                todo!()
            },
            (EngineFeedMode::Stream, EngineAuditMode::Enabled) => {
                todo!()
            },
            (EngineFeedMode::Stream, EngineAuditMode::Disabled) => {
                todo!()
            },
        };

        todo!()

        // EngineCrypto {
        //     core: engine.core,
        //     components: engine.components,
        //     feed_tx: engine.feed_tx,
        //     inspector: None,
        // }
    }
}

#[derive(Debug)]
pub struct EngineBuilderCrypto {
    args: String,
    engine_feed_mode: Option<EngineFeedMode>,
    audit_mode: Option<EngineAuditMode>,
    trading_state: Option<String>,
    balances: String,
}

impl EngineBuilder for EngineBuilderCrypto {
    type EngineBuild = EngineBuildCrypto<EngineCrypto<String>>;
}

#[derive(Debug)]
pub struct EngineBuildArgsCrypto<'i, I, C, S, MS> {
    pub instruments: &'i I,
    pub clock: C,
    pub strategy: S,
    pub market_stream: MS,
    pub executions: String,
    pub risk: String,
    pub global_data: String,
    pub instrument_data_init: String,
}

impl EngineBuildArgs for EngineBuildArgsCrypto<'_, String, String, String, String> {}
