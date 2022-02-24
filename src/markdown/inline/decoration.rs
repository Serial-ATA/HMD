use crate::consts::*;
use crate::markdown::escape::prevent_backslash_escape;
use crate::markdown::inline::link::render_link;
use crate::markdown::inline::tag::render_tag;


pub fn render_code_spans(content: &Vec<u16>) -> Vec<u16> {

    let mut is_inside_code = false;
    let mut last_index = 0;
    let mut result = vec![];

    for (ind, c) in content.iter().enumerate() {

        if *c == U16_BACKTICK {

            if is_inside_code {
                result.push(vec![
                    vec![U16_LESS_THAN, U16_SMALL_C, U16_SMALL_O, U16_SMALL_D, U16_SMALL_E, U16_SPACE, U16_SMALL_C, U16_SMALL_L, U16_SMALL_A, U16_SMALL_S, U16_SMALL_S, U16_EQUAL, U16_DOUBLE_QUOTE, U16_SMALL_S, U16_SMALL_H, U16_SMALL_O, U16_SMALL_R, U16_SMALL_T, U16_DOUBLE_QUOTE, U16_GREATER_THAN],
                    prevent_backslash_escape(&content[last_index..ind].to_vec()),
                    vec![U16_LESS_THAN, U16_SLASH, U16_SMALL_C, U16_SMALL_O, U16_SMALL_D, U16_SMALL_E, U16_GREATER_THAN],
                ].concat());
            }

            else {
                result.push(render_link(&content[last_index..ind].to_vec()));
            }

            last_index = ind + 1;
            is_inside_code = !is_inside_code;
        }

    }

    if is_inside_code {
        last_index -= 1;
    }

    result.push(render_link(&content[last_index..].to_vec()));
    result.concat()
}


// <em>
pub fn render_italic(content: &Vec<u16>) -> Vec<u16> {

    let mut is_inside_emphasis = false;
    let mut last_index = 0;
    let mut result = vec![];

    for curr_index in 0..content.len() {

        if is_italic_delimiter(content, curr_index) {

            // <em> ends
            if is_inside_emphasis && content[curr_index - 1] != U16_SPACE {
                result.push(vec![U16_LESS_THAN, U16_SMALL_E, U16_SMALL_M, U16_GREATER_THAN]);
                result.push(render_bold(&content[last_index..curr_index].to_vec()));
                result.push(vec![U16_LESS_THAN, U16_SLASH, U16_SMALL_E, U16_SMALL_M, U16_GREATER_THAN]);
            }

            // <em> begins
            else if curr_index + 1 < content.len() && content[curr_index + 1] != U16_SPACE {
                result.push(render_bold(&content[last_index..curr_index].to_vec()));
            }

            else {
                continue;
            }

            last_index = curr_index + 1;
            is_inside_emphasis = !is_inside_emphasis;
        }

    }

    if is_inside_emphasis {
        last_index -= 1;
    }

    result.push(render_bold(&content[last_index..].to_vec()));
    result.concat()
}


fn is_italic_delimiter(content: &Vec<u16>, index: usize) -> bool {

    content[index] == U16_ASTERISK && (
        index == 0 || content[index - 1] != U16_ASTERISK
    ) && (
        index == content.len() - 1 || content[index + 1] != U16_ASTERISK
    )
}


// <strong>
fn render_bold(content: &Vec<u16>) -> Vec<u16> {

    let mut is_inside_bold = false;
    let mut last_index = 0;
    let mut result = vec![];

    for curr_index in 0..content.len() {

        if is_bold_delimiter(content, curr_index) {

            // <strong> ends
            if is_inside_bold && content[curr_index - 1] != U16_SPACE {
                result.push(vec![U16_LESS_THAN, U16_SMALL_S, U16_SMALL_T, U16_SMALL_R, U16_SMALL_O, U16_SMALL_N, U16_SMALL_G, U16_GREATER_THAN]);
                result.push(render_del(&content[last_index..curr_index].to_vec()));
                result.push(vec![U16_LESS_THAN, U16_SLASH, U16_SMALL_S, U16_SMALL_T, U16_SMALL_R, U16_SMALL_O, U16_SMALL_N, U16_SMALL_G, U16_GREATER_THAN]);
            }

            // <strong> begins
            else if curr_index + 2 < content.len() && content[curr_index + 2] != U16_SPACE {
                result.push(render_del(&content[last_index..curr_index].to_vec()));
            }

            else {
                continue;
            }

            last_index = curr_index + 2;
            is_inside_bold = !is_inside_bold;
        }

    }

    if is_inside_bold {
        last_index -= 2;
    }

    result.push(render_del(&content[last_index..].to_vec()));
    result.concat()
}


fn is_bold_delimiter(content: &Vec<u16>, index: usize) -> bool {

    content[index] == U16_ASTERISK && index + 1 < content.len() && content[index + 1] == U16_ASTERISK
}


// <del>
fn render_del(content: &Vec<u16>) -> Vec<u16> {

    let mut is_inside_del = false;
    let mut last_index = 0;
    let mut result = vec![];

    for curr_index in 0..content.len() {

        if is_del_delimiter(content, curr_index) {

            // <del> ends
            if is_inside_del && content[curr_index - 1] != U16_SPACE {
                result.push(vec![U16_LESS_THAN, U16_SMALL_D, U16_SMALL_E, U16_SMALL_L, U16_GREATER_THAN]);
                result.push(render_underline(&content[last_index..curr_index].to_vec()));
                result.push(vec![U16_LESS_THAN, U16_SLASH, U16_SMALL_D, U16_SMALL_E, U16_SMALL_L, U16_GREATER_THAN]);
            }

            // <del> begins
            else if curr_index + 2 < content.len() && content[curr_index + 2] != U16_SPACE {
                result.push(render_underline(&content[last_index..curr_index].to_vec()));
            }

            else {
                continue;
            }

            last_index = curr_index + 2;
            is_inside_del = !is_inside_del;
        }

    }

    if is_inside_del {
        last_index -= 2;
    }

    result.push(render_underline(&content[last_index..].to_vec()));
    result.concat()
}


