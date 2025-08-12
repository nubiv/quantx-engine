use futures::Stream;

use crate::engine::clock::EngineClock;

pub trait Processor<Event> {
    type Inspect;
    fn process(&mut self, event: Event) -> Self::Inspect;
}

pub trait EngineCore {}

pub trait EngineCoreDynamic {
    // TODO:
    // EngineCore for Dynamic Strategies (libloading or alternatives)
}

pub fn sync_run<Events, Engine>(feed: &mut Events, engine: &mut Engine)
where
    Events: Iterator,
    Engine: Processor<Events::Item>,
{
}

pub fn sync_run_with_audit<Events, Engine>(feed: &mut Events, engine: &mut Engine)
where
    Events: Iterator,
    Engine: Processor<Events::Item>,
{
}

pub async fn async_run<Events, Engine>(feed: &mut Events, engine: &mut Engine)
where
    Events: Stream,
    Engine: Processor<Events::Item>,
{
}

pub async fn async_run_with_audit<Events, Engine>(feed: &mut Events, engine: &mut Engine)
where
    Events: Stream,
    Engine: Processor<Events::Item>,
{
}
