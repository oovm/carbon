pub struct Config {
    pub theme: String,
    pub syntax: String,
    pub html_font: String,
    pub html_type: CarbonHTML,
    pub file_title: Option<String>,
    pub line_number : bool
}

pub enum CarbonHTML {
    /// single
    Embedded,
    /// Full html
    Independent,
}

const FONT_CSS: &str = r#"
font-size: 14px
font-family: "Fira Code", Consolas, Monaco, Menlo, Consolas, monospace;
"#;

impl Default for Config {
    fn default() -> Self {
        Self {
            //
            syntax: String::from("rs"),
            theme: String::from("nyx-bold"),
            html_font: String::from(FONT_CSS.replace('\n', "")),
            html_type: CarbonHTML::Independent,
            file_title: None,
            line_number: false
        }
    }
}
