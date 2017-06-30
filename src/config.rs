use std::collections::HashMap;
use std::env;
use std::fs::File;
use toml;
use error;
use std::io::Read;

#[derive(Deserialize, Debug, Clone)]
struct Config{
    pub routes: HashMap<String, String>,
}


fn get_config() -> error::Result<Config> {
    let mut file_str = env::current_dir()?;
    file_str.push("proxy.toml");
    let mut file = File::open(file_str)?;
    let mut content = String::new();
    file.read_to_string(&mut content);

    let config: Config = toml::from_str(&content)?;
    Ok(config)
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
