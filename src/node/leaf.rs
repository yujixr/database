use super::*;

pub struct LeafNode<K, V, const N: usize> {
    kv_series: Vec<(K, V)>,
}

impl<K, V, const N: usize> LeafNode<K, V, N> {
    pub fn new(kv_series: Vec<(K, V)>) -> Self {
        LeafNode { kv_series }
    }
}

impl<K, V, const N: usize> Node<K, V, N> for LeafNode<K, V, N>
where
    K: 'static + fmt::Debug + Clone + Ord,
    V: 'static + fmt::Debug + Clone,
{
    fn find(&self, key: &K) -> Option<&V> {
        match self.kv_series.binary_search_by_key(&key, |(key, _)| key) {
            Ok(idx) => match self.kv_series.get(idx) {
                Some((_, value)) => Some(value),
                None => None,
            },
            Err(_) => None,
        }
    }

    fn insert(
        &mut self,
        key: &K,
        new_value: V,
        allow_upsert: bool,
    ) -> Result<(), NodeError<K, V, N>> {
        let r = match self.kv_series.binary_search_by_key(&key, |(key, _)| key) {
            Ok(idx) => {
                if allow_upsert {
                    match self.kv_series.get_mut(idx) {
                        Some((_, value)) => {
                            *value = new_value;
                            Ok(())
                        }
                        None => Err(NodeError::Unknown),
                    }
                } else {
                    Err(NodeError::Duplicated)
                }
            }
            Err(idx) => {
                self.kv_series.insert(idx, (key.clone(), new_value));
                Ok(())
            }
        };

        if self.kv_series.len() > N {
            let second_kv_series = self.kv_series.split_off((self.kv_series.len() + 1) / 2);
            let second_last_key = second_kv_series.last().ok_or(NodeError::Unknown)?.0.clone();
            let first_last_key = self.kv_series.last().ok_or(NodeError::Unknown)?.0.clone();

            Err(NodeError::Overflow((
                first_last_key,
                second_last_key,
                Box::new(LeafNode {
                    kv_series: second_kv_series,
                }),
            )))
        } else {
            r
        }
    }

    fn update(&mut self, key: &K, new_value: V) -> Result<(), NodeError<K, V, N>> {
        match self.kv_series.binary_search_by_key(&key, |(key, _)| key) {
            Ok(idx) => match self.kv_series.get_mut(idx) {
                Some((_, value)) => {
                    *value = new_value;
                    Ok(())
                }
                None => Err(NodeError::Unknown),
            },
            Err(_) => Err(NodeError::NotFound),
        }
    }

    fn remove(&mut self, key: &K) -> Result<(), NodeError<K, V, N>> {
        match self.kv_series.binary_search_by_key(&key, |(key, _)| key) {
            Ok(idx) => {
                self.kv_series.remove(idx);
                Ok(())
            }
            Err(_) => Err(NodeError::NotFound),
        }
    }

    fn collect(&self) -> Vec<(K, V)> {
        self.kv_series.clone()
    }
}