fn is_del_delimiter(content: &Vec<u16>, index: usize) -> bool {

    content[index] == U16_TILDE && index + 1 < content.len() && content[index + 1] == U16_TILDE
}


// <u>
fn render_underline(content: &Vec<u16>) -> Vec<u16> {
    
    let mut is_inside_underline = false;
    let mut last_index = 0;
    let mut result = vec![];

    for curr_index in 0..content.len() {

        // <u> ends
        if is_inside_underline && is_underline_end(content, curr_index) {
            result.push(vec![U16_LESS_THAN, U16_SMALL_U, U16_GREATER_THAN]);
            result.push(render_subscript(&content[last_index..curr_index].to_vec()));
            result.push(vec![U16_LESS_THAN, U16_SLASH, U16_SMALL_U, U16_GREATER_THAN]);
        }

        // <u> begins
        else if !is_inside_underline && is_underline_begin(content, curr_index) {
            result.push(render_subscript(&content[last_index..curr_index].to_vec()));
        }

        else {
            continue;
        }

        is_inside_underline = !is_inside_underline;
        last_index = curr_index + 2;
    }
    
    if is_inside_underline {
        last_index -= 2;
    }

    result.push(render_subscript(&content[last_index..].to_vec()));
    result.concat()
}


// content[index] == `~`
fn is_underline_begin(content: &Vec<u16>, index: usize) -> bool {

    content[index] == U16_TILDE && index + 2 < content.len() && content[index + 1] == U16_UNDERBAR && content[index + 2] != U16_SPACE
}


// content[index] == `_`
fn is_underline_end(content: &Vec<u16>, index: usize) -> bool {

    content[index] == U16_UNDERBAR && index + 1 < content.len() && content[index + 1] == U16_TILDE && index > 0 && content[index - 1] != U16_SPACE
}


// <sub>
fn render_subscript(content: &Vec<u16>) -> Vec<u16> {

    let mut is_inside_subscript = false;
    let mut last_index = 0;
    let mut result = vec![];

    for curr_index in 0..content.len() {

        if is_subscript_delimiter(content, curr_index) {

            // <sub> ends
            if is_inside_subscript && content[curr_index - 1] != U16_SPACE && content[curr_index - 1] != U16_UNDERBAR {
                result.push(vec![U16_LESS_THAN, U16_SMALL_S, U16_SMALL_U, U16_SMALL_B, U16_GREATER_THAN]);
                result.push(render_superscript(&content[last_index..curr_index].to_vec()));
                result.push(vec![U16_LESS_THAN, U16_SLASH, U16_SMALL_S, U16_SMALL_U, U16_SMALL_B, U16_GREATER_THAN]);
            }

            // <sub> begins
            else if curr_index + 1 < content.len() && content[curr_index + 1] != U16_SPACE && content[curr_index + 1] != U16_UNDERBAR {
                result.push(render_superscript(&content[last_index..curr_index].to_vec()));
            }

            else {
                continue;
            }

            last_index = curr_index + 1;
            is_inside_subscript = !is_inside_subscript;
        }

        if is_inside_subscript && content[curr_index] == U16_SPACE {
            last_index -= 1;
            is_inside_subscript = false;
        }

    }

    if is_inside_subscript {
        last_index -= 1;
    }

    result.push(render_superscript(&content[last_index..].to_vec()));
    result.concat()
}


fn is_subscript_delimiter(content: &Vec<u16>, index: usize) -> bool {

    content[index] == U16_TILDE && (
        index == 0 || content[index - 1] != U16_TILDE
    ) && (
        index == content.len() - 1 || content[index + 1] != U16_TILDE
    )
}


// <sup>
fn render_superscript(content: &Vec<u16>) -> Vec<u16> {

    let mut is_inside_superscript = false;
    let mut last_index = 0;
    let mut result = vec![];

    for curr_index in 0..content.len() {

        if is_superscript_delimiter(content, curr_index) {

            // <sup> ends
            if is_inside_superscript && content[curr_index - 1] != U16_SPACE {
                result.push(vec![U16_LESS_THAN, U16_SMALL_S, U16_SMALL_U, U16_SMALL_P, U16_GREATER_THAN]);
                result.push(render_tag(&content[last_index..curr_index].to_vec()));
                result.push(vec![U16_LESS_THAN, U16_SLASH, U16_SMALL_S, U16_SMALL_U, U16_SMALL_P, U16_GREATER_THAN]);
            }

            // <sup> begins
            else if curr_index + 1 < content.len() && content[curr_index + 1] != U16_SPACE {
                result.push(render_tag(&content[last_index..curr_index].to_vec()));
            }

            else {
                continue;
            }

            last_index = curr_index + 1;
            is_inside_superscript = !is_inside_superscript;
        }

        if is_inside_superscript && content[curr_index] == U16_SPACE {
            last_index -= 1;
            is_inside_superscript = false;
        }

    }

    if is_inside_superscript {
        last_index -= 1;
    }

    result.push(render_tag(&content[last_index..].to_vec()));
    result.concat()
}


fn is_superscript_delimiter(content: &Vec<u16>, index: usize) -> bool {

    content[index] == U16_CARET && (
        index == 0 || content[index - 1] != U16_CARET
    ) && (
        index == content.len() - 1 || content[index + 1] != U16_CARET
    )
}
