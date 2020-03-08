use crate::{CarbonError, Config};
use carbon_dump::{SYNTAX_SET, THEME_SET};
use std::ops::Deref;
use syntect::{
    easy::HighlightLines,
    highlighting::{Color, Style},
    util::{as_24_bit_terminal_escaped, as_latex_escaped, LinesWithEndings},
};
use crate::utils::{CarbonHTML, html_render_line, html_render_line_number};

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
        let syntax = SYNTAX_SET.find_syntax_by_extension(&self.syntax).ok_or(CarbonError::no_theme(self))?;
        let theme = THEME_SET.themes.get(&self.theme).ok_or(CarbonError::no_theme(self))?;
        // The main process of the program
        let mut out = String::with_capacity(25 * input.len());
        let c = theme.settings.background.unwrap_or(Color::WHITE);
        let render = if self.line_number {  html_render_line_number} else { html_render_line };
        match self.html_type {
            CarbonHTML::Embedded => {
                match &self.file_title {
                    None => {
                        out.push_str(&render(input, syntax, theme));
                    }
                    Some(s) => {
                        out.push_str(&format!(
                            "<div class=\"carbon\" style=\"background-color:#{:02x}{:02x}{:02x};\">\n",
                            c.r, c.g, c.b
                        ));
                        out.push_str(&render(input, syntax, theme));
                        out.push_str(&format!("</div>"));
                    }
                }
            }
            CarbonHTML::Independent => {
                out.push_str(&format!("<head><style>pre{{{}}}</style></head>", self.html_font));
                out.push_str(&format!("<body style=\"background-color:#{:02x}{:02x}{:02x};\">\n", c.r, c.g, c.b));
                out.push_str(&format!("{}", render(input, syntax, theme)));
                out.push_str(&format!("</body>"));
            }
        };
        return Ok(out);
    }
}