use crate::engine::{component::EngineComponents, core::EngineCore, inspect::Inspect};

mod clock;
mod component;
mod core;
mod execution;
mod inspect;

#[derive(Debug)]
pub struct Engine {
    core: EngineCore,
    components: EngineComponents,
    feed_tx: String,
    inspector: Option<Inspect>,
}

#[derive(Debug)]
pub struct EngineBuilder {}

#[derive(Debug)]
pub struct EngineBuildArgs {}
