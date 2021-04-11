use std::{
    env,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as IoError,
    net::{IpAddr, Ipv4Addr},
    num::ParseIntError,
    path::{Path, PathBuf},
    process,
};

const HELP: &str = concat!(
    env!("CARGO_PKG_NAME"),
    " ",
    env!("CARGO_PKG_VERSION"),
    "
",
    env!("CARGO_PKG_AUTHORS"),
    "
",
    env!("CARGO_PKG_DESCRIPTION"),
    "

FLAGS:

    --help          Print help information.
    --index         Enable indexing for browser directory viewing.

OPTIONS:

    --dir <PATH>    Path of the directory to serve.
    --host <IP>     IP address of the host to bind to.
    --port <NUMBER> Port to bind to."
);

#[derive(Debug)]
pub enum EnvironmentError {
    CurrentDirectoryInvalid { source: IoError },
    NoMatchingValue { name: String },
    PortNotInteger { port: String, source: ParseIntError },
}

impl Display for EnvironmentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::CurrentDirectoryInvalid { .. } => {
                f.write_str("current directory is invalid or does not exist")
            }
            Self::NoMatchingValue { name, .. } => {
                f.write_str("flag '")?;
                f.write_str(name)?;
                f.write_str("' has no matching value")
            }
            Self::PortNotInteger { port, .. } => {
                f.write_str("provided port '")?;
                f.write_str(port)?;
                f.write_str("' is not a valid integer")
            }
        }
    }
}

impl Error for EnvironmentError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::CurrentDirectoryInvalid { source } => Some(source),
            Self::NoMatchingValue { .. } => None,
            Self::PortNotInteger { source, .. } => Some(source),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Environment {
    dir: PathBuf,
    host: IpAddr,
    index: bool,
    port: u16,
}

impl Environment {
    pub fn new() -> Result<Self, EnvironmentError> {
        let mut args = env::args().skip(1);

        let mut dir: Option<PathBuf> = None;
        let mut host: Option<IpAddr> = None;
        let mut index = false;
        let mut port: Option<u16> = None;

        while let Some(name) = args.next() {
            match name.as_ref() {
                "--dir" => {
                    let value = value(&mut args, name)?;

                    dir.replace(PathBuf::from(value));
                }
                "--help" => {
                    println!("{}", HELP);

                    process::exit(0);
                }
                "--host" => {
                    let value = value(&mut args, name)?;

                    host.replace(value.parse().unwrap());
                }
                "--index" => {
                    index = true;
                }
                "--port" => {
                    let value = value(&mut args, name)?;

                    port.replace(value.parse().map_err(|source| {
                        EnvironmentError::PortNotInteger {
                            port: value,
                            source,
                        }
                    })?);
                }
                _ => {}
            }
        }

        let dir = match dir {
            Some(dir) => dir,
            None => env::current_dir()
                .map_err(|source| EnvironmentError::CurrentDirectoryInvalid { source })?,
        };

        Ok(Self {
            dir,
            host: host.unwrap_or_else(|| IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
            index,
            port: port.unwrap_or(5555),
        })
    }

    // Retrieve a reference to the directory to serve.
    pub fn dir_ref(&self) -> &Path {
        &self.dir
    }

    // Retrieve a reference to the IP address of the host to bind to.
    pub fn host_ref(&self) -> &IpAddr {
        &self.host
    }

    // Whether to serve an index.
    pub fn index(&self) -> bool {
        self.index
    }

    // Port to bind to.
    pub fn port(&self) -> u16 {
        self.port
    }
}

fn value(args: &mut dyn Iterator<Item = String>, name: String) -> Result<String, EnvironmentError> {
    args.next()
        .ok_or(EnvironmentError::NoMatchingValue { name })
}
