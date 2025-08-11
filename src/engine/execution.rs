pub trait ExecutionManager {
    type ExecutionTx;

    fn get_by_exchange_index(&self, exchange_index: &str) -> Option<&Self::ExecutionTx>;
    fn iter(&self) -> impl Iterator<Item = &Self::ExecutionTx>;
}

#[derive(Debug)]
pub struct Execution {}

#[derive(Debug)]
pub struct ExecutionBuilder {}
