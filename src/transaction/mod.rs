use crate::table::{Primitive, Table};
use crate::{io, Node};
use core::hash::Hash;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fmt, path::Path};

mod exec;

#[derive(thiserror::Error, Debug)]
pub enum TransactionError {
    #[error("target key not found; transaction aborted")]
    KeyNotFound,
}

pub struct Transaction<'a, K, V, const N: usize>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    write_set_primary: HashMap<K, WritePrimary<V>>,
    write_set_secondary: HashMap<String, HashMap<Primitive, WriteSecondary<K>>>,
    table: &'a mut Table<K, V, N>,
}

pub enum Request<K, V> {
    Find(K),
    Insert((K, V)),
    Update((K, V)),
    Remove(K),
}

#[derive(Serialize, Deserialize)]
pub enum WritePrimary<V> {
    Insert(V),
    Update(V),
    Remove,
}

#[derive(Serialize, Deserialize)]
pub enum WriteSecondary<K> {
    AppendTo(K),
    RemoveFrom(K),
}

impl<K, V, const N: usize> Transaction<'_, K, V, N>
where
    K: 'static + fmt::Debug + Clone + Serialize + Hash + Ord,
    V: 'static + fmt::Debug + Clone + Serialize,
{
    pub fn new(table: &mut Table<K, V, N>) -> Transaction<K, V, N> {
        let write_set_primary = HashMap::new();
        let write_set_secondary = HashMap::new();
        Transaction {
            write_set_primary,
            write_set_secondary,
            table,
        }
    }

    pub fn commit(self, folder_path: &Path) -> Result<(), Box<dyn Error>> {
        if self.write_set_primary.len() != 0 {
            for (key, w) in self.write_set_primary.iter() {
                if let WritePrimary::Insert(_) = w {
                } else {
                    self.table
                        .primary
                        .find(key)
                        .ok_or(TransactionError::KeyNotFound)?;
                }
            }

            self.write_log(folder_path)?;

            for (key, w) in self.write_set_primary.into_iter() {
                match w {
                    WritePrimary::Insert(value) => self.table.primary.insert(&key, value),
                    WritePrimary::Update(value) => self.table.primary.update(&key, value),
                    WritePrimary::Remove => self.table.primary.remove(&key),
                }?;
            }
        }

        Ok(())
    }

    pub fn abort(self) {}

    fn write_log(&self, folder_path: &Path) -> Result<(), Box<dyn Error>> {
        io::dump(
            &folder_path.join(super::WAL_FOLDER_PATH),
            &self.write_set_primary,
        )?;
        Ok(())
    }
}
