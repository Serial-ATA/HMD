use crate::markdown::line::{Line, LineType};
use self::decoration::render_code_spans;

mod decoration;
mod link;
pub mod tag;


/*
I can't figure out nice way to deal with multiple nested inline elements. I can't understand [this article](https://github.github.com/gfm/#delimiter-run). So this is detour I chose.

# precedence

*inlines are rendered sequentially, in this order*

code span
link
image
italic
bold
del
underline
subscript
superscript
alignment
color
size
box
--- implemented so far
svg
footnote
*/


pub fn render_inlines(lines: Vec<Line>) -> Vec<Line> {

    lines.into_iter().map(render_inline).collect()
}


pub fn render_inline(mut line: Line) -> Line {
    
    match line.line_type {
        LineType::FencedCode | LineType::CodeFence | LineType::ThematicBreak
        | LineType::Empty | LineType::TableDelimiter | LineType::RenderedTable
        | LineType::Tag => line,

        LineType::Paragraph | LineType::Header | LineType::Blockquote(_)
        | LineType::OrderedList | LineType::UnorderedList => {
            let new_content = render_code_spans(&line.content);
            line.content = new_content;

            line
        }
    }
}