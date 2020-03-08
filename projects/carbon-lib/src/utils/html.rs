use syntect::parsing::{SyntaxSet, SyntaxReference};
use syntect::highlighting::{Theme, Style};
use syntect::html::{start_highlighted_html_snippet, IncludeBackground, append_highlighted_html_for_styled_line, styled_line_to_highlighted_html};
use syntect::easy::HighlightLines;
use syntect::util::LinesWithEndings;
use carbon_dump::{SYNTAX_SET, THEME_SET};
use std::ops::Deref;

pub fn html_render_line(s: &str, syntax: &SyntaxReference, theme: &Theme) -> String {
    let mut highlighter = HighlightLines::new(syntax, theme);
    let (mut output, bg) = start_highlighted_html_snippet(theme);
    for line in LinesWithEndings::from(s) {
        let regions = highlighter.highlight(line, SYNTAX_SET.deref());
        append_highlighted_html_for_styled_line(&regions[..], IncludeBackground::IfDifferent(bg), &mut output);
    }
    output.push_str("</pre>\n");
    output
}

pub fn html_render_line_number(s: &str, syntax: &SyntaxReference, theme: &Theme) -> String {
    let all = format!("{}", s.lines().count()).len();
    let mut line_number = 1;
    let mut highlighter = HighlightLines::new(syntax, theme);
    let (mut output, bg) = start_highlighted_html_snippet(theme);
    for line in LinesWithEndings::from(s) {
        let regions = highlighter.highlight(line, SYNTAX_SET.deref());
        let color = theme.settings.foreground.unwrap();
        let line = format!(
            r#"<span style="color:#{:02X}{:02X}{:02X};"> {:>width$} </span>{}"#,
            color.r,
            color.g,
            color.b,
            line_number,
            styled_line_to_highlighted_html(&regions[..], IncludeBackground::IfDifferent(bg)),
            width = all
        );
        line_number += 1;
        output.push_str(&line)
    }
    output.push_str("</pre>\n");
    output
}

pub fn html_fancy_box(s: &str, title: &Option<String>) -> String {
    format!(
        r#"<div class="carbon">
        <div class="controls"><div class="circle red"></div><div class="circle yellow"></div><div class="circle green"></div>{}</div>
        <div class="content">{}</div></div>"#,
        match title {
            Some(s) => { format!(r#"<div class="title">{}</div>"#, s) }
            None => { String::new() }
        },
        s
    )
}
