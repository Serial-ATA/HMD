use crate::markdown::escape::prevent_backslash_escape;
use crate::markdown::line::{Line, LineType};
use crate::markdown::syntax_highlighter::SyntaxHighlighter;
use crate::utils::{drop_while, get_parenthesis_end_index};
use crate::consts::*;
use std::str::FromStr;


#[derive(Default)]
struct CodeFenceOption {
    language: Option<Vec<u16>>,
    line_num: Option<i32>,
}


fn parse_code_fence_option(content: &Vec<u16>) -> CodeFenceOption {

    let mut content = drop_while(content, U16_BACKTICK);
    content = content.iter().filter(|c| *c != &U16_SPACE).map(|c| *c).collect();

    let params = content.split(|c| *c == U16_COMMA);

    let mut language = None;
    let mut line_num = None;

    'outer: for p in params {
        
        'inner: for (ind, c) in p.iter().enumerate() {

            if *c == U16_LEFT_PARENTHESIS {

                match get_parenthesis_end_index(&p.to_vec(), ind) {
                    None => {break 'inner;}
                    Some(i) => {
                        match i32::from_str(&String::from_utf16_lossy(&p[ind + 1..i])) {
                            Err(_) => {break 'inner;}
                            Ok(n) => {line_num = Some(n); continue 'outer;}
                        }
                    }
                }

            }

        }

        language = Some(p.to_vec());
    }

    CodeFenceOption {
        language, line_num
    }
}


pub fn merge_code_fence(lines: &Vec<Line>, syntax_highlighter: &SyntaxHighlighter) -> Vec<Line> {

    let mut curr_fenced_code = vec![];
    let mut is_inside_fence = false;
    let mut result = Vec::with_capacity(lines.len());
    let mut code_fence_option = CodeFenceOption::default();

    for ln in lines.iter() {

        match ln.line_type {
            LineType::CodeFence => {

                if is_inside_fence {
                    result.push(construct_code_fence(curr_fenced_code, code_fence_option, syntax_highlighter));
                    curr_fenced_code = vec![];
                    code_fence_option = CodeFenceOption::default();
                }

                else {
                    code_fence_option = parse_code_fence_option(&ln.content);
                }

                is_inside_fence = !is_inside_fence;
            }
            _ => {

                if is_inside_fence {
                    curr_fenced_code.push(ln.clone());
                }

                else {
                    result.push(ln.clone());
                }

            }
        }

    }

    if curr_fenced_code.len() > 0 {
        result.push(construct_code_fence(curr_fenced_code, code_fence_option, syntax_highlighter));
    }

    result
}


fn construct_code_fence(mut lines: Vec<Line>, mut options: CodeFenceOption, syntax_highlighter: &SyntaxHighlighter) -> Line {

    let mut result = Line {
        content: vec![],
        indent: 0,
        line_type: LineType::FencedCode
    };

    if lines.len() == 0 {
        return result;
    }

    match options.language {
        Some(ref s) => {
            lines = syntax_highlighter.highlight_syntax(lines, &String::from_utf16_lossy(&s));
        }
        _ => {}
    }

    let mut content = Vec::with_capacity(lines.len() * 3);

    // all those messes are to remove a trailing `\n`
    for (ind, ln) in lines.iter().enumerate() {

        if ind + 1 < lines.len() {
            content.push(render_line_number(&mut options));
        }

        content.push(ln.into_raw());
        content.push(vec![U16_NEWLINE]);
    }

    result.content = content[0..content.len() - 1].concat();
    result.content = prevent_backslash_escape(&result.content);

    result
}


// <span class="line_no">0</span>
fn render_line_number(options: &mut CodeFenceOption) -> Vec<u16> {

    match options.line_num {
        None => {vec![]}
        Some(ref mut n) => {
            *n += 1;
            vec![
                vec![U16_LESS_THAN, U16_SMALL_S, U16_SMALL_P, U16_SMALL_A, U16_SMALL_N, U16_SPACE, U16_SMALL_C, U16_SMALL_L, U16_SMALL_A, U16_SMALL_S, U16_SMALL_S, U16_EQUAL, U16_DOUBLE_QUOTE, U16_SMALL_L, U16_SMALL_I, U16_SMALL_N, U16_SMALL_E, U16_UNDERBAR, U16_SMALL_N, U16_SMALL_O, U16_DOUBLE_QUOTE, U16_GREATER_THAN],
                (*n - 1).to_string().encode_utf16().collect::<Vec<u16>>(),
                vec![U16_LESS_THAN, U16_SLASH, U16_SMALL_S, U16_SMALL_P, U16_SMALL_A, U16_SMALL_N, U16_GREATER_THAN]
            ].concat()
        }
    }

}