mod io;
mod node;
mod persistence;
mod table;
mod tests;
mod transaction;

pub use node::{Node, RootNode};
pub use persistence::{dump, load};
pub use table::Table;
pub use transaction::{Request, Transaction};

const WAL_FOLDER_PATH: &str = "commit";
const DUMP_FILE_PATH: &str = "full_dump.json";
