use crate::markdown::escape::undo_html_escapes;
use crate::markdown::line::{Line, LineType};
use crate::consts::*;
use syntect::parsing::SyntaxSet;
use syntect::easy::HighlightLines;
use syntect::highlighting::{ThemeSet, Theme};
use syntect::html::{IncludeBackground, append_highlighted_html_for_styled_line};

pub struct SyntaxHighlighter {
    syntaxes: SyntaxSet,
    theme: Theme
}

impl SyntaxHighlighter {
    pub fn new() -> SyntaxHighlighter {
        let syntax_set = SyntaxSet::load_defaults_nonewlines();
        let theme_set = ThemeSet::load_defaults();
        let theme = theme_set.themes["base16-eighties.dark"].clone();

        SyntaxHighlighter {
            syntaxes: syntax_set,
            theme
        }
    }

    pub fn highlight_syntax(&self, lines: Vec<Line>, syntax: &str) -> Vec<Line> {

        let syntax_rule = match self.syntaxes.find_syntax_by_token(syntax) {
            None => {return lines;}
            Some(s) => s
        };

        let mut highlighter = HighlightLines::new(syntax_rule, &self.theme);
        let mut output = String::new();

        for ln in lines.iter() {
            let mut raw_line = undo_html_escapes(&ln.into_raw());
            raw_line.push(U16_NEWLINE);

            let curr_line = &String::from_utf16_lossy(&raw_line);
            let styled_line = highlighter.highlight(curr_line, &self.syntaxes);
            append_highlighted_html_for_styled_line(
                &styled_line[..],
                IncludeBackground::No,
                &mut output
            );
        }

        let result = output.encode_utf16().collect::<Vec<u16>>();

        result.split(
            |c|
            *c == U16_NEWLINE
        ).map(
            |ln| Line {
                line_type: LineType::Paragraph,
                content: ln.to_vec(),
                indent: 0
            }
        ).collect::<Vec<Line>>()
    }

}
