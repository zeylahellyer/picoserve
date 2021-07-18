use super::super::response::{Response, WriteError};
use std::{
    error::Error,
    ffi::OsStr,
    fmt::{Display, Formatter, Result as FmtResult},
    fs,
    io::ErrorKind,
    net::TcpStream,
    path::Path,
};

/// Error occurred when processing a GET request.
#[derive(Debug)]
pub enum GetError {
    /// Failed to write to a TCP stream.
    Write {
        /// Source of the error.
        source: WriteError,
    },
}

impl Display for GetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a")
    }
}

impl Error for GetError {}

/// Handle a GET request.
pub fn get(stream: &mut TcpStream, path: &Path) -> Result<(), GetError> {
    match fs::read(path) {
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
