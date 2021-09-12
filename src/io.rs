use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use serde::{de::DeserializeOwned, Serialize};
use sha2::{self, Digest};
use std::{
    error::Error,
    fs, io,
    io::Read,
    path::{Path, PathBuf},
    time::SystemTime,
};

#[derive(thiserror::Error, Debug)]
pub enum IOError {
    #[error("hash is not matched")]
    HashMismatch,
    #[error("file size is not matched")]
    FileSizeMismatch,
}

fn now() -> Result<u128, Box<dyn Error>> {
    Ok(SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_nanos())
}

fn hash<T>(value: T) -> Vec<u8>
where
    T: AsRef<[u8]>,
{
    let mut hasher = sha2::Sha512::new();
    hasher.update(&value);
    hasher.finalize().as_ref().to_vec()
}

pub fn dump<T>(folder_path: &Path, value: &T) -> Result<PathBuf, Box<dyn Error>>
where
    T: ?Sized + Serialize,
{
    let json = serde_json::to_string(value)?;
    let json_len = json.len() as u64;
    let json_hash = hash(&json);

    fs::create_dir_all(folder_path)?;
    let file_path = folder_path.join(format!("{}.json", now()?));
    let mut f = fs::File::create(&file_path)?;

    f.write_u64::<LittleEndian>(json_len)?;
    io::Write::write_all(&mut f, &json_hash)?;
    io::Write::write_all(&mut f, json.as_bytes())?;
    f.sync_all()?;

    Ok(file_path)
}

pub fn load<T>(file_path: &Path) -> Result<T, Box<dyn Error>>
where
    T: DeserializeOwned,
{
    let f = fs::File::open(file_path)?;
    let mut buf_reader = io::BufReader::new(f);

    let json_len = buf_reader.read_u64::<LittleEndian>()? as usize;
    let mut json_hash = [0u8; 64];
    let mut json_string = String::new();

    buf_reader.read_exact(&mut json_hash)?;
    buf_reader.read_to_string(&mut json_string)?;

    if json_len != json_string.len() {
        Err(Box::new(IOError::FileSizeMismatch))
    } else if json_hash != hash(&json_string).as_slice() {
        Err(Box::new(IOError::HashMismatch))
    } else {
        let value: T = serde_json::from_str(&json_string)?;
        Ok(value)
    }
}

pub fn remove_dir(folder_path: &Path) -> Result<(), Box<dyn Error>> {
    if let Err(e) = fs::remove_dir_all(&folder_path.join("commit")) {
        if let std::io::ErrorKind::NotFound = e.kind() {
            Ok(())
        } else {
            Err(Box::new(e))
        }
    } else {
        Ok(())
    }
}
