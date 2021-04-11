use crate::response::{Response, WriteError};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult, Write as _},
    fs,
    io::Error as IoError,
    net::{SocketAddr, TcpStream},
    path::PathBuf,
};

#[derive(Debug)]
pub enum IndexError {
    ReadingDirectory {
        path: PathBuf,
        source: IoError,
    },
    ReadingEntry {
        source: IoError,
    },
    ReadingMetadata {
        path: PathBuf,
        source: IoError,
    },
    WritingToStream {
        buf: String,
        remote_ip: Option<SocketAddr>,
        source: WriteError,
    },
}

impl Display for IndexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::ReadingDirectory { path, .. } => {
                f.write_str("failed to read directory '")?;
                f.write_str(&path.to_string_lossy())?;
                f.write_char('\'')
            }
            Self::ReadingEntry { .. } => f.write_str("failed to read entry data in directory"),
            Self::ReadingMetadata { path, .. } => {
                f.write_str("failed to read metadata for path '")?;
                f.write_str(&path.to_string_lossy())?;
                f.write_char('\'')
            }
            Self::WritingToStream { buf, remote_ip, .. } => {
                f.write_str("failed to write to remote (")?;

                if let Some(ip) = remote_ip {
                    f.write_char('\'')?;
                    ip.fmt(f)?;
                    f.write_char('\'')?;
                } else {
                    f.write_str("unknown")?;
                }

                f.write_str("); content: ")?;
                f.write_str(buf)
            }
        }
    }
}

impl Error for IndexError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ReadingDirectory { source, .. } => Some(source),
            Self::ReadingEntry { source, .. } => Some(source),
            Self::ReadingMetadata { source, .. } => Some(source),
            Self::WritingToStream { source, .. } => Some(source),
        }
    }
}

pub fn index(stream: &mut TcpStream, path: PathBuf) -> Result<(), IndexError> {
    let mut buf = String::new();

    let mut dirs = Vec::new();
    let mut files = Vec::new();

    let dir =
        fs::read_dir(&path).map_err(|source| IndexError::ReadingDirectory { path, source })?;

    for entry in dir {
        let entry = entry.map_err(|source| IndexError::ReadingEntry { source })?;

        let metadata = entry
            .metadata()
            .map_err(|source| IndexError::ReadingMetadata {
                path: entry.path(),
                source,
            })?;
        let filename = entry.file_name();

        let name = if let Some(name) = filename.as_os_str().to_str() {
            name
        } else {
            continue;
        };

        // Skip hidden directories, `.`, and `..`.
        if name.starts_with('.') {
            continue;
        }

        if metadata.is_dir() {
            dirs.push(name.to_owned());
        } else {
            files.push(name.to_owned());
        }
    }

    dirs.sort();
    files.sort();

    if !dirs.is_empty() {
        buf.push_str("<h2>directories</h2>");

        for dir in dirs {
            write_anchor(&mut buf, &dir);
        }
    }

    if !files.is_empty() {
        buf.push_str("<h2>files</h2>");

        for file in files {
            write_anchor(&mut buf, &file);
        }
    }

    Response::new(buf.as_bytes())
        .ok()
        .write(stream)
        .map_err(|source| IndexError::WritingToStream {
            buf,
            remote_ip: stream.peer_addr().ok(),
            source,
        })
}

fn write_anchor(buf: &mut String, path: &str) {
    buf.push_str("<a href='./");
    buf.push_str(&path);
    buf.push_str("'>");
    buf.push_str(&path);
    buf.push_str("</a><br />");
}
