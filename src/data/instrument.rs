#[derive(Debug)]
pub struct InstrumentIndex(usize);

impl InstrumentIndex {
    pub fn index(&self) -> usize {
        self.0
    }
}

impl std::fmt::Display for InstrumentIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExchangeIndex({})", self.0)
    }
}
