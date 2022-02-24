use crate::markdown::line::{Line, LineType};
use crate::consts::*;


pub fn merge_paragraph(lines: &Vec<Line>) -> Vec<Line> {

    let mut curr_paragraph = vec![];
    let mut result = Vec::with_capacity(lines.len());

    for ln in lines.iter() {

        match ln.line_type {
            LineType::Paragraph => {
                curr_paragraph.push(ln);
            }
            _ => {

                if curr_paragraph.len() > 0 {
                    result.push(construct_paragraph(curr_paragraph));
                    curr_paragraph = vec![];
                }

                result.push(ln.clone());

            }
        }

    }

    if curr_paragraph.len() > 0 {
        result.push(construct_paragraph(curr_paragraph));
    }

    result
}


fn construct_paragraph(paragraph: Vec<&Line>) -> Line {

    let mut contents = Vec::with_capacity(paragraph.len() * 2);

    if paragraph.len() > 0 {

        for p in paragraph.iter() {
            contents.push(p.content.clone());

            if p.content.len() > 2
            && p.content[p.content.len() - 1] == U16_SPACE
            && p.content[p.content.len() - 2] == U16_SPACE  {
                contents.push(vec![U16_LESS_THAN, U16_SMALL_B, U16_SMALL_R, U16_SPACE, U16_SLASH, U16_GREATER_THAN]);
            }

            else {
                contents.push(vec![U16_SPACE]);
            }

        }

        contents.pop();
    }

    Line {
        content: contents.concat(),
        indent: 0,
        line_type: LineType::Paragraph
    }
}