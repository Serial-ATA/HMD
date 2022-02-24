use crate::utils::*;
use crate::consts::*;
use crate::markdown::inline::tag::*;
use crate::markdown::line::{Line, LineType};
use crate::markdown::blockquote::count_blockquote;


pub fn get_type(line: &Line) -> LineType {

    if is_header(line) {
        LineType::Header
    }

    else if is_thematic_break(line) {
        LineType::ThematicBreak
    }

    else if is_code_fence(line) {
        LineType::CodeFence
    }

    else if is_table_delimiter(line) {
        LineType::TableDelimiter
    }

    else if is_blockquote(line) {
        LineType::Blockquote(count_blockquote(line))
    }

    else if is_empty(line) {
        LineType::Empty
    }

    else if is_independent_tag(line) {
        LineType::Tag
    }

    else if is_unordered_list(line) {
        LineType::UnorderedList
    }

    else if is_ordered_list(line) {
        LineType::OrderedList
    }

    else {
        LineType::Paragraph
    }

}


fn is_header(line: &Line) -> bool {

    line.indent == 0
    && {
        let (pre, post) = take_and_drop_while(&line.content, U16_SHARP);

        pre.len() > 0 && pre.len() < 7 && post[0] == U16_SPACE && drop_while(&post, U16_SPACE).len() > 0
    }
}


fn is_thematic_break(line: &Line) -> bool {

    line.indent < 4 && line.content.len() > 2 && (
        line.content[0] == U16_ASTERISK ||
        line.content[0] == U16_MINUS ||
        line.content[0] == U16_UNDERBAR
    ) && {
        let (pre, post) = take_and_drop_while(&line.content, line.content[0]);

        pre.len() > 2 && post.iter().filter(|c| *c != &U16_SPACE && *c != &U16_TAB).collect::<Vec<&u16>>().len() == 0
    }
}


fn is_code_fence(line: &Line) -> bool {

    line.indent == 0 && line.content.len() > 2 && &line.content[0..3] == &[U16_BACKTICK, U16_BACKTICK, U16_BACKTICK]
}


fn is_blockquote(line: &Line) -> bool {

    line.indent < 4 && line.content.len() > 2 && line.content[0] == U16_GREATER_THAN && {
        let content = drop_while(&line.content, U16_GREATER_THAN);

        content.len() > 0 && content[0] == U16_SPACE
    }
}


fn is_unordered_list(line: &Line) -> bool {

    line.content.len() > 2 && line.content[0] == U16_MINUS && line.content[1] == U16_SPACE
}


fn is_ordered_list(line: &Line) -> bool {

    line.content.len() > 3 && (
        line.content[0] == U16_1 ||
        line.content[0] == U16_SMALL_A ||
        line.content[0] == U16_SMALL_I ||
        line.content[0] == U16_BIG_A ||
        line.content[0] == U16_BIG_I
    ) && line.content[1] == U16_DOT && line.content[2] == U16_SPACE
}


fn is_independent_tag(line: &Line) -> bool {

    line.indent == 0 && line.content.len() > 4 && is_tag(&line.content, 0) && {
        let tag_end_index = get_bracket_end_index(&line.content, 0).unwrap();

        line.content.len() == tag_end_index + 1
    }
}


fn is_table_delimiter(line: &Line) -> bool {

    if line.content.len() == 0 || line.content[0] != U16_VERTICAL_BAR || line.content[line.content.len() - 1] != U16_VERTICAL_BAR {
        return false;
    }

    for c in line.content.iter() {

        if *c != U16_COLON && *c != U16_MINUS && *c != U16_VERTICAL_BAR && *c != U16_SPACE {
            return false;
        }

    }

    true
}


fn is_empty(line: &Line) -> bool {
    line.content.len() == 0
}