use crate::engine::clock::EngineClock;

pub trait Processor {
    type Inspect;
    fn process(&mut self, event: &str) -> Self::Inspect;
}

pub trait EngineCore {}

pub trait EngineCoreDynamic {
    // TODO:
    // EngineCore for Dynamic Strategies (libloading or alternatives)
}
