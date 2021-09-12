use super::io;
use super::node::{Node, RootNode};
use core::hash::Hash;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug, path::Path};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransactionError {
    #[error("target key not found; transaction aborted")]
    KeyNotFound,
}

pub struct Transaction<'a, K, V>
where
    K: Debug,
    V: Debug,
{
    write_set: HashMap<K, Write<V>>,
    root_node: &'a RootNode<K, V>,
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
    K: 'static + Debug + Clone + Serialize + Hash + Ord,
    V: 'static + Debug + Clone + Serialize,
{
    pub fn new(root_node: &RootNode<K, V>) -> Transaction<K, V> {
        let write_set = HashMap::new();
        Transaction {
            write_set,
            root_node,
        }
    }

    pub fn exec(&mut self, req: Request<K, V>) -> Result<Option<V>, Box<dyn std::error::Error>> {
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

    pub fn commit(
        self,
        folder_path: &Path,
    ) -> Result<
        Box<dyn Fn(RootNode<K, V>) -> Result<RootNode<K, V>, Box<dyn std::error::Error>>>,
        Box<dyn std::error::Error>,
    > {
        if self.write_set.len() == 0 {
            Ok(Box::new(|root_node| Ok(root_node)))
        } else {
            for (key, w) in self.write_set.iter() {
                if let Write::Insert(_) = w {
                } else {
                    self.root_node
                        .find(key)
                        .ok_or(TransactionError::KeyNotFound)?;
                }
            }

            self.write_log(folder_path)?;
            let write_set = self.write_set;

            Ok(Box::new(move |mut root_node: RootNode<K, V>| {
                for (key, w) in write_set.iter() {
                    root_node = match w {
                        Write::Insert(value) => root_node.insert(key, value.clone(), true),
                        Write::Update(value) => {
                            root_node.update(key, value.clone())?;
                            Ok(root_node)
                        }
                        Write::Remove => {
                            root_node.remove(key)?;
                            Ok(root_node)
                        }
                    }?;
                }
                Ok(root_node)
            }))
        }
    }

    pub fn abort(self) {}

    fn write_log(&self, folder_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        io::dump(&folder_path.join("./commit"), &self.write_set)?;
        Ok(())
    }
}
