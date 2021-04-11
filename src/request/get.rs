use crate::response::{Response, WriteError};
use std::{
    error::Error,
    ffi::OsStr,
    fmt::{Display, Formatter, Result as FmtResult},
    fs,
    io::ErrorKind,
    net::TcpStream,
    path::PathBuf,
};

#[derive(Debug)]
pub enum GetError {
    Write { source: WriteError },
}

impl Display for GetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a")
    }
}

impl Error for GetError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Write { source } => Some(source),
        }
    }
}

pub fn get(stream: &mut TcpStream, path: PathBuf) -> Result<(), GetError> {
    match fs::read(&path) {
        Ok(bytes) => Response::new(&bytes)
            .extension(path.extension().and_then(OsStr::to_str))
            .ok()
            .write(stream)
            .map_err(|source| GetError::Write { source }),
        Err(source) => {
            let response = Response::new(b"");

            let response = match source.kind() {
                ErrorKind::Other if source.raw_os_error() == Some(21) => response.forbidden(),
                ErrorKind::NotFound => response.not_found(),
                _ => response.internal_service_error(),
            };

            response
                .write(stream)
                .map_err(|source| GetError::Write { source })
        }
    }
}
