#[macro_use]
extern crate error_chain;

mod container{
    error_chain!{
        errors {
            InvalidRoute(r: String) {
                description("invalid route specified")
                display("invalid route specified: '{}'", r)
            }
        }
    }
}

fn main() {
    // run().fuck();   
    println!("{:?}", run());
}

fn run() -> container::Result<()> {
    if 1 != 2 {
        //不填写kind默认是Msg
        bail!("enjie");
    }
    bail!(container::ErrorKind::InvalidRoute("enjie".to_owned()));

}