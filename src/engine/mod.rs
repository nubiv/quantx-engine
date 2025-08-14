pub mod clock;
mod command;
mod component;
mod core;
mod execution;
pub mod inspect;
mod risk;
mod state;
mod strategy;

pub trait Engine {
    type Core;
    type Components;
    type FeedTx;
    type Inspect;

    fn shutdown() -> Self;
    fn send(&self);
    fn update_state(&self);
    // fn wire_inspector(&mut self) -> Option<&Inspect>;
}

pub trait EngineBuild {
    type Engine: Engine;

    fn new() -> Self;
    async fn init_internal(self, runtime: tokio::runtime::Handle) -> Self::Engine;

    async fn init(self) -> Self::Engine
    where
        Self: Sized,
    {
        self.init_internal(tokio::runtime::Handle::current()).await
    }

    async fn init_with_runtime(self, runtime: tokio::runtime::Handle) -> Self::Engine
    where
        Self: Sized,
    {
        self.init_internal(runtime).await
    }
}

pub trait EngineBuilder {
    type EngineBuild: EngineBuild;
}

pub trait EngineBuildArgs {}

#[derive(Debug, Default)]
pub enum EngineFeedMode {
    #[default]
    Iterator,
    Stream,
}

#[derive(Debug, Default)]
pub enum EngineAuditMode {
    Enabled,
    #[default]
    Disabled,
}
