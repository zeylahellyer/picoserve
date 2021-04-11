mod content_type;
mod env;
mod request;
mod response;

use env::Environment;
use std::{error::Error, net::TcpListener};

fn main() -> Result<(), Box<dyn Error>> {
    let env = Environment::new()?;

    let listener = TcpListener::bind((*env.host_ref(), env.port()))?;
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
