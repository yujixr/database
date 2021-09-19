use super::secondary::SecondaryIndex;
use crate::RootNode;
use std::{collections::HashMap, fmt};

pub struct Table<K, V, const N: usize>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    pub primary: RootNode<K, V, N>,
    pub secondaries: HashMap<String, Box<dyn SecondaryIndex<K, V, N>>>,
}
