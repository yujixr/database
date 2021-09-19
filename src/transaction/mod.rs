use crate::io;
use crate::table::Table;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fmt, hash::Hash, path::Path};

mod commit;
mod exec;
mod query;

#[derive(thiserror::Error, Debug)]
pub enum TransactionError {
    #[error("target key not found; transaction aborted")]
    KeyNotFound,
    #[error("target key not found")]
    SecondaryIndexNotFound,
    #[error("key type not matched")]
    IllegalKeyType,
    #[error("unknown transaction error")]
    Unknown,
}

pub struct Transaction<'a, K, V, const N: usize>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    write_set: HashMap<K, Write<V>>,
    table: &'a mut Table<K, V, N>,
}

pub enum Request<K, V> {
    Insert((K, V)),
    Update((K, V)),
    Remove(K),
}

#[derive(Serialize, Deserialize)]
pub enum Write<V> {
    Insert(V),
    Update(V),
    Remove,
}

#[derive(Serialize, Deserialize)]
pub enum WriteSecondary {
    InsertTo,
    RemoveFrom,
}

impl<K, V, const N: usize> Transaction<'_, K, V, N>
where
    K: 'static + fmt::Debug + Clone + Serialize + Hash + Ord,
    V: 'static + fmt::Debug + Clone + Serialize,
{
    pub fn new(table: &mut Table<K, V, N>) -> Transaction<K, V, N> {
        let write_set = HashMap::new();
        Transaction { write_set, table }
    }

    pub fn abort(self) {}

    fn write_log(&self, folder_path: &Path) -> Result<(), Box<dyn Error>> {
        io::dump(&folder_path.join(crate::WAL_FOLDER_PATH), &self.write_set)?;
        Ok(())
    }
}
