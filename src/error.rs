error_chain!{
    foreign_links {
        Io(::std::io::Error);
        AddrParse(::std::net::AddrParseError);
        Log(::log::SetLoggerError);
        Http(::hyper::Error);
        Toml(::toml::de::Error);
    }

    errors {
        InvalidRoute(r: String){
            description("invalid rout specified")
            display("invalid route specified: '{}'", r)
        }
    }
}
