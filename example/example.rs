
#[macro_use] extern crate failure;
#[macro_use] extern crate log;
extern crate env_logger;

#[derive(Debug, Fail)]
enum ExampleError {
    #[fail(display = "Something bad happened: {}", details)]
    Miscellaneous {
        details: String,
    }
}


fn main() {
    env_logger::init().unwrap();

    info!("All is well."); 
}
