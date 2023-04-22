use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("NBT error: {0}")]
    NBT(#[from] nbt::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
