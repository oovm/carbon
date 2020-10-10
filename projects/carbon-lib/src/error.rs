#[derive(Debug)]
pub enum CarbonError {
    ThemeNotFound(String),
    SyntaxNotFound(String),
}

pub type CarbonResult<T> = Result<T, CarbonError>;

impl CarbonError {}
