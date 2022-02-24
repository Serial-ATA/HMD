use crate::markdown::line::{Line, LineType};
use crate::markdown::inline::render_inline;
use crate::consts::*;


#[derive(Debug)]
enum TableAlignment {
    Left, Center, Right
}


impl TableAlignment {

    fn render(&self) -> Vec<u16> {
        match self {
            TableAlignment::Center => vec![U16_LESS_THAN, U16_SMALL_T, U16_SMALL_D, U16_SPACE, U16_SMALL_C, U16_SMALL_L, U16_SMALL_A, U16_SMALL_S, U16_SMALL_S, U16_EQUAL, U16_DOUBLE_QUOTE, U16_SMALL_A, U16_SMALL_L, U16_SMALL_I, U16_SMALL_G, U16_SMALL_N, U16_UNDERBAR, U16_SMALL_C, U16_SMALL_E, U16_SMALL_N, U16_SMALL_T, U16_SMALL_E, U16_SMALL_R, U16_DOUBLE_QUOTE, U16_GREATER_THAN],
            TableAlignment::Left => vec![U16_LESS_THAN, U16_SMALL_T, U16_SMALL_D, U16_SPACE, U16_SMALL_C, U16_SMALL_L, U16_SMALL_A, U16_SMALL_S, U16_SMALL_S, U16_EQUAL, U16_DOUBLE_QUOTE, U16_SMALL_A, U16_SMALL_L, U16_SMALL_I, U16_SMALL_G, U16_SMALL_N, U16_UNDERBAR, U16_SMALL_L, U16_SMALL_E, U16_SMALL_F, U16_SMALL_T, U16_DOUBLE_QUOTE, U16_GREATER_THAN],
            TableAlignment::Right => vec![U16_LESS_THAN, U16_SMALL_T, U16_SMALL_D, U16_SPACE, U16_SMALL_C, U16_SMALL_L, U16_SMALL_A, U16_SMALL_S, U16_SMALL_S, U16_EQUAL, U16_DOUBLE_QUOTE, U16_SMALL_A, U16_SMALL_L, U16_SMALL_I, U16_SMALL_G, U16_SMALL_N, U16_UNDERBAR, U16_SMALL_R, U16_SMALL_I, U16_SMALL_G, U16_SMALL_H, U16_SMALL_T, U16_DOUBLE_QUOTE, U16_GREATER_THAN],
        }
    }

}


pub fn merge_table(lines: &Vec<Line>) -> Vec<Line> {

    let mut result = Vec::with_capacity(lines.len());
    let mut is_inside_table = false;
    let mut curr_table = vec![];

    for (ind, ln) in lines.iter().enumerate() {

        if is_inside_table {

            if is_valid_table_row(ln) {
                curr_table.push(ln.clone());
            }

            else {
                result.push(construct_table(curr_table));
                result.push(ln.clone());
                curr_table = vec![];
                is_inside_table = false;
            }

        }

        else {

            if ln.line_type == LineType::TableDelimiter {

                if ind > 0 && is_valid_table_row(&lines[ind - 1]) && row_to_cells(&ln.content).len() == row_to_cells(&lines[ind - 1].content).len() {
                    curr_table.push(result.pop().unwrap());
                    curr_table.push(ln.clone());
                    is_inside_table = true;
                }

                else {
                    let mut ln_ = ln.clone();
                    ln_.line_type = LineType::Paragraph;
                    ln_ = render_inline(ln_);
                    result.push(ln_);
                }

            }

            else {
                result.push(ln.clone());
            }

        }

    }

    if is_inside_table {
        result.push(construct_table(curr_table));
    }

    result
}


fn construct_table(lines: Vec<Line>) -> Line {

    let table_head = row_to_cells(&lines[0].content);
    let table_body = lines[2..].iter().map(|ln| row_to_cells(&ln.content)).collect::<Vec<Vec<Vec<u16>>>>();
    let mut result = vec![];
    let alignments = row_to_cells(&lines[1].content).iter().map(|cell| parse_column_alignment(cell)).collect::<Vec<TableAlignment>>();

    result.push(vec![U16_LESS_THAN, U16_SMALL_T, U16_SMALL_A, U16_SMALL_B, U16_SMALL_L, U16_SMALL_E, U16_GREATER_THAN]);
    result.push(vec![U16_LESS_THAN, U16_SMALL_T, U16_SMALL_H, U16_SMALL_E, U16_SMALL_A, U16_SMALL_D, U16_GREATER_THAN]);

    for th in table_head.iter() {
        result.push(vec![U16_LESS_THAN, U16_SMALL_T, U16_SMALL_H, U16_GREATER_THAN]);
        result.push(th.clone());
        result.push(vec![U16_LESS_THAN, U16_SLASH, U16_SMALL_T, U16_SMALL_H, U16_GREATER_THAN]);
    }

    result.push(vec![U16_LESS_THAN, U16_SLASH, U16_SMALL_T, U16_SMALL_H, U16_SMALL_E, U16_SMALL_A, U16_SMALL_D, U16_GREATER_THAN]);
    result.push(vec![U16_LESS_THAN, U16_SMALL_T, U16_SMALL_B, U16_SMALL_O, U16_SMALL_D, U16_SMALL_Y, U16_GREATER_THAN]);

    for tr in table_body.iter() {
        result.push(vec![U16_LESS_THAN, U16_SMALL_T, U16_SMALL_R, U16_GREATER_THAN]);

        for (ind, td) in tr.iter().enumerate() {

            if ind == alignments.len() {
                break;
            }

            result.push(alignments[ind].render());
            result.push(td.clone());
            result.push(vec![U16_LESS_THAN, U16_SLASH, U16_SMALL_T, U16_SMALL_D, U16_GREATER_THAN]);
        }

        result.push(vec![U16_LESS_THAN, U16_SLASH, U16_SMALL_T, U16_SMALL_R, U16_GREATER_THAN]);
    }

    result.push(vec![U16_LESS_THAN, U16_SLASH, U16_SMALL_T, U16_SMALL_B, U16_SMALL_O, U16_SMALL_D, U16_SMALL_Y, U16_GREATER_THAN]);
    result.push(vec![U16_LESS_THAN, U16_SLASH, U16_SMALL_T, U16_SMALL_A, U16_SMALL_B, U16_SMALL_L, U16_SMALL_E, U16_GREATER_THAN]);

    Line {
        content: result.concat(),
        indent: 0,
        line_type: LineType::RenderedTable
    }
}


fn row_to_cells(row: &Vec<u16>) -> Vec<Vec<u16>> {

    let result = row.split(|c| *c == U16_VERTICAL_BAR).map(|c| c.to_vec()).collect::<Vec<Vec<u16>>>();

    result[1..result.len() - 1].to_vec()
}


fn parse_column_alignment(content: &Vec<u16>) -> TableAlignment {

    if content.len() < 2 {
        TableAlignment::Left
    }

    else if content[0] == U16_COLON {

        if content[content.len() - 1] == U16_COLON {
            TableAlignment::Center
        }

        else {
            TableAlignment::Left
        }

    }

    else if content[content.len() - 1] == U16_COLON {
        TableAlignment::Right
    }

    else {
        TableAlignment::Left
    }

}


fn is_valid_table_row(line: &Line) -> bool {
    (line.line_type == LineType::Paragraph || line.line_type == LineType::TableDelimiter)
    && line.content.len() > 2
    && line.content[0] == U16_VERTICAL_BAR && line.content[line.content.len() - 1] == U16_VERTICAL_BAR
}