pub struct Config {
    pub theme: String,
    pub syntax: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            //
            syntax: String::from("rs"),
            theme: String::from("one-dark"),
        }
    }
}
