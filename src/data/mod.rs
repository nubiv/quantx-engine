mod asset;
mod exchange;
mod instrument;

pub type FnvIndexMap<K, V> = indexmap::IndexMap<K, V, fnv::FnvBuildHasher>;
pub type FnvIndexSet<T> = indexmap::IndexSet<T, fnv::FnvBuildHasher>;

#[derive(Debug)]
pub struct InternalIndexMap {}
