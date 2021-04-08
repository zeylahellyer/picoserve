mod env;
mod index;
mod mime;
mod response;

use env::Environment;
use response::Status;
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
        response::write(&mut stream, Status::BadRequest, b"", None)?;

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
        Ok(bytes) => response::write(
            &mut stream,
            Status::Ok,
            &bytes,
            path.extension().and_then(OsStr::to_str),
        )
        .map_err(From::from),
        Err(source) => {
            let code = match source.kind() {
                ErrorKind::Other if source.raw_os_error() == Some(21) => Status::Forbidden,
                ErrorKind::NotFound => Status::NotFound,
                _ => Status::InternalServiceError,
            };

            response::write(&mut stream, code, b"", None).map_err(From::from)
        }
    }
}
