use crate::{
    utils::{html_render_line, html_render_line_number, CarbonHTML},
    CarbonError, Render,
};
use carbon_dump::SYNTAX_SET;
use std::ops::Deref;
use syntect::{
    easy::HighlightLines,
    highlighting::{Color, Style},
    util::{as_24_bit_terminal_escaped, as_latex_escaped, LinesWithEndings},
};

impl Render {
    pub fn render_terminal(&self, input: &str) -> Result<String, CarbonError> {
        let set = SYNTAX_SET.deref();
        // The main process of the program
        let mut h = HighlightLines::new(&self.syntax, &self.theme);
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
        // The main process of the program
        let mut h = HighlightLines::new(&self.syntax, &self.theme);
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
        // The main process of the program
        let mut out = String::with_capacity(25 * input.len());
        let c = self.theme.settings.background.unwrap_or(Color::WHITE);
        match self.html_type {
            CarbonHTML::Inline => {
                out.push_str("<pre class=\"carbon\">");
                out.push_str(&self.render_check_line_number(input));
                out.push_str(&format!("</pre>"));
            }
            CarbonHTML::Embedded => match &self.file_title {
                None => {
                    out.push_str(&self.render_check_line_number(input));
                }
                Some(s) => {
                    out.push_str(&format!(
                        "<div class=\"carbon\" style=\"background-color:#{:02x}{:02x}{:02x};\">\n",
                        c.r, c.g, c.b
                    ));
                    out.push_str(&self.render_check_line_number(input));
                    out.push_str(&format!("</div>"));
                }
            },
            CarbonHTML::Independent => {
                out.push_str(&format!("<head><style>pre{{{}}}</style></head>", self.html_style));
                out.push_str(&format!("<body style=\"background-color:#{:02x}{:02x}{:02x};\">\n", c.r, c.g, c.b));
                out.push_str(&format!("{}", self.render_check_line_number(input)));
                out.push_str(&format!("</body>"));
            }
        };
        return Ok(out);
    }
}

impl Render {
    fn render_check_line_number(&self, input: &str) -> String {
        match self.line_number {
            None => html_render_line(input, &self.syntax, &self.theme),
            Some(_) => html_render_line_number(input, &self.syntax, &self.theme),
        }
    }
}
