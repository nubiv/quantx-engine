// Raw -> Indexer -> InstrumentKey (ExchangeId, InstrumentId), Subscription (ExchangeId, InstrumentId, ChannelKind) -> MarketEvent -> EngineEvent
// Raw -> Indexer -> InstrumentKey (ExchangeId, InstrumentId), Subscription (ExchangeId, InstrumentId, ChannelKind) -> AccountEvent -> EngineEvent
//
// EngineEvent -> <Processor>
// Clock -> match EngineEvent -> Strategy -> AuditTick
//
// Strategy -> (Opens, Cancels) -> RiskManager -> (OpensApproved, CancelsApproved, OpensRejected, CancelsRejected) -> Apprroved to InFlightRecord
//
// ExecutionTxs -> FnvIndexMap
//
// MarketEvent
//  time_exchange
//  time_local
//  kind
//  exchange?
//  instrument?
//
// AccountEvent
//  time_exchange
//  time_local
//  kind
//  exchange?
//
// EngineEvent
//  shutdown
//  command
//  feature_switch
//  market_event
//  account_event

// MarketConstraintFuture
