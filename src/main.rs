//! # picoserve
//!
//! picoserve is a simple 0-dependency HTTP server that just serves files. There's
//! no authentication, no caching control, no uploading, and so on. You can just
//! GET files and, optionally, index directories for viewing.
//!
//! ## Installation
//!
//! ```sh
//! $ cargo install --git https://github.com/zeylahellyer/picoserve
//! ```
//!
//! ## Usage
//!
//! Start the pico server, which will by default serve the current directory and
//! bind to `127.0.0.1:5555`:
//!
//! ```sh
//! $ picoserve
//! = Listening on 127.0.0.1:5555
//! = Serving /Users/zeyla/dev/picoserve
//! ```
//!
//! You can then GET files:
//!
//! ```sh
//! $ http get 127.0.0.1:5555/Cargo.toml
//! HTTP/1.1 200 OK
//! Content-Length: 479
//! Content-Type: text/plain
//! Server: picoserve/0.1.0
//!
//! [package]
//! authors = ["Zeyla Hellyer <zeyla@hellyer.dev>"]
//! categories = ["command-line-utilities"]
//! description = "Get setup with serving a directory quickly."
//! documentation = "https://github.com/zeylahellyer/picoserve"
//! edition = "2018"
//! include = ["src/**/*.rs", "Cargo.toml"]
//! keywords = ["http server"]
//! license = "ISC"
//! name = "picoserve"
//! readme = "README.md"
//! repository = "https://github.com/zeylahellyer/picoserve.git"
//! version = "0.1.0"
//!
//! [dev-dependencies]
//! rusty-hook = "0.11.2"
//! ```
//!
//! You can index directories for browser viewing on port 8080:
//!
//! ```sh
//! $ picoserve --index --port 8080
//! = Listening on 127.0.0.1:8080
//! = Serving /Users/zeyla/dev/picoserve
//! = Indexing directories for browser file listing
//! ```
//!
//! ![Index in the browser](README-index.png)
//!
//! ### Flags
//!
//! - `--host <value>`: set the host to bind to
//! - `--port <value>`: set the port to bind to
//! - `--dir <value>`: path to the directory to serve
//! - `--index`: enable an index for browser viewing
//!
//! ## License
//!
//! ISC.

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
    disjoint_capture_drop_reorder,
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
// These lints would require a higher MSRV.
#![allow(clippy::missing_const_for_fn)]
// These lints would introduce less concise code.
#![allow(clippy::module_name_repetitions)]

mod content_type;
mod env;
mod request;
mod response;

use self::env::{Environment, EnvironmentError};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as IoError,
    net::TcpListener,
};

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

impl Error for ApplicationError {}

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
