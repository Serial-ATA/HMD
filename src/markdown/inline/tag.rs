use crate::consts::*;
use crate::markdown::line::{Line, LineType};
use crate::utils::{get_bracket_end_index, lowercase};
use std::str::FromStr;


pub fn render_independent_tag(lines: &Vec<Line>) -> Vec<Line> {

    let mut result = Vec::with_capacity(lines.len());

    for ln in lines.iter() {

        if ln.line_type == LineType::Tag {
            let mut new_line = ln.clone();

            match parse_tag(&ln.content, 0) {
                Err(_) => {new_line.line_type = LineType::Paragraph;}
                Ok(c) => {new_line.content = c;}
            }

            result.push(new_line);
        }

        else {
            result.push(ln.clone());
        }

    }

    result
}


pub fn render_tag(content: &Vec<u16>) -> Vec<u16> {

    let mut result = vec![];
    let mut curr_index = 0;
    let mut last_index = 0;

    while curr_index < content.len() {

        if is_tag(content, curr_index) {
            let tag = parse_tag(content, curr_index);

            if tag.is_ok() {
                result.push(content[last_index..curr_index].to_vec());
                result.push(tag.unwrap());
                last_index = get_bracket_end_index(content, curr_index).unwrap() + 1;
                curr_index = last_index;
                continue;
            }

        }

        curr_index += 1;
    }

    if last_index < curr_index {
        result.push(content[last_index..].to_vec());
    }

    result.concat()
}


pub fn is_tag(content: &Vec<u16>, index: usize) -> bool {

    content[index] == U16_LEFT_SQUARE_BRACKET && index + 1 < content.len() && content[index + 1] == U16_LEFT_SQUARE_BRACKET && {

        let end1 = match get_bracket_end_index(content, index) {
            None => {return false;}
            Some(i) => i
        };

        let end2 = match get_bracket_end_index(content, index + 1) {
            None => {return false;}
            Some(i) => i
        };

        end2 + 1 == end1
    }
}


fn parse_tag(content: &Vec<u16>, index: usize) -> Result<Vec<u16>, ()> {

    let end_index = get_bracket_end_index(content, index + 1).unwrap();
    let content = content[index + 2..end_index].iter().filter(|c| *c != &U16_SPACE).map(|c| lowercase(*c)).collect::<Vec<u16>>();

    if content.len() == 0 {
        return Err(());
    }

    // <span class="font_red">
    // <span class="font_big">
    if is_color_name(&content) || is_size_name(&content) {
        return Ok(vec![
            vec![U16_LESS_THAN, U16_SMALL_S, U16_SMALL_P, U16_SMALL_A, U16_SMALL_N, U16_SPACE, U16_SMALL_C, U16_SMALL_L, U16_SMALL_A, U16_SMALL_S, U16_SMALL_S, U16_EQUAL, U16_DOUBLE_QUOTE, U16_SMALL_F, U16_SMALL_O, U16_SMALL_N, U16_SMALL_T, U16_UNDERBAR],
            content,
            vec![U16_DOUBLE_QUOTE, U16_GREATER_THAN]
        ].concat());
    }

    // <div class="align_center">
    if is_alignment_name(&content) {
        return Ok(vec![
            vec![U16_LESS_THAN, U16_SMALL_D, U16_SMALL_I, U16_SMALL_V, U16_SPACE, U16_SMALL_C, U16_SMALL_L, U16_SMALL_A, U16_SMALL_S, U16_SMALL_S, U16_EQUAL, U16_DOUBLE_QUOTE, U16_SMALL_A, U16_SMALL_L, U16_SMALL_I, U16_SMALL_G, U16_SMALL_N, U16_UNDERBAR],
            content,
            vec![U16_DOUBLE_QUOTE, U16_GREATER_THAN]
        ].concat());
    }

    // <div class="box">
    if is_box_name(&content) {
        return Ok(vec![U16_LESS_THAN, U16_SMALL_D, U16_SMALL_I, U16_SMALL_V, U16_SPACE, U16_SMALL_C, U16_SMALL_L, U16_SMALL_A, U16_SMALL_S, U16_SMALL_S, U16_EQUAL, U16_DOUBLE_QUOTE, U16_SMALL_B, U16_SMALL_O, U16_SMALL_X, U16_DOUBLE_QUOTE, U16_GREATER_THAN]);
    }

    if is_blank_name(&content) {
        return Ok(vec![U16_AMPERSAND, U16_SMALL_N, U16_SMALL_B, U16_SMALL_S, U16_SMALL_P, U16_SEMI_COLON]);
    }

    if is_icon(&content) {

        match parse_icon(&content) {
            Some(s) => {return Ok(s);}
            _ => {}
        }

    }

    if content[0] == U16_SLASH {

        if is_color_name(&content[1..]) || is_size_name(&content[1..]) {
            return Ok(vec![U16_LESS_THAN, U16_SLASH, U16_SMALL_S, U16_SMALL_P, U16_SMALL_A, U16_SMALL_N, U16_GREATER_THAN]);
        }

        if is_alignment_name(&content[1..]) || is_box_name(&content[1..]) {
            return Ok(vec![U16_LESS_THAN, U16_SLASH, U16_SMALL_D, U16_SMALL_I, U16_SMALL_V, U16_GREATER_THAN]);
        }

    }

    Err(())
}

