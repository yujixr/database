use super::{Transaction, TransactionError, Write};
use crate::{table::Primitive, Node};
use std::{collections::HashSet, error::Error, fmt, hash::Hash};

impl<K, V, const N: usize> Transaction<'_, K, V, N>
where
    K: 'static + fmt::Debug + Clone + Hash + Ord,
    V: 'static + fmt::Debug + Clone,
{
    pub fn find(&self, key: &K) -> Result<Option<V>, Box<dyn Error>> {
        Ok(if let Some(w) = self.write_set.get(key) {
            match w {
                Write::Insert(value) => Some(value.clone()),
                Write::Update(value) => Some(value.clone()),
                Write::Remove => None,
            }
        } else {
            match self.table.primary.find(key) {
                Some(value) => Some(value.clone()),
                None => None,
            }
        })
    }

    pub fn select(&self, index: &String, key: &Primitive) -> Result<HashSet<K>, Box<dyn Error>> {
        let index = self
            .table
            .secondaries
            .get(index)
            .ok_or(TransactionError::SecondaryIndexNotFound)?;

        if !index.validate(key) {
            Err(TransactionError::IllegalKeyType)?;
        }

        let mut primary_keys = if let Some(primary_keys) = index.find(key) {
            primary_keys.to_owned()
        } else {
            HashSet::new()
        };

        primary_keys.retain(|primary_key| {
            if let Some(w) = self.write_set.get(&primary_key) {
                match w {
                    Write::Insert(value) | Write::Update(value)
                        if &index.select(value.clone()) != key =>
                    {
                        false
                    }
                    Write::Remove => false,
                    _ => true,
                }
            } else {
                true
            }
        });

        for (primary_key, w) in self.write_set.iter() {
            match w {
                Write::Insert(value) | Write::Update(value)
                    if &index.select(value.clone()) == key =>
                {
                    primary_keys.insert(primary_key.clone());
                }
                _ => {}
            }
        }

        Ok(primary_keys)
    }
}
