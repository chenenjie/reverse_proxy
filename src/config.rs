use std::collections::HashMap;
use std::env;
use std::fs::File;
use toml;
use error;
use std::io::Read;
use regex::Regex;

#[derive(Deserialize, Debug, Clone)]
struct Config{
    pub routes: HashMap<String, String>,
}

struct MappingRoute{
    pub routes: HashMap<String, Regex>,
}


pub fn get_config() -> error::Result<HashMap<String, Regex>> {
    let mut file_str = env::current_dir()?;
    file_str.push("proxy.toml");
    let mut file = File::open(file_str)?;
    let mut content = String::new();
    file.read_to_string(&mut content);

    let config: Config = toml::from_str(&content)?;

    let routes = config.routes.iter().map(|(key, value)|{
        //println!("({}{}", key, r"/(.*))");
        (value.clone(), Regex::new(&format!("({}{}", key, r"/(.*))|\z")).unwrap())
    }).collect();

    //let map_route = MappingRoute{
        //routes: routes,
    //};
    Ok(routes)
}



#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_toml(){
        let result = get_config();
        println!("{:?}", result);
    }
}
