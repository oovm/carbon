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
        let mut out = String::with_capacity(10 * input.len());
        for line in LinesWithEndings::from(input) {
            // LinesWithEndings enables use of newlines mode
            let ranges: Vec<(Style, &str)> = h.highlight(line, set);
            let escaped = as_latex_escaped(&ranges[..]);
            out.push_str(&escaped);
            out.push_str("\n\n")
        }
        return Ok(out);
    }
    pub fn render_html(&self, input: &str) -> Result<String, CarbonError> {
        let all = SYNTAX_SET.deref();
        let syntax = all.find_syntax_by_extension(&self.syntax).ok_or(CarbonError::no_theme(self))?;
        let theme = THEME_SET.themes.get(&self.theme).ok_or(CarbonError::no_theme(self))?;
        // The main process of the program
        let mut out = String::with_capacity(25 * input.len());
        let style = "
        pre {
            font-size:13px;
            font-family: Consolas, \"Liberation Mono\", Menlo, Courier, monospace;
        }";
        out.push_str(&format!("<head><title>{}</title><style>{}</style></head>", input, style));

        let c = theme.settings.background.unwrap_or(Color::WHITE);
        out.push_str(&format!("<body style=\"background-color:#{:02x}{:02x}{:02x};\">\n", c.r, c.g, c.b));
        let html = highlighted_html_for_string(input, all, syntax, theme);
        out.push_str(&format!("{}", html));
        out.push_str(&format!("</body>"));
        return Ok(out);
    }
}
