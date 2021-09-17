use super::secondary::SecondaryIndex;
use crate::RootNode;
use std::fmt;

pub struct Table<K, V, const N: usize>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    pub primary: RootNode<K, V, N>,
    pub secondaries: Vec<Box<dyn SecondaryIndex<K, V, N>>>,
}
