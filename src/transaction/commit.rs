use super::{Transaction, TransactionError, Write};
use crate::Node;
use serde::Serialize;
use std::{error::Error, fmt, hash::Hash, path::Path};

impl<K, V, const N: usize> Transaction<'_, K, V, N>
where
    K: 'static + fmt::Debug + Clone + Serialize + Hash + Ord,
    V: 'static + fmt::Debug + Clone + Serialize,
{
    pub fn commit(self, folder_path: &Path) -> Result<(), Box<dyn Error>> {
        if self.write_set.len() != 0 {
            for (key, w) in self.write_set.iter() {
                match self.table.primary.find(key) {
                    Some(_) => match w {
                        Write::Insert(_) => Err(TransactionError::Unknown),
                        _ => Ok(()),
                    },
                    None => match w {
                        Write::Insert(_) => Ok(()),
                        _ => Err(TransactionError::KeyNotFound),
                    },
                }?;
            }

            self.write_log(folder_path)?;

            for (primary_key, w) in self.write_set.into_iter() {
                match w {
                    Write::Insert(value) => {
                        for (_, secondary) in self.table.secondaries.iter_mut() {
                            let key = secondary.select(&value);
                            secondary.append_to(&key, primary_key.clone())?;
                        }
                        self.table.primary.insert(&primary_key, value)
                    }
                    Write::Update(value) => {
                        let old_value = self
                            .table
                            .primary
                            .find(&primary_key)
                            .ok_or(TransactionError::Unknown)?;

                        for (_, secondary) in self.table.secondaries.iter_mut() {
                            let old_key = secondary.select(&old_value);
                            let new_key = secondary.select(&value);
                            secondary.remove_from(&old_key, primary_key.clone())?;
                            secondary.append_to(&new_key, primary_key.clone())?;
                        }

                        self.table.primary.update(&primary_key, value)
                    }
                    Write::Remove => {
                        let old_value = self
                            .table
                            .primary
                            .find(&primary_key)
                            .ok_or(TransactionError::Unknown)?;

                        for (_, secondary) in self.table.secondaries.iter_mut() {
                            let old_key = secondary.select(&old_value);
                            secondary.remove_from(&old_key, primary_key.clone())?;
                        }

                        self.table.primary.remove(&primary_key)
                    }
                }?;
            }
        }

        Ok(())
    }
}
