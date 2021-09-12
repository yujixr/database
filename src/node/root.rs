use super::*;

pub struct RootNode<K, V>
where
    K: Debug,
    V: Debug,
{
    root: IntermediateNode<K, V>,
}

impl<K, V> RootNode<K, V>
where
    K: 'static + Debug + Clone + Ord,
    V: 'static + Debug + Clone,
{
    pub fn new() -> Self {
        RootNode {
            root: IntermediateNode::new(Vec::new()),
        }
    }

    pub fn insert(
        mut self,
        key: &K,
        value: V,
        allow_upsert: bool,
    ) -> Result<RootNode<K, V>, NodeError<K, V>> {
        let result = self.root.insert(key, value, allow_upsert);
        if let Err(NodeError::Overflow((first_last_key, second_last_key, second_node))) = result {
            Ok(RootNode {
                root: IntermediateNode::new(vec![
                    (first_last_key, Box::new(self.root)),
                    (second_last_key, second_node),
                ]),
            })
        } else {
            match result {
                Ok(_) => Ok(RootNode { root: self.root }),
                Err(e) => Err(e),
            }
        }
    }
}

impl<K, V> Node<K, V> for RootNode<K, V>
where
    K: 'static + Debug + Clone + Ord,
    V: 'static + Debug + Clone,
{
    fn find(&self, key: &K) -> Option<&V> {
        self.root.find(key)
    }

    fn insert(&mut self, _: &K, _: V, _: bool) -> Result<(), NodeError<K, V>> {
        Err(NodeError::NotSupported)
    }

    fn update(&mut self, key: &K, value: V) -> Result<(), NodeError<K, V>> {
        self.root.update(key, value)
    }

    fn remove(&mut self, key: &K) -> Result<(), NodeError<K, V>> {
        self.root.remove(key)
    }

    fn collect(&self) -> Vec<(K, V)> {
        self.root.collect()
    }
}
