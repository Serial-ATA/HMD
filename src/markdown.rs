mod line;
mod predicates;
mod code_fence;
mod list;
mod table;
mod blockquote;
mod paragraph;
mod inline;
mod escape;
mod syntax_highlighter;

use escape::*;
use inline::render_inlines;
use inline::tag::render_independent_tag;
use line::{code_to_lines, render_lines};
use code_fence::merge_code_fence;
use paragraph::merge_paragraph;
use table::merge_table;
use blockquote::merge_blockquote;
use list::merge_list;


pub fn render(content: &String) -> Result<String, ()> {

    let syntax_highlighter = self::syntax_highlighter::SyntaxHighlighter::new();

    let mut u16_content = content.encode_utf16().collect::<Vec<u16>>();
    u16_content = escape_htmls(&u16_content);

    let mut lines = code_to_lines(&u16_content);
    lines = merge_code_fence(&lines, &syntax_highlighter);
    lines = render_independent_tag(&lines);
    lines = render_inlines(lines);

    // `merge_paragraph` must be called after `merge_table`
    // because unused `TableDelimiter`s must be turned into `Paragraph`s.
    lines = merge_table(&lines);
    lines = merge_paragraph(&lines);

    lines = merge_list(&lines);
    lines = merge_blockquote(&lines);

    let mut result = render_lines(&lines);
    result = escape_backslashes(&result);

    Ok(String::from_utf16_lossy(&result))
}