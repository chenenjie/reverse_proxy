extern crate regex;

use regex::RegexSet;
use regex::Regex;

fn main() {
    let set = RegexSet::new(&[
        r"@(.*)\.com",
    ]).unwrap();

    let re = Regex::new(r"(/eern/(.*))|\z").unwrap();
    let re1 = Regex::new(r"wfefn").unwrap();
    let re2 = Regex::new(r"(/eern/(.*))|\z").unwrap();
    let re3 = Regex::new(r"(/eern/(.*))|\z").unwrap();

    //if let Some(cap) = re.captures("localhost/eern/enjie/fe.html") {
        //let site_url = match cap.get(2) {
            //Some(m) => m.as_str(),
            //None => "",
        //}; 
        //println!("{:?}", site_url);
    //};
    let routes = vec![re, re1, re2, re3];

    let result: Vec<Option<&str>> = routes.iter().map(|regex| {
        if let Some(m) = regex.captures("localhost/eern/enjie/fe.html"){
            let site_url = match m.get(2) {
                Some(url) => Some(url.as_str()),
                None => None,
            };
            return site_url;
        }
        None
    }).collect();       

    println!("{:?}", result);

    //let matchs = set.matches("foo@example.com");
    //println!("{:?}", matchs);

    //let index = matchs.iter().next().unwrap();
    //println!("{}", index);

}