// TODO: make a `color` module
// define all colors there
// `color_names.contains(string)` would be much better!
fn is_color_name(string: &[u16]) -> bool {
    string == &vec![U16_SMALL_A, U16_SMALL_Q, U16_SMALL_U, U16_SMALL_A] ||
    string == &vec![U16_SMALL_B, U16_SMALL_L, U16_SMALL_U, U16_SMALL_E] ||
    string == &vec![U16_SMALL_G, U16_SMALL_R, U16_SMALL_E, U16_SMALL_E, U16_SMALL_N] ||
    string == &vec![U16_SMALL_L, U16_SMALL_I, U16_SMALL_M, U16_SMALL_E] ||
    string == &vec![U16_SMALL_O, U16_SMALL_R, U16_SMALL_A, U16_SMALL_N, U16_SMALL_G, U16_SMALL_E] ||
    string == &vec![U16_SMALL_R, U16_SMALL_E, U16_SMALL_D] ||
    string == &vec![U16_SMALL_V, U16_SMALL_I, U16_SMALL_O, U16_SMALL_L, U16_SMALL_E, U16_SMALL_T] ||
    string == &vec![U16_SMALL_W, U16_SMALL_H, U16_SMALL_I, U16_SMALL_T, U16_SMALL_E] ||
    string == &vec![U16_SMALL_Y, U16_SMALL_E, U16_SMALL_L, U16_SMALL_L, U16_SMALL_O, U16_SMALL_W]
}

fn is_size_name(string: &[u16]) -> bool {
    string == &vec![U16_SMALL_B, U16_SMALL_I, U16_SMALL_G] ||
    string == &vec![U16_SMALL_S, U16_SMALL_M, U16_SMALL_A, U16_SMALL_L, U16_SMALL_L] ||
    string == &vec![U16_SMALL_M, U16_SMALL_E, U16_SMALL_D, U16_SMALL_I, U16_SMALL_U, U16_SMALL_M]
}

fn is_alignment_name(string: &[u16]) -> bool {
    string == &vec![U16_SMALL_L, U16_SMALL_E, U16_SMALL_F, U16_SMALL_T] ||
    string == &vec![U16_SMALL_R, U16_SMALL_I, U16_SMALL_G, U16_SMALL_H, U16_SMALL_T] ||
    string == &vec![U16_SMALL_C, U16_SMALL_E, U16_SMALL_N, U16_SMALL_T, U16_SMALL_E, U16_SMALL_R]
}

fn is_box_name(string: &[u16]) -> bool {
    string.len() == 3 && string[0] == U16_SMALL_B && string[1] == U16_SMALL_O && string[2] == U16_SMALL_X
}

fn is_blank_name(string: &[u16]) -> bool {
    string.len() == 5 && string[0] == U16_SMALL_B && string[1] == U16_SMALL_L && string[2] == U16_SMALL_A && string[3] == U16_SMALL_N && string[4] == U16_SMALL_K
}

// if true, it's possibly a valid icon
// if not, it can never be an icon
fn is_icon(string: &[u16]) -> bool {
    string.len() > 5 && string[0] == U16_SMALL_I && string[1] == U16_SMALL_C && string[2] == U16_SMALL_O && string[3] == U16_SMALL_N && string[4] == U16_EQUAL
}

// `[[a = b, c = d, e = f]]` -> vec![(`a`, `b`), (`c`, `d`), (`e`, `f`)]
fn parse_arguments(content: &[u16]) -> Vec<(Vec<u16>, Vec<u16>)> {  // Vec<(key, value)>
    content.split(
        |c|
        *c == U16_COMMA
    ).filter_map(
        |arg| {
            let arg_split = arg.split(
                |c|
                *c == U16_EQUAL
            ).map(
                |word|
                word.to_vec()
            ).collect::<Vec<Vec<u16>>>();

            if arg_split.len() == 2 {
                Some((arg_split[0].clone(), arg_split[1].clone()))
            } else {
                None
            }
        }
    ).collect()
}

fn parse_icon(content: &[u16]) -> Option<Vec<u16>> {

    let mut curr_icon = None;
    let mut curr_size = None;
    let mut curr_color = None;
    let args = parse_arguments(content);

    // if the same arg is given twice, the later one is applied
    // I'm not raising an error for that
    for (key, value) in args.iter() {

        if key == &vec![U16_SMALL_I, U16_SMALL_C, U16_SMALL_O, U16_SMALL_N] {
            curr_icon = Some(value.clone());
        }

        else if key == &vec![U16_SMALL_S, U16_SMALL_I, U16_SMALL_Z, U16_SMALL_E] {
            curr_size = Some(value.clone());
        }

        else if key == &vec![U16_SMALL_C, U16_SMALL_O, U16_SMALL_L, U16_SMALL_O, U16_SMALL_R] {
            curr_color = Some(value.clone());
        }

        else {
            return None;
        }

    }

    let curr_icon = if curr_icon.is_none() {
        return None
    } else {
        curr_icon.unwrap()
    };

    let curr_size = if curr_size.is_none() {
        24
    } else {
        match i32::from_str(&String::from_utf16_lossy(&curr_size.unwrap())) {
            Err(_) => {return None}
            Ok(n) => n
        }
    };

    let curr_color = match curr_color {
        None => vec![U16_SMALL_W, U16_SMALL_H, U16_SMALL_I, U16_SMALL_T, U16_SMALL_E],
        Some(s) => if is_color_name(&s) {
            s.clone()
        } else {
            return None
        }
    };

    panic!("Not Implemented Yet!")
}