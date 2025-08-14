use futures::Stream;
use tracing::info;

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

pub fn sync_run<Events, EngineCore>(feed: &mut Events, engine: &mut EngineCore)
where
    Events: Iterator,
    EngineCore: Processor<Events::Item>,
{
}

pub fn sync_run_with_inspect<Events, EngineCore>(feed: &mut Events, engine: &mut EngineCore)
where
    Events: Iterator,
    EngineCore: Processor<Events::Item>,
{
}

pub async fn async_run<Events, EngineCore>(feed: &mut Events, engine: &mut EngineCore)
where
    Events: Stream,
    EngineCore: Processor<Events::Item>,
{
}

pub async fn async_run_with_inspect<Events, EngineCore>(feed: &mut Events, engine: &mut EngineCore)
where
    Events: Stream,
    EngineCore: Processor<Events::Item>,
{
    info!(feed_mode = "async", audit_mode = "enabled", "Engine running");

    // Run Engine process loop until shutdown
    // let shutdown_audit = loop {
    // let Some(event) = feed.next().await else {
    //     break engine.audit(FeedEnded);
    // };

    // Process Event with AuditTick generation
    // let audit = process_event(engine, event);
    // let output = engine.process(event);
    // let audit = engine.audit(output);

    // Check if AuditTick indicates shutdown is required
    // if audit.event.is_terminal() {
    //     break audit;
    // }

    // Send AuditTick to AuditManager
    // audit_tx.send(audit);
    // };

    // Send Shutdown audit
    // audit_tx.send(shutdown_audit.clone());

    // info!(
    //     shutdown_audit = ?shutdown_audit.event,
    //     context = ?shutdown_audit.context,
    //     "Engine shutting down"
    // );

    // let _ = engine.shutdown();

    // shutdown_audit.event
}
