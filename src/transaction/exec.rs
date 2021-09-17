use super::{Request, Transaction, WritePrimary};
use crate::Node;
use core::hash::Hash;
use std::{error::Error, fmt};

impl<K, V, const N: usize> Transaction<'_, K, V, N>
where
    K: 'static + fmt::Debug + Clone + Hash + Ord,
    V: 'static + fmt::Debug + Clone,
{
    pub fn exec(&mut self, req: Request<K, V>) -> Result<Option<V>, Box<dyn Error>> {
        Ok(match req {
            Request::Find(key) => {
                if let Some(w) = self.write_set_primary.get(&key) {
                    match w {
                        WritePrimary::Insert(value) => Some(value.clone()),
                        WritePrimary::Update(value) => Some(value.clone()),
                        WritePrimary::Remove => None,
                    }
                } else {
                    if let Some(value) = self.table.primary.find(&key) {
                        Some(value.clone())
                    } else {
                        None
                    }
                }
            }
            Request::Insert((key, value)) => {
                if let Some(w) = self.write_set_primary.get_mut(&key) {
                    match w {
                        WritePrimary::Insert(_) => {
                            *w = WritePrimary::Insert(value);
                        }
                        WritePrimary::Update(_) => {
                            *w = WritePrimary::Update(value);
                        }
                        WritePrimary::Remove => {
                            *w = WritePrimary::Update(value);
                        }
                    }
                } else {
                    if let Some(_) = self.table.primary.find(&key) {
                        self.write_set_primary
                            .insert(key, WritePrimary::Update(value));
                    } else {
                        self.write_set_primary
                            .insert(key, WritePrimary::Insert(value));
                    }
                }
                None
            }
            Request::Update((key, value)) => {
                if let Some(w) = self.write_set_primary.get_mut(&key) {
                    match w {
                        WritePrimary::Insert(_) => {
                            *w = WritePrimary::Insert(value);
                        }
                        WritePrimary::Update(_) => {
                            *w = WritePrimary::Update(value);
                        }
                        WritePrimary::Remove => {
                            *w = WritePrimary::Update(value);
                        }
                    }
                } else {
                    self.write_set_primary
                        .insert(key, WritePrimary::Update(value));
                }
                None
            }
            Request::Remove(key) => {
                if let Some(WritePrimary::Insert(_)) = self.write_set_primary.get(&key) {
                    self.write_set_primary.remove(&key);
                } else {
                    self.write_set_primary.insert(key, WritePrimary::Remove);
                }
                None
            }
        })
    }
}
