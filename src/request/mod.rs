mod get;
mod index;

use self::{get::GetError, index::IndexError};
use crate::{
    env::Environment,
    response::{Response, WriteError},
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    io::{Error as IoError, Read},
    net::TcpStream,
};

#[derive(Debug)]
pub enum RequestError {
    Get { source: GetError },
    Index { source: IndexError },
    ReadFromStream { source: IoError },
    Write { source: WriteError },
}

impl Display for RequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("display")
    }
}

impl Error for RequestError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Get { source } => Some(source),
            Self::Index { source } => Some(source),
            Self::ReadFromStream { source } => Some(source),
            Self::Write { source } => Some(source),
        }
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum RequestedMethod {
    Get,
}

impl RequestedMethod {
    fn from_input(buf: &[u8]) -> Option<Self> {
        match buf.split(|x| *x == b' ').next() {
            Some(b"GET") => Some(Self::Get),
            _ => None,
        }
    }
}

pub fn handle(stream: &mut TcpStream, env: &Environment) -> Result<(), RequestError> {
    let mut buf = [0; 1024];

    let _ = stream
        .read(&mut buf)
        .map_err(|source| RequestError::ReadFromStream { source })?;

    if RequestedMethod::from_input(&buf).is_none() {
        return Response::new(b"")
            .method_not_allowed(&[b"GET"])
            .write(stream)
            .map_err(|source| RequestError::Write { source });
    }

    let part = buf[5..].split(|x| *x == b'\r').next().unwrap();
    let relative = &part[..part.len() - 9];

    let path = if relative == b"/" {
        env.dir_ref().to_owned()
    } else {
        let mut path = env.dir_ref().to_owned();
        path.push(String::from_utf8_lossy(relative).as_ref());

        path
    };

    let path_str = path.to_str().unwrap().trim();

    if env.index() && path_str.ends_with('/') {
        return index::index(stream, path).map_err(|source| RequestError::Index { source });
    }

    get::get(stream, path).map_err(|source| RequestError::Get { source })
}

#[cfg(test)]
mod tests {
    use super::RequestedMethod;

    #[test]
    fn test_method() {
        assert_eq!(
            Some(RequestedMethod::Get),
            RequestedMethod::from_input(b"GET /test.html\r\n")
        );
    }
}
