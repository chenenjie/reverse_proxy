extern crate regex;

use regex::RegexSet;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let set = RegexSet::new(&[
        r"@(.*)\.com",
    ]).unwrap();
    let pattern = format!("{}{}", "(/eern/", r"(.*))|\z");
    let mut map = HashMap::new();
    map.insert("url1", r"(/eern/(.*))|\z");
    map.insert("url2", r"(/eern/(.*))|\z");
    map.insert("url3", r"(/eern/(.*))|\z");
    map.insert("url4", pattern.as_str());

    let translate_regex: HashMap<&str, Regex> = map.iter().map(|(key, value)|{
        (key.to_owned(), Regex::new(value).unwrap())
    }).collect();
    //translate_regex.get("url1").fuck();

    println!("{:?}", translate_regex);

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

    //println!("{:?}", result);

    //let matchs = set.matches("foo@example.com");
    //println!("{:?}", matchs);

    //let index = matchs.iter().next().unwrap();
    //println!("{}", index);

}
