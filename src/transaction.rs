use super::io;
use super::node::{Node, RootNode};
use core::hash::Hash;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fmt, path::Path};

#[derive(thiserror::Error, Debug)]
pub enum TransactionError {
    #[error("target key not found; transaction aborted")]
    KeyNotFound,
}

pub struct Transaction<'a, K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    write_set: HashMap<K, Write<V>>,
    root_node: &'a mut RootNode<K, V>,
}

pub enum Request<K, V> {
    Find(K),
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

impl<K, V> Transaction<'_, K, V>
where
    K: 'static + fmt::Debug + Clone + Serialize + Hash + Ord,
    V: 'static + fmt::Debug + Clone + Serialize,
{
    pub fn new(root_node: &mut RootNode<K, V>) -> Transaction<K, V> {
        let write_set = HashMap::new();
        Transaction {
            write_set,
            root_node,
        }
    }

    pub fn exec(&mut self, req: Request<K, V>) -> Result<Option<V>, Box<dyn Error>> {
        Ok(match req {
            Request::Find(key) => {
                if let Some(w) = self.write_set.get(&key) {
                    match w {
                        Write::Insert(value) => Some(value.clone()),
                        Write::Update(value) => Some(value.clone()),
                        Write::Remove => None,
                    }
                } else {
                    if let Some(value) = self.root_node.find(&key) {
                        Some(value.clone())
                    } else {
                        None
                    }
                }
            }
            Request::Insert((key, value)) => {
                if let Some(w) = self.write_set.get_mut(&key) {
                    match w {
                        Write::Insert(_) => {
                            *w = Write::Insert(value);
                        }
                        Write::Update(_) => {
                            *w = Write::Update(value);
                        }
                        Write::Remove => {
                            *w = Write::Update(value);
                        }
                    }
                } else {
                    self.write_set.insert(key, Write::Insert(value));
                }
                None
            }
            Request::Update((key, value)) => {
                if let Some(w) = self.write_set.get_mut(&key) {
                    match w {
                        Write::Insert(_) => {
                            *w = Write::Insert(value);
                        }
                        Write::Update(_) => {
                            *w = Write::Update(value);
                        }
                        Write::Remove => {
                            *w = Write::Update(value);
                        }
                    }
                } else {
                    self.write_set.insert(key, Write::Update(value));
                }
                None
            }
            Request::Remove(key) => {
                if let Some(Write::Insert(_)) = self.write_set.get(&key) {
                    self.write_set.remove(&key);
                } else {
                    self.write_set.insert(key, Write::Remove);
                }
                None
            }
        })
    }

    pub fn commit(self, folder_path: &Path) -> Result<(), Box<dyn Error>> {
        if self.write_set.len() != 0 {
            for (key, w) in self.write_set.iter() {
                if let Write::Insert(_) = w {
                } else {
                    self.root_node
                        .find(key)
                        .ok_or(TransactionError::KeyNotFound)?;
                }
            }

            self.write_log(folder_path)?;

            for (key, w) in self.write_set.into_iter() {
                match w {
                    Write::Insert(value) => self.root_node.insert(&key, value, true),
                    Write::Update(value) => self.root_node.update(&key, value),
                    Write::Remove => self.root_node.remove(&key),
                }?;
            }
        }
        Ok(())
    }

    pub fn abort(self) {}

    fn write_log(&self, folder_path: &Path) -> Result<(), Box<dyn Error>> {
        io::dump(&folder_path.join(super::WAL_FOLDER_PATH), &self.write_set)?;
        Ok(())
    }
}
