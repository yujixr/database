use std::fmt;

use intermediate::IntermediateNode;
use leaf::LeafNode;
pub use root::RootNode;

mod intermediate;
mod leaf;
mod root;

#[derive(thiserror::Error, Debug)]
pub enum NodeError<K, V, const N: usize>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    #[error("node overflowed")]
    Overflow((K, K, Box<dyn Node<K, V, N>>)),
    #[error("key duplicated")]
    Duplicated,
    #[error("key not found")]
    NotFound,
    #[error("unknown node error")]
    Unknown,
}

pub trait Node<K, V, const N: usize>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn find(&self, key: &K) -> Option<&V>;
    fn insert(&mut self, key: &K, value: V, allow_upsert: bool) -> Result<(), NodeError<K, V, N>>;
    fn update(&mut self, key: &K, value: V) -> Result<(), NodeError<K, V, N>>;
    fn remove(&mut self, key: &K) -> Result<(), NodeError<K, V, N>>;
    fn collect(&self) -> Vec<(K, V)>;
}

impl<K, V, const N: usize> fmt::Debug for dyn Node<K, V, N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "node")?;
        Ok(())
    }
}
