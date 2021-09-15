use super::*;

pub struct RootNode<K, V, const N: usize>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    root: IntermediateNode<K, V, N>,
}

impl<K, V, const N: usize> RootNode<K, V, N>
where
    K: 'static + fmt::Debug + Clone + Ord,
    V: 'static + fmt::Debug + Clone,
{
    pub fn new() -> Self {
        RootNode {
            root: IntermediateNode::new(Vec::new()),
        }
    }
}

impl<K, V, const N: usize> Node<K, V, N> for RootNode<K, V, N>
where
    K: 'static + fmt::Debug + Clone + Ord,
    V: 'static + fmt::Debug + Clone,
{
    fn find(&self, key: &K) -> Option<&V> {
        self.root.find(key)
    }

    fn insert(&mut self, key: &K, value: V) -> Result<(), NodeError<K, V, N>> {
        let result = self.root.insert(key, value);
        if let Err(NodeError::Overflow((first_last_key, second_last_key, second_node))) = result {
            let old_root = std::mem::take(self);
            self.root = IntermediateNode::new(vec![
                (first_last_key, Box::new(old_root.root)),
                (second_last_key, second_node),
            ]);
        } else {
            result?;
        }
        Ok(())
    }

    fn update(&mut self, key: &K, value: V) -> Result<(), NodeError<K, V, N>> {
        self.root.update(key, value)
    }

    fn remove(&mut self, key: &K) -> Result<(), NodeError<K, V, N>> {
        self.root.remove(key)
    }

    fn collect(&self) -> Vec<(K, V)> {
        self.root.collect()
    }
}

impl<K, V, const N: usize> Default for RootNode<K, V, N>
where
    K: 'static + fmt::Debug + Clone + Ord,
    V: 'static + fmt::Debug + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}
