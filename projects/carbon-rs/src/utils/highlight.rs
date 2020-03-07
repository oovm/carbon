use crate::{CarbonError, Config};
use carbon_dump::{SYNTAX_SET, THEME_SET};
use lazy_static::lazy_static;
use std::ops::Deref;
use syntect::{
    dumps::from_binary,
    easy::HighlightLines,
    highlighting::{Color, Style, ThemeSet},
    html::{highlighted_html_for_file, highlighted_html_for_string},
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, as_latex_escaped, LinesWithEndings},
};

fn load() -> (&'static SyntaxSet, &'static ThemeSet) {
    let themes = &THEME_SET.themes;
    (SYNTAX_SET.deref(), THEME_SET.deref())
}

impl Config {
    pub fn render_terminal(&self, input: &str) -> Result<String, CarbonError> {
        let set = SYNTAX_SET.deref();
        let syntax = set.find_syntax_by_extension(&self.syntax).ok_or(CarbonError::no_theme(self))?;
        let theme = THEME_SET.themes.get(&self.theme).ok_or(CarbonError::no_theme(self))?;
        // The main process of the program
        let mut h = HighlightLines::new(syntax, theme);
        let mut out = String::with_capacity(2 * input.len());
        for line in LinesWithEndings::from(input) {
            let ranges: Vec<(Style, &str)> = h.highlight(line, set);
            let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
            out.push_str(&escaped)
        }
        return Ok(out);
    }

    pub fn render_latex(&self, input: &str) -> Result<String, CarbonError> {
        let set = SYNTAX_SET.deref();
        let syntax = set.find_syntax_by_extension(&self.syntax).ok_or(CarbonError::no_theme(self))?;
        let theme = THEME_SET.themes.get(&self.theme).ok_or(CarbonError::no_theme(self))?;
        // The main process of the program
        let mut h = HighlightLines::new(syntax, theme);
        let mut out = String::with_capacity(2 * input.len());
        for line in LinesWithEndings::from(input) {
            // LinesWithEndings enables use of newlines mode
            let ranges: Vec<(Style, &str)> = h.highlight(line, set);
            let escaped = as_latex_escaped(&ranges[..]);
            out.push_str(&escaped);
            out.push_str("\n\n")
        }
        return Ok(out);
    }
}

#[test]
fn html() {
    let (ps, ts) = load();
    let cfg = Config::default();
    let s = "pub struct Wow { hi: u64 }\nfn blah() -> u64 {}\n";
    let style = "
        pre {
            font-size:13px;
            font-family: Consolas, \"Liberation Mono\", Menlo, Courier, monospace;
        }";
    println!("<head><title>{}</title><style>{}</style></head>", s, style);
    let theme = &ts.themes[&cfg.theme];
    let c = theme.settings.background.unwrap_or(Color::WHITE);
    println!("<body style=\"background-color:#{:02x}{:02x}{:02x};\">\n", c.r, c.g, c.b);
    let html = highlighted_html_for_string(s, &ps, theme).unwrap();
    println!("{}", html);
    println!("</body>");
}
