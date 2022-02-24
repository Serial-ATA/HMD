use crate::consts::*;


/*
`<`s are converted to `&lt` and `&gt`, always!
`>`s are kept untouched because they could be part of a blockquote
backslashes are always escaped.
*/

pub fn escape_htmls(content: &Vec<u16>) -> Vec<u16> {

    let mut result = Vec::with_capacity(content.len() + content.len() / 4);

    for c in content.iter() {

        match *c {
            U16_AMPERSAND => {
                result.push(U16_AMPERSAND);
                result.push(U16_SMALL_A);
                result.push(U16_SMALL_M);
                result.push(U16_SMALL_P);
                result.push(U16_SEMI_COLON);
            },
            U16_LESS_THAN => {
                result.push(U16_AMPERSAND);
                result.push(U16_SMALL_L);
                result.push(U16_SMALL_T);
                result.push(U16_SEMI_COLON);
            },
            U16_DOUBLE_QUOTE => {
                result.push(U16_AMPERSAND);
                result.push(U16_SMALL_Q);
                result.push(U16_SMALL_U);
                result.push(U16_SMALL_O);
                result.push(U16_SMALL_T);
                result.push(U16_SEMI_COLON);
            },
            U16_SINGLE_QUOTE => {
                result.push(U16_AMPERSAND);
                result.push(U16_SMALL_A);
                result.push(U16_SMALL_P);
                result.push(U16_SMALL_O);
                result.push(U16_SMALL_S);
                result.push(U16_SEMI_COLON);
            },
            _ => {
                result.push(*c);
            }
        }
    }

    result
}


pub fn undo_html_escapes(content: &Vec<u16>) -> Vec<u16> {
    
    let mut result = Vec::with_capacity(content.len());
    let mut index = 0;

    while index < content.len() {

        match is_escaped(content, index) {
            None => {
                result.push(content[index]);
            }
            Some((c, i)) => {
                result.push(c);
                index = i;
            }
        }

        index += 1;
    }

    result
}


// does not undo `<`
pub fn undo_html_escapes_safely(content: &Vec<u16>) -> Vec<u16> {
    
    let mut result = Vec::with_capacity(content.len());
    let mut index = 0;

    while index < content.len() {

        match is_escaped(content, index) {
            None => {
                result.push(content[index]);
            }
            Some((c, i)) => {

                if c == U16_LESS_THAN {
                    result.push(content[index]);
                }

                else {
                    result.push(c);
                    index = i;
                }

            }
        }

        index += 1;
    }

    result
}


pub fn is_escaped(content: &Vec<u16>, index: usize) -> Option<(u16, usize)> {

    if content[index] == U16_AMPERSAND && index + 4 < content.len() {

        if content[index + 1] == U16_SMALL_A {

            if content[index + 2] == U16_SMALL_M && content[index + 3] == U16_SMALL_P && content[index + 4] == U16_SEMI_COLON {
                return Some((U16_AMPERSAND, index + 4));
            }

            if content[index + 2] == U16_SMALL_P && content[index + 3] == U16_SMALL_O && content[index + 4] == U16_SMALL_S && index + 5 < content.len() && content[index + 5] == U16_SEMI_COLON {
                return Some((U16_SINGLE_QUOTE, index + 5));
            }

        }

        else if content[index + 1] == U16_SMALL_L && content[index + 2] == U16_SMALL_T && content[index + 3] == U16_SEMI_COLON {
            return Some((U16_LESS_THAN, index + 3));
        }

        else if content[index + 1] == U16_SMALL_Q && content[index + 2] == U16_SMALL_U && content[index + 3] == U16_SMALL_O && content[index + 4] == U16_SMALL_T && index + 5 < content.len() && content[index + 5] == U16_SEMI_COLON {
            return Some((U16_DOUBLE_QUOTE, index + 5));
        }

    }

    None
}


pub fn escape_backslashes(content: &Vec<u16>) -> Vec<u16> {

    let mut result = Vec::with_capacity(content.len());
    let mut is_escaped = false;

    for c in content.iter() {

        if is_escaped {
            is_escaped = false;
            result.push(*c);
            continue;
        }

        if *c == U16_BACKSLASH {
            is_escaped = true;
        }

        else {
            result.push(*c);
        }

    }

    result
}


pub fn prevent_backslash_escape(content: &Vec<u16>) -> Vec<u16> {

    let mut result = Vec::with_capacity(content.len() + content.len() / 10);

    for c in content.iter() {

        if *c == U16_BACKSLASH {
            result.push(U16_BACKSLASH);
            result.push(U16_BACKSLASH);
        }

        else {
            result.push(*c);
        }

    }

    result
}