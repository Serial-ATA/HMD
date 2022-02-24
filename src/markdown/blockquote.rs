use crate::markdown::line::{Line, LineType};
use crate::utils::{take_while, drop_while};
use crate::consts::*;


impl Line {

    fn get_blockquote_level(&self) -> usize {

        match self.line_type {
            LineType::Blockquote(n) => n,
            _ => 0
        }

    }

}


pub fn merge_blockquote(lines: &Vec<Line>) -> Vec<Line> {

    let mut current_blockquote = vec![];
    let mut result = Vec::with_capacity(lines.len());

    for ln in lines.iter() {

        match ln.line_type {
            LineType::Blockquote(_) => {
                current_blockquote.push(ln);
            },
            LineType::Paragraph => {

                if current_blockquote.len() > 0 {
                    current_blockquote.push(ln);
                }

                else {
                    result.push(ln.clone());
                }

            },
            _ => {

                if current_blockquote.len() > 0 {
                    result.push(construct_blockquote(current_blockquote));
                    current_blockquote = vec![];
                }

                result.push(ln.clone());
            }

        }

    }

    if current_blockquote.len() > 0 {
        result.push(construct_blockquote(current_blockquote));
    }

    result
}


pub fn count_blockquote(line: &Line) -> usize {
    take_while(&line.content, U16_GREATER_THAN).len()
}


fn construct_blockquote(lines: Vec<&Line>) -> Line {

    let mut current_blockquote_level = 0;
    let mut content = Vec::with_capacity(lines.len());

    // laziness
    // https://github.github.com/gfm/#block-quotes
    for ln in lines.iter() {
        let ln_level = ln.get_blockquote_level();

        if ln_level > current_blockquote_level {
            content.push(vec![vec![U16_LESS_THAN, U16_SMALL_B, U16_SMALL_L, U16_SMALL_O, U16_SMALL_C, U16_SMALL_K, U16_SMALL_Q, U16_SMALL_U, U16_SMALL_O, U16_SMALL_T, U16_SMALL_E, U16_GREATER_THAN];ln_level - current_blockquote_level].concat());
            current_blockquote_level = ln_level;
        }

        content.push(drop_while(&drop_while(&ln.content, U16_GREATER_THAN), U16_SPACE));

        if ln.content.len() > 2
        && ln.content[ln.content.len() - 1] == U16_SPACE
        && ln.content[ln.content.len() - 2] == U16_SPACE  {
            content.push(vec![U16_LESS_THAN, U16_SMALL_B, U16_SMALL_R, U16_SPACE, U16_SLASH, U16_GREATER_THAN]);
        }

        else {
            content.push(vec![U16_SPACE]);
        }

    }

    content.push(vec![vec![U16_LESS_THAN, U16_SLASH, U16_SMALL_B, U16_SMALL_L, U16_SMALL_O, U16_SMALL_C, U16_SMALL_K, U16_SMALL_Q, U16_SMALL_U, U16_SMALL_O, U16_SMALL_T, U16_SMALL_E, U16_GREATER_THAN];current_blockquote_level].concat());

    Line {
        content: content.concat(),
        indent: 0,
        line_type: LineType::Blockquote(0)
    }
}