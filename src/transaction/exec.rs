use super::{Request, Transaction, Write};
use crate::Node;
use core::hash::Hash;
use std::{error::Error, fmt};

impl<K, V, const N: usize> Transaction<'_, K, V, N>
where
    K: 'static + fmt::Debug + Clone + Hash + Ord,
    V: 'static + fmt::Debug + Clone,
{
    pub fn exec(&mut self, req: Request<K, V>) -> Result<(), Box<dyn Error>> {
        match req {
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
                    if let Some(_) = self.table.primary.find(&key) {
                        self.write_set.insert(key, Write::Update(value));
                    } else {
                        self.write_set.insert(key, Write::Insert(value));
                    }
                }
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
            }
            Request::Remove(key) => {
                if let Some(Write::Insert(_)) = self.write_set.get(&key) {
                    self.write_set.remove(&key);
                } else {
                    self.write_set.insert(key, Write::Remove);
                }
            }
        }
        Ok(())
    }
}
