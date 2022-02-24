use crate::markdown::line::{Line, LineType};
use crate::consts::*;


enum ListOrder {
    None, Num, AlphaLow, AlphaUpper, RomanLow, RomanUpper
}


impl ListOrder {

    fn opening_tag(&self) -> Vec<u16> {

        match self {
            ListOrder::None => vec![U16_LESS_THAN, U16_SMALL_U, U16_SMALL_L, U16_GREATER_THAN],
            ListOrder::Num => vec![U16_LESS_THAN, U16_SMALL_O, U16_SMALL_L, U16_SPACE, U16_SMALL_T, U16_SMALL_Y, U16_SMALL_P, U16_SMALL_E, U16_EQUAL, U16_DOUBLE_QUOTE, U16_1, U16_DOUBLE_QUOTE, U16_GREATER_THAN],
            ListOrder::AlphaLow => vec![U16_LESS_THAN, U16_SMALL_O, U16_SMALL_L, U16_SPACE, U16_SMALL_T, U16_SMALL_Y, U16_SMALL_P, U16_SMALL_E, U16_EQUAL, U16_DOUBLE_QUOTE, U16_SMALL_A, U16_DOUBLE_QUOTE, U16_GREATER_THAN],
            ListOrder::AlphaUpper => vec![U16_LESS_THAN, U16_SMALL_O, U16_SMALL_L, U16_SPACE, U16_SMALL_T, U16_SMALL_Y, U16_SMALL_P, U16_SMALL_E, U16_EQUAL, U16_DOUBLE_QUOTE, U16_BIG_A, U16_DOUBLE_QUOTE, U16_GREATER_THAN],
            ListOrder::RomanLow => vec![U16_LESS_THAN, U16_SMALL_O, U16_SMALL_L, U16_SPACE, U16_SMALL_T, U16_SMALL_Y, U16_SMALL_P, U16_SMALL_E, U16_EQUAL, U16_DOUBLE_QUOTE, U16_SMALL_I, U16_DOUBLE_QUOTE, U16_GREATER_THAN],
            ListOrder::RomanUpper => vec![U16_LESS_THAN, U16_SMALL_O, U16_SMALL_L, U16_SPACE, U16_SMALL_T, U16_SMALL_Y, U16_SMALL_P, U16_SMALL_E, U16_EQUAL, U16_DOUBLE_QUOTE, U16_BIG_I, U16_DOUBLE_QUOTE, U16_GREATER_THAN],
        }

    }

    fn closing_tag(&self) -> Vec<u16> {

        match self {
            ListOrder::None => vec![U16_LESS_THAN, U16_SLASH, U16_SMALL_U, U16_SMALL_L, U16_GREATER_THAN],
            _ => vec![U16_LESS_THAN, U16_SLASH, U16_SMALL_O, U16_SMALL_L, U16_GREATER_THAN],
        }
    
    }

}


impl Line {

    fn get_list_ordering(&self) -> ListOrder {

        match self.line_type {
            LineType::UnorderedList => ListOrder::None,
            LineType::OrderedList => {
                match self.content[0] {
                    U16_1 => ListOrder::Num,
                    U16_SMALL_A => ListOrder::AlphaLow,
                    U16_BIG_A => ListOrder::AlphaUpper,
                    U16_SMALL_I => ListOrder::RomanLow,
                    U16_BIG_I => ListOrder::RomanUpper,
                    _ => panic!()
                }
            }
            _ => panic!()
        }

    }

    fn get_list_content(&self) -> Vec<u16> {

        match self.line_type {
            LineType::UnorderedList => self.content[2..].to_vec(),
            LineType::OrderedList => self.content[3..].to_vec(),
            _ => panic!()
        }

    }

}


pub fn merge_list(lines: &Vec<Line>) -> Vec<Line> {

    let mut current_list = vec![];
    let mut result = Vec::with_capacity(lines.len());

    for ln in lines.iter() {

        match ln.line_type {
            LineType::UnorderedList | LineType::OrderedList => {
                current_list.push(ln.clone());
            },
            LineType::Paragraph => {

                if current_list.len() > 0 {
                    current_list.push(ln.clone());
                }

                else {
                    result.push(ln.clone());
                }

            },
            _ => {

                if current_list.len() > 0 {
                    result.push(construct_list(current_list));
                    current_list = vec![];
                }

                result.push(ln.clone());
            }

        }

    }

    if current_list.len() > 0 {
        result.push(construct_list(current_list));
    }

    result
}


fn construct_list(mut lines: Vec<Line>) -> Line {

    lines = merge_paragraph_to_list(lines);
    let (_, content) = construct_list_recursive(&lines, 0, lines[0].indent, lines[0].get_list_ordering());

    Line {
        line_type: LineType::UnorderedList,
        content,
        indent: 0
    }
}


