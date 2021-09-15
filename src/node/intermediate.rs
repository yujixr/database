use super::*;
use std::{cmp::max, fmt};

pub struct IntermediateNode<K, V, const N: usize> {
    children: Vec<(K, Box<dyn Node<K, V, N>>)>,
}

impl<K, V, const N: usize> IntermediateNode<K, V, N>
where
    K: Ord,
{
    pub fn new(children: Vec<(K, Box<dyn Node<K, V, N>>)>) -> Self {
        IntermediateNode { children }
    }

    fn get_child_index(&self, key: &K) -> usize {
        self.children
            .binary_search_by_key(&key, |(key, _)| key)
            .map_or_else(|idx| idx, |idx| idx)
    }

    fn get_child(&self, key: &K) -> Option<&(K, Box<dyn Node<K, V, N>>)> {
        let idx = self.get_child_index(key);
        if idx == self.children.len() {
            self.children.last()
        } else {
            self.children.get(idx)
        }
    }

    fn get_child_mut(&mut self, key: &K) -> Option<&mut (K, Box<dyn Node<K, V, N>>)> {
        let idx = self.get_child_index(key);
        if idx == self.children.len() {
            self.children.last_mut()
        } else {
            self.children.get_mut(idx)
        }
    }
}

impl<K, V, const N: usize> Node<K, V, N> for IntermediateNode<K, V, N>
where
    K: 'static + fmt::Debug + Clone + Ord,
    V: 'static + fmt::Debug + Clone,
{
    fn find(&self, key: &K) -> Option<&V> {
        self.get_child(&key).map_or(None, |child| child.1.find(key))
    }

    fn insert(&mut self, key: &K, value: V) -> Result<(), NodeError<K, V, N>> {
        match self.get_child_mut(&key) {
            Some(child) => {
                let result = child.1.insert(&key, value);
                child.0 = max(&child.0, &key).clone();

                if let Err(NodeError::Overflow((first_last_key, second_last_key, second_node))) =
                    result
                {
                    child.0 = first_last_key.clone();
                    let idx = self.get_child_index(&first_last_key);
                    self.children
                        .insert(idx + 1, (second_last_key, second_node));
                    if self.children.len() > N + 1 {
                        let second_kv_series =
                            self.children.split_off((self.children.len() + 1) / 2);
                        let second_last_key =
                            second_kv_series.last().ok_or(NodeError::Unknown)?.0.clone();
                        let first_last_key =
                            self.children.last().ok_or(NodeError::Unknown)?.0.clone();

                        Err(NodeError::Overflow((
                            first_last_key,
                            second_last_key,
                            Box::new(IntermediateNode {
                                children: second_kv_series,
                            }),
                        )))
                    } else {
                        Ok(())
                    }
                } else {
                    result
                }
            }
            None => {
                if self.children.len() == 0 {
                    self.children = vec![(
                        key.clone(),
                        Box::new(LeafNode::new(vec![(key.clone(), value)])),
                    )];
                    Ok(())
                } else {
                    Err(NodeError::Unknown)
                }
            }
        }
    }

    fn update(&mut self, key: &K, value: V) -> Result<(), NodeError<K, V, N>> {
        self.get_child_mut(&key)
            .ok_or(NodeError::NotFound)?
            .1
            .update(key, value)
    }

    fn remove(&mut self, key: &K) -> Result<(), NodeError<K, V, N>> {
        self.get_child_mut(&key)
            .ok_or(NodeError::NotFound)?
            .1
            .remove(key)
    }

    fn collect(&self) -> Vec<(K, V)> {
        self.children
            .iter()
            .flat_map(|(_, child)| child.collect())
            .collect()
    }
}
