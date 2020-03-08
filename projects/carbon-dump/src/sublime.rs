use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::Write,
    iter::FromIterator,
};
use syntect::{
    dumps::*,
    highlighting::ThemeSet,
    parsing::{SyntaxSet, SyntaxSetBuilder},
};

pub fn dump_languages(from_dir: &str, into_file: &str) -> Result<(), Box<dyn Error>> {
    let mut builder = SyntaxSetBuilder::new();
    builder.add_plain_text_syntax();
    builder.add_from_folder(from_dir, true)?;
    dump_to_file(&builder.build(), into_file)?;
    Ok(())
}

pub fn dump_themes(from_dir: &str, into_file: &str) -> Result<(), Box<dyn Error>> {
    let ts = ThemeSet::load_from_folder(from_dir)?;
    dump_to_file(&ts, into_file)?;
    Ok(())
}

pub fn write_readme() -> std::io::Result<()> {
    let syntax: SyntaxSet = from_binary(include_bytes!("../languages.dump"));
    let theme: ThemeSet = from_binary(include_bytes!("../themes.dump"));
    let mut readme = String::with_capacity(1024);
    readme.push_str("# Carbon\n");
    readme.push_str(&format!("## Supported languages ({})\n", syntax.syntaxes().len()));
    let mut syntax_map: HashMap<String, HashSet<String>> = HashMap::new();
    for s in syntax.syntaxes() {
        syntax_map
            .entry(s.name.clone())
            .and_modify(|e| {
                for ext in &s.file_extensions {
                    e.insert(ext.clone());
                }
            })
            .or_insert_with(|| HashSet::from_iter(s.file_extensions.iter().cloned()));
    }
    let mut keys = syntax_map.keys().collect::<Vec<_>>();
    keys.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    for k in keys {
        if !syntax_map[k].is_empty() {
            let mut extensions_sorted = syntax_map[k].iter().cloned().collect::<Vec<_>>();
            extensions_sorted.sort();
            readme.push_str(&format!("- **{}:** {}\n", k, extensions_sorted.join(", ")))
        }
    }
    readme.push_str(&format!("## Supported themes ({})\n", theme.themes.len()));
    let mut keys = theme.themes.keys().collect::<Vec<_>>();
    keys.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    for k in keys {
        if let Some(name) = theme.themes[k].name.as_ref() {
            readme.push_str(&format!("- **{}:** {}\n", name, k))
        }
    }
    let mut file = File::create("readme.md")?;
    println!("{}", readme);
    file.write_all(readme.as_bytes())?;
    Ok(())
}

#[test]
fn main() {
    dump_languages("languages", "languages.dump").unwrap();
    dump_themes("themes", "themes.dump").unwrap();
    write_readme().unwrap();
}
