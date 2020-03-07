lazy_static::lazy_static! {
    pub static ref SYNTAX_SET: SyntaxSet = from_binary(include_bytes!("languages.dump"));
    pub static ref THEME_SET: ThemeSet = from_binary(include_bytes!("themes.dump"));
}