pub use carbon_dump::{SYNTAX_SET, THEME_SET};

mod error;
mod highlight;
pub mod utils;

use crate::utils::CarbonHTML;
pub use error::{CarbonError, CarbonResult};
pub use utils::Render;

#[test]
fn main() {
    let render = Render::default();
    // render.set_syntax("rs");
    const TEST: &str = include_str!("lib.rs");
    match render.render_html(TEST) {
        Ok(o) => {
            println!("Render HTML:");
            println!("{}", o)
        }
        Err(e) => println!("Error: {:?}", e),
    };
    match render.render_latex(TEST) {
        Ok(o) => {
            println!("Render LaTeX:");
            println!("{}", o)
        }
        Err(e) => println!("Error: {:?}", e),
    };
    match render.render_terminal(TEST) {
        Ok(o) => {
            println!("Render Terminal:");
            println!("{}", o)
        }
        Err(e) => println!("Error: {:?}", e),
    };
}

#[test]
fn html() {
    let mut render = Render::default();
    // render.set_syntax("rs");
    render.html_type = CarbonHTML::Embedded;
    render.line_number = Some(1);
    const TEST: &str = include_str!("lib.rs");
    match render.render_html(TEST) {
        Ok(o) => println!("{}", o),
        Err(e) => println!("Error: {:?}", e),
    };
}
