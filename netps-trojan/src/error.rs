use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("NoEnoughPasswordLength")]
    WrongFormat,

    #[error("UnknownCmd")]
    UnknownCmd,

    #[error("UnknownAddressType")]
    UnknownAddressType,

    #[error(transparent)]
    IoError(#[from] netps_core::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
