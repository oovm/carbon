use crate::{CarbonError, CarbonResult};
use carbon_dump::{SYNTAX_SET, THEME_SET};
use syntect::{highlighting::Theme, parsing::SyntaxReference};

pub struct Render {
    pub(crate) theme: Theme,
    pub(crate) syntax: SyntaxReference,
    pub html_style: String,
    pub html_type: CarbonHTML,
    pub file_title: Option<String>,
    pub line_number: Option<usize>,
}

pub enum CarbonHTML {
    /// inline
    Inline,
    /// single
    Embedded,
    /// Full html
    Independent,
}

impl Default for Render {
    fn default() -> Self {
        let theme = THEME_SET.themes.get("one-dark").unwrap();
        let syntax = SYNTAX_SET.find_syntax_by_extension("rs").unwrap();
        Self {
            syntax: syntax.clone(),
            theme: theme.clone(),
            html_style: String::from(include_str!("render.css")),
            html_type: CarbonHTML::Independent,
            file_title: None,
            line_number: None,
        }
    }
}

impl Render {
    pub fn set_theme(&mut self, s: &str) -> CarbonResult<()> {
        let theme = THEME_SET.themes.get(s).ok_or(CarbonError::ThemeNotFound(s.to_string()))?;
        self.theme = theme.clone();
        Ok(())
    }

    pub fn set_syntax(&mut self, s: &str) -> CarbonResult<()> {
        let syntax = SYNTAX_SET.find_syntax_by_extension(s).ok_or(CarbonError::SyntaxNotFound(s.to_string()))?;
        self.syntax = syntax.clone();
        Ok(())
    }
}
