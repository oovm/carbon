use carbon_dump::{SYNTAX_SET, THEME_SET};

mod error;
pub mod utils;

pub use error::CarbonError;
pub use utils::Config;

fn main() {
    let mut cfg = Config::default();
    cfg.syntax = String::from("rs");
    match cfg.render_terminal("pub struct Wow { hi: u64 }\nfn blah() -> u64 {}") {
        Ok(o) => {
            println!("Render Terminal");
            println!("{}", o)
        }
        Err(e) => println!("Error: {:?}", e),
    };
}
