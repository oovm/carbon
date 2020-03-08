use carbon_dump::{SYNTAX_SET, THEME_SET};

mod highlight;
mod error;
pub mod utils;

pub use error::CarbonError;
pub use utils::Config;
use crate::utils::CarbonHTML;

#[test]
fn main() {
    let mut cfg = Config::default();
    cfg.syntax = String::from("rs");
    const TEST: &str = include_str!("lib.rs");
    match cfg.render_html(TEST) {
        Ok(o) => {
            println!("Render HTML:");
            println!("{}", o)
        }
        Err(e) => println!("Error: {:?}", e),
    };
    match cfg.render_latex(TEST) {
        Ok(o) => {
            println!("Render LaTeX:");
            println!("{}", o)
        }
        Err(e) => println!("Error: {:?}", e),
    };
    match cfg.render_terminal(TEST) {
        Ok(o) => {
            println!("Render Terminal:");
            println!("{}", o)
        }
        Err(e) => println!("Error: {:?}", e),
    };
}

#[test]
fn html() {
    let mut cfg = Config::default();
    cfg.syntax = String::from("rs");
    cfg.html_type = CarbonHTML::Embedded;
    cfg.line_number = true;
    const TEST: &str = include_str!("lib.rs");
    match cfg.render_html(TEST) {
        Ok(o) => {
            println!("{}", o)
        }
        Err(e) => println!("Error: {:?}", e),
    };
}
