use super::io;
use super::node::{Node, RootNode};
use super::transaction::Write;
use serde::{de::DeserializeOwned, Serialize};
use std::{collections::HashMap, error::Error, fmt, fs, hash::Hash, path::Path};

pub fn dump<
    K: 'static + fmt::Debug + Clone + Serialize + Ord,
    V: 'static + fmt::Debug + Clone + Serialize,
>(
    root_node: &RootNode<K, V>,
    folder_path: &Path,
) -> Result<(), Box<dyn Error>> {
    let file_path = io::dump(folder_path, &Node::collect(root_node))?;
    fs::rename(file_path, &folder_path.join("./full_dump.json"))?;
    io::remove_dir(&folder_path.join("./commit"))
}

pub fn load<
    K: 'static + fmt::Debug + Clone + Serialize + DeserializeOwned + Ord + Hash,
    V: 'static + fmt::Debug + Clone + Serialize + DeserializeOwned,
>(
    folder_path: &Path,
) -> Result<RootNode<K, V>, Box<dyn Error>> {
    let mut root_node = RootNode::<K, V>::new();
    let kv_series: Vec<(K, V)> = io::load(&folder_path.join("./full_dump.json"))?;

    for (key, value) in kv_series.into_iter() {
        root_node = root_node.insert(&key, value, false)?;
    }

    match fs::read_dir(&folder_path.join("./commit")) {
        Ok(dir) => {
            let mut entries = dir
                .map(|res| {
                    res.map(|e| {
                        println!("{:?}", e.path());
                        e.path()
                    })
                })
                .collect::<Result<Vec<_>, std::io::Error>>()?;
            entries.sort();

            for path in entries {
                let write_set: HashMap<K, Write<V>> = io::load(&path)?;
                for (key, w) in write_set {
                    root_node = match w {
                        Write::Insert(value) => root_node.insert(&key, value, true),
                        Write::Update(value) => {
                            root_node.update(&key, value)?;
                            Ok(root_node)
                        }
                        Write::Remove => {
                            root_node.remove(&key)?;
                            Ok(root_node)
                        }
                    }?;
                }
            }
            Ok(root_node)
        }
        Err(e) => {
            if let std::io::ErrorKind::NotFound = e.kind() {
                Ok(root_node)
            } else {
                Err(Box::new(e))
            }
        }
    }
}
