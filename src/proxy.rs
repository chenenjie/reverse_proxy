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


pub struct Proxy {
    //pub routes: Vec<(String, Regex)>,
    pub client: Client<HttpConnector, Body>,
}


impl Proxy {
    //fn fetch_tail_url(&self, path :&str) -> Option<String> {
        //let result = self.routes.iter().map(|(_, regex)| {
            //if let Some(m) = regex.captures(path){
                //let site_url = match m.get(2) {
                    //Some(url) => Ok(m.as_str()),
                    //None => None,
                //};
                //return site_url;
            //};
            //None
        //}).collect();       

        //if result.len() != 0 {
            //result.get(0)
        //}else {
            //None
        //}
    //}
}

impl Service for Proxy{
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = hyper::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let uri = req.uri();
            
        let url = Uri::from_str("http://localhost:8080").unwrap();
        println!("{:?}", url);

        let mut proxied_request = hyper::client::Request::new(req.method().clone(), url);
        *proxied_request.headers_mut() = req.headers().clone();
        let req = self.client.request(proxied_request); 

        Box::new(req.then(|res| {
            println!("got response back!");
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


