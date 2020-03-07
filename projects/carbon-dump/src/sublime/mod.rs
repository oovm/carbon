use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use syntect::dumps::*;
use syntect::highlighting::ThemeSet;
use syntect::parsing::{SyntaxSetBuilder, SyntaxSet};

pub fn dump_languages(package_dir: &str, packpath_newlines: &str) {
    let mut builder = SyntaxSetBuilder::new();
    builder.add_plain_text_syntax();
    builder.add_from_folder(package_dir, true).unwrap();
    let ss = builder.build();
    dump_to_file(&ss, packpath_newlines).unwrap();
    let mut syntaxes: HashMap<String, HashSet<String>> = HashMap::new();

    for s in ss.syntaxes().iter() {
        syntaxes
            .entry(s.name.clone())
            .and_modify(|e| {
                for ext in &s.file_extensions {
                    e.insert(ext.clone());
                }
            })
            .or_insert_with(|| HashSet::from_iter(s.file_extensions.iter().cloned()));
    }
    let mut keys = syntaxes.keys().collect::<Vec<_>>();
    keys.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    for k in keys {
        if !syntaxes[k].is_empty() {
            let mut extensions_sorted = syntaxes[k].iter().cloned().collect::<Vec<_>>();
            extensions_sorted.sort();
            println!("- {} -> {:?}", k, extensions_sorted);
        }
    }
}

pub fn dump_themes(theme_dir: &str, packpath: &str) {
    let ts = ThemeSet::load_from_folder(theme_dir).unwrap();
    for path in ts.themes.keys() {
        println!("- {:?}", path);
    }
    dump_to_file(&ts, packpath).unwrap();
}


pub fn write_readme() {
    let syntax: SyntaxSet = from_binary(include_bytes!("../../languages.dump"));
    let theme: ThemeSet = from_binary(include_bytes!("../../themes.dump"));
    let mut readme = String::with_capacity(1024);
    readme.push_str("# Carbon\n");
    readme.push_str("## Supported languages\n");
    for s in syntax.syntaxes() {
        readme.push_str(&format!("- **{}**: {}\n", s.name, s.file_extensions.join(", ")))
    }
    println!("{}", readme)
}

#[test]
fn main() {
    dump_languages("languages", "languages.dump");
    dump_themes("themes", "themes.dump");
    write_readme()
}
