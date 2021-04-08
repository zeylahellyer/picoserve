mod env;
mod extension;
mod index;
mod response;

use env::Environment;
use response::Response;
use std::{
    error::Error,
    ffi::OsStr,
    fs,
    io::{ErrorKind, Read},
    net::{TcpListener, TcpStream},
};

fn main() -> Result<(), Box<dyn Error>> {
    let env = Environment::new()?;

    let listener = TcpListener::bind((*env.host_ref(), env.port()))?;
    println!("= Listening on {}:{}", env.host_ref(), env.port());
    println!("= Serving {}", env.dir_ref().display());

    if env.index() {
        println!("= Indexing directories for browser file listing");
    }

    for stream in listener.incoming() {
        if let Err(why) = handle_stream(stream.unwrap(), &env) {
            eprintln!("Failed to handle stream: {:?}", why);
        }
    }

    Ok(())
}

fn handle_stream(
    mut stream: TcpStream,
    env: &Environment,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut buf = [0; 1024];

    let _ = stream.read(&mut buf)?;

    if !buf.starts_with(b"GET /") {
        Response::new(b"")
            .method_not_allowed(&[b"GET"])
            .write(&mut stream)?;

        return Ok(());
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
        return index::list(stream, path).map_err(From::from);
    }

    match fs::read(&path) {
        Ok(bytes) => Response::new(&bytes)
            .extension(path.extension().and_then(OsStr::to_str))
            .ok()
            .write(&mut stream)
            .map_err(From::from),
        Err(source) => {
            let response = Response::new(b"");

            let response = match source.kind() {
                ErrorKind::Other if source.raw_os_error() == Some(21) => response.forbidden(),
                ErrorKind::NotFound => response.not_found(),
                _ => response.internal_service_error(),
            };

            response.write(&mut stream).map_err(From::from)
        }
    }
}
