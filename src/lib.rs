
#[macro_use]
extern crate error_chain;
extern crate hyper;
extern crate tokio_core; 
extern crate futures;
extern crate log;
extern crate env_logger;
extern crate regex;
extern crate toml;
extern crate serde;
#[macro_use]
extern crate serde_derive;
//#[macro_use]
//extern crate serde_derive;
//extern crate toml;
//extern crate tokio_signal;
//extern crate tokio_tls;
//extern crate tokio_service;
//extern crate native_tls;
//extern crate chrono;
//extern crate regex;

use tokio_core::reactor::{Core, Handle};
use tokio_core::net::TcpListener;
use hyper::server::Http;
use futures::Future;
use futures::Stream;

static CONFIG_FILE_NAME: &'static str = "proxy.toml";

mod error;
mod proxy;
mod config;

pub fn run() -> error::Result<()> {
    let mut core = Core::new()?;
    let handle = core.handle();

    let http = Http::new();

    env_logger::init().unwrap();
    
    let addr = "0.0.0.0:1024".parse()?;
    let sock = TcpListener::bind(&addr, &handle)?;

    let client = hyper::Client::new(&handle);

    let server = sock.incoming().for_each(|(sock, remote_addr)|{
        let service = proxy::Proxy{client: client.clone()};
        futures::future::ok(remote_addr).and_then(|remote_addr| {
            http.bind_connection(&handle, sock, remote_addr, service);
            Ok(())
        })
    });
    core.run(server);
    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        
    }
}
