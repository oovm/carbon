use crate::Config;

#[derive(Debug)]
pub enum CarbonError {
    ThemeNotFound(Box<str>),
    SyntaxNotFound(Box<str>),
}

impl CarbonError {
    pub fn no_theme(cfg: &Config) -> Self {
        CarbonError::ThemeNotFound(cfg.theme.clone().into_boxed_str())
    }
    pub fn no_syntax(cfg: &Config) -> Self {
        CarbonError::SyntaxNotFound(cfg.syntax.clone().into_boxed_str())
    }
}
