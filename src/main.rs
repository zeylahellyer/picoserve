#![doc = include_str!("../README.md")]
#![deny(
    box_pointers,
    clippy::all,
    clippy::cargo,
    clippy::complexity,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    future_incompatible,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    non_ascii_idents,
    nonstandard_style,
    noop_method_call,
    rust_2018_compatibility,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_lifetimes,
    unused_results,
    unused,
    warnings
)]
// These lints would introduce less concise code.
#![allow(clippy::module_name_repetitions)]

mod content_type;
mod env;
mod request;
mod response;

use self::env::{Environment, EnvironmentError};
use core::fmt::{Display, Formatter, Result as FmtResult};
use std::{error::Error, io::Error as IoError, net::TcpListener};

#[derive(Debug)]
enum ApplicationError {
    LoadingEnvironment { source: EnvironmentError },
    TcpBinding { source: IoError },
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::LoadingEnvironment { .. } => f.write_str("failed to load from environment"),
            Self::TcpBinding { .. } => f.write_str("failed to bind to host and port"),
        }
    }
}

impl Error for ApplicationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::LoadingEnvironment { source } => Some(source),
            Self::TcpBinding { source } => Some(source),
        }
    }
}

fn main() -> Result<(), ApplicationError> {
    let env =
        Environment::new().map_err(|source| ApplicationError::LoadingEnvironment { source })?;

    let listener = TcpListener::bind((*env.host_ref(), env.port()))
        .map_err(|source| ApplicationError::TcpBinding { source })?;
    println!("= Listening on {}:{}", env.host_ref(), env.port());
    println!("= Serving {}", env.dir_ref().display());

    if env.index() {
        println!("= Indexing directories for browser file listing");
    }

    for stream in listener.incoming() {
        if let Err(why) = request::handle(&mut stream.unwrap(), &env) {
            eprintln!("Failed to handle stream: {:?}", why);
        }
    }

    Ok(())
}