fn merge_paragraph_to_list(mut lines: Vec<Line>) -> Vec<Line> {

    let mut result = Vec::with_capacity(lines.len());

    for ind in 0..lines.len() {

        if lines[ind].line_type == LineType::Paragraph {
            continue;
        }

        if ind + 1 < lines.len() && lines[ind + 1].line_type == LineType::Paragraph {
            lines[ind].content = vec![
                lines[ind].content.clone(),
                vec![U16_SPACE],
                lines[ind + 1].content.clone()
            ].concat();
        }

        result.push(lines[ind].clone());
    }

    result
}


fn construct_list_recursive(lines: &Vec<Line>, begin_index: usize, curr_indent: usize, ordering: ListOrder) -> (usize, Vec<u16>) {

    let mut result = vec![];
    let mut index = begin_index;
    result.push(ordering.opening_tag());

    while index < lines.len() {
        let ln = &lines[index];

        if ln.indent < curr_indent {
            result.push(ordering.closing_tag());
            return (index, result.concat());
        }

        else if ln.indent > curr_indent {
            result.pop();  // `</li>`
            let (next_index, content) = construct_list_recursive(lines, index, ln.indent, ln.get_list_ordering());

            index = next_index;
            result.push(content);
            result.push(vec![U16_LESS_THAN, U16_SLASH, U16_SMALL_L, U16_SMALL_I, U16_GREATER_THAN]);
            continue;
        }

        else {
            result.push(vec![U16_LESS_THAN, U16_SMALL_L, U16_SMALL_I, U16_GREATER_THAN]);
            result.push(render_task_list(ln.get_list_content()));
            result.push(vec![U16_LESS_THAN, U16_SLASH, U16_SMALL_L, U16_SMALL_I, U16_GREATER_THAN]);
        }

        index += 1;
    }

    result.push(ordering.closing_tag());

    (lines.len(), result.concat())
}

// `line` parameter is a content of a list
// <div class="checked_box"><span class="checkmark"></span></div>
// <div class="unchecked_box"></div>
fn render_task_list(line: Vec<u16>) -> Vec<u16> {
    
    if is_task_list(&line) {
        vec![
            vec![U16_LESS_THAN, U16_SMALL_D, U16_SMALL_I, U16_SMALL_V, U16_SPACE, U16_SMALL_C, U16_SMALL_L, U16_SMALL_A, U16_SMALL_S, U16_SMALL_S, U16_EQUAL, U16_DOUBLE_QUOTE],
            if line[1] == U16_SPACE {
                vec![U16_SMALL_U, U16_SMALL_N, U16_SMALL_C, U16_SMALL_H, U16_SMALL_E, U16_SMALL_C, U16_SMALL_K, U16_SMALL_E, U16_SMALL_D, U16_UNDERBAR, U16_SMALL_B, U16_SMALL_O, U16_SMALL_X, U16_DOUBLE_QUOTE, U16_GREATER_THAN]
            } else {
                vec![U16_SMALL_C, U16_SMALL_H, U16_SMALL_E, U16_SMALL_C, U16_SMALL_K, U16_SMALL_E, U16_SMALL_D, U16_UNDERBAR, U16_SMALL_B, U16_SMALL_O, U16_SMALL_X, U16_DOUBLE_QUOTE, U16_GREATER_THAN, U16_LESS_THAN, U16_SMALL_S, U16_SMALL_P, U16_SMALL_A, U16_SMALL_N, U16_SPACE, U16_SMALL_C, U16_SMALL_L, U16_SMALL_A, U16_SMALL_S, U16_SMALL_S, U16_EQUAL, U16_DOUBLE_QUOTE, U16_SMALL_C, U16_SMALL_H, U16_SMALL_E, U16_SMALL_C, U16_SMALL_K, U16_SMALL_M, U16_SMALL_A, U16_SMALL_R, U16_SMALL_K, U16_DOUBLE_QUOTE, U16_GREATER_THAN, U16_LESS_THAN, U16_SLASH, U16_SMALL_S, U16_SMALL_P, U16_SMALL_A, U16_SMALL_N, U16_GREATER_THAN]
            },
            vec![U16_LESS_THAN, U16_SLASH, U16_SMALL_D, U16_SMALL_I, U16_SMALL_V, U16_GREATER_THAN],
            line[3..].to_vec()
        ].concat()
    }

    else {
        line
    }

}


fn is_task_list(line: &Vec<u16>) -> bool {
    line.len() > 3 && line[0] == U16_LEFT_SQUARE_BRACKET && (line[1] == U16_SPACE || line[1] == U16_SMALL_X || line[1] == U16_BIG_X) && line[2] == U16_RIGHT_SQUARE_BRACKET && line[3] == U16_SPACE
}