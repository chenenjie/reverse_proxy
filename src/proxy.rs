use futures::Future;
use futures::future;
use hyper::server::Service;
use hyper::server::Request;
use hyper::server::Response;
use hyper::{Client, StatusCode, Body};
use hyper;
use hyper::client::HttpConnector;
use std::str::FromStr;
use hyper::Uri;
use regex::Regex;
use std::collections::HashMap;


pub struct Proxy {
    pub client: Client<HttpConnector, Body>,
    pub routes: HashMap<String, Regex>
}


impl Proxy {
    fn fetch_tail_url(&self, path :&str) -> Option<(String, Option<String>)> {
        let mut result : Vec<Option<(String, Option<String>)>>= self.routes.iter().map(|(host, regex)| {
            if let Some(m) = regex.captures(path){
                let site_url = match m.get(2) {
                    Some(url) => Some(String::from(url.as_str())),
                    None => None,
                };
                return Some((host.to_owned(), site_url));
            };
            None
        }).collect();       

        println!("{:?}", result);
        if result.len() != 0 {
            result.remove(0)
        }else {
            None
        }
    }
}

impl Service for Proxy{
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = hyper::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let uri = req.uri();
            
        println!("uri .path {:?}", uri.path());
        //let url = Uri::from_str("http://localhost:8080").unwrap();
        //println!("{:?}", url);
        let url = match self.fetch_tail_url(uri.path()){
            Some((addr, site_url)) => {
                let complete_url = format!("{}{}", addr, site_url.unwrap_or("".to_owned()));
                println!("complete_url {}", complete_url);
                complete_url.parse().expect("translate url is unavaiable")
            },
            _ => uri.clone() 
        };

        let mut proxied_request = hyper::client::Request::new(req.method().clone(), url);
        let mut proxyied_headers = req.headers().clone();
        proxyied_headers.remove_raw("host");
        *proxied_request.headers_mut() = proxyied_headers;
        let req = self.client.request(proxied_request); 

        Box::new(req.then(|res| {
            println!("got response back! {:?}", res);
            if let Ok(res) = res{
                future::ok(
                    Response::new()
                        .with_status(res.status().clone())
                        .with_headers(res.headers().clone())
                        .with_body(res.body()))
            } else {
                future::ok(
                    Response::new().with_status(StatusCode::ServiceUnavailable))
            }
        })) as Self::Future
    }
}


