mod channel;
mod clock;
mod engine;
mod error;
mod event;
mod reco;
mod trace;

use std::fmt::Debug;
use tracing::info;

// pub fn sync_run<Events, Engine>(feed: &mut Events, engine: &mut Engine) -> Engine::Audit {
//     info!(feed_mode = "sync", audit_mode = "disabled", "Engine running");

//     // Run Engine process loop until shutdown
//     let shutdown_audit = loop {
//         let Some(event) = feed.next() else {
//             break engine.audit(FeedEnded);
//         };

//         // Process Event with AuditTick generation
//         let audit = process_with_audit(engine, event);

//         // Check if AuditTick indicates a shutdown is required
//         if audit.event.is_terminal() {
//             break audit;
//         }
//     };

//     info!(
//         shutdown_audit = ?shutdown_audit.event,
//         context = ?shutdown_audit.context,
//         "Engine shutting down"
//     );

//     let _ = engine.shutdown();

//     shutdown_audit.event
// }

// pub fn sync_run_with_audit<Events, Engine, AuditTx>(feed: &mut Events, engine: &mut Engine, audit_tx: &mut ChannelTxDroppable<AuditTx>) -> Engine::Audit
// where
//     Events: Iterator,
//     Events::Item: Debug + Clone,
//     Engine: Processor<Events::Item> + Auditor<Engine::Audit, Context = EngineContext> + SyncShutdown,
//     Engine::Audit: From<FeedEnded> + Terminal + Debug + Clone,
//     AuditTx: Tx<Item = AuditTick<Engine::Audit, EngineContext>>,
// {
//     info!(feed_mode = "sync", audit_mode = "enabled", "Engine running");

//     // Run Engine process loop until shutdown
//     let shutdown_audit = loop {
//         let Some(event) = feed.next() else {
//             break engine.audit(FeedEnded);
//         };

//         // Process Event with AuditTick generation
//         let audit = process_with_audit(engine, event);

//         // Check if AuditTick indicates shutdown is required
//         if audit.event.is_terminal() {
//             break audit;
//         }

//         // Send AuditTick to AuditManager
//         audit_tx.send(audit);
//     };

//     // Send Shutdown audit
//     audit_tx.send(shutdown_audit.clone());

//     info!(
//         shutdown_audit = ?shutdown_audit.event,
//         context = ?shutdown_audit.context,
//         "Engine shutting down"
//     );

//     let _ = engine.shutdown();

//     shutdown_audit.event
// }

// pub async fn async_run<Events, Engine>(feed: &mut Events, engine: &mut Engine) -> Engine::Audit
// where
//     Events: Stream + Unpin,
//     Events::Item: Debug + Clone,
//     Engine: Processor<Events::Item> + Auditor<Engine::Audit, Context = EngineContext> + SyncShutdown,
//     Engine::Audit: From<FeedEnded> + Terminal + Debug,
// {
//     info!(feed_mode = "async", audit_mode = "disabled", "Engine running");

//     // Run Engine process loop until shutdown
//     let shutdown_audit = loop {
//         let Some(event) = feed.next().await else {
//             break engine.audit(FeedEnded);
//         };

//         // Process Event with AuditTick generation
//         let audit = process_with_audit(engine, event);

//         // Check if AuditTick indicates shutdown is required
//         if audit.event.is_terminal() {
//             break audit;
//         }
//     };

//     info!(
//         shutdown_audit = ?shutdown_audit.event,
//         context = ?shutdown_audit.context,
//         "Engine shutting down"
//     );

//     let _ = engine.shutdown();

//     shutdown_audit.event
// }

// pub async fn async_run_with_audit<Events, Engine, AuditTx>(feed: &mut Events, engine: &mut Engine, audit_tx: &mut ChannelTxDroppable<AuditTx>) -> Engine::Audit
// where
//     Events: Stream + Unpin,
//     Events::Item: Debug + Clone,
//     Engine: Processor<Events::Item> + Auditor<Engine::Audit, Context = EngineContext> + SyncShutdown,
//     Engine::Audit: From<FeedEnded> + Terminal + Debug + Clone,
//     AuditTx: Tx<Item = AuditTick<Engine::Audit, EngineContext>>,
// {
//     info!(feed_mode = "async", audit_mode = "enabled", "Engine running");

//     // Run Engine process loop until shutdown
//     let shutdown_audit = loop {
//         let Some(event) = feed.next().await else {
//             break engine.audit(FeedEnded);
//         };

//         // Process Event with AuditTick generation
//         let audit = process_with_audit(engine, event);

//         // Check if AuditTick indicates shutdown is required
//         if audit.event.is_terminal() {
//             break audit;
//         }

//         // Send AuditTick to AuditManager
//         audit_tx.send(audit);
//     };

//     // Send Shutdown audit
//     audit_tx.send(shutdown_audit.clone());

//     info!(
//         shutdown_audit = ?shutdown_audit.event,
//         context = ?shutdown_audit.context,
//         "Engine shutting down"
//     );

//     let _ = engine.shutdown();

//     shutdown_audit.event
// }
