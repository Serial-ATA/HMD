use crate::utils::{get_bracket_end_index, get_parenthesis_end_index, remove_special_characters};
use crate::markdown::escape::undo_html_escapes_safely;
use crate::markdown::inline::decoration::render_italic;
use crate::consts::*;


pub fn render_link(content: &Vec<u16>) -> Vec<u16> {

    let mut result = Vec::with_capacity(content.len());
    let mut curr_index = 0;

    while curr_index < content.len() {

        if is_valid_link(content, curr_index) {
            let link_end_index = get_bracket_end_index(content, curr_index).unwrap();

            // TODO: security check
            let ref_begin_index = link_end_index + 1;
            let ref_end_index = get_parenthesis_end_index(content, ref_begin_index).unwrap();

            let link = if curr_index > 0 && content[curr_index - 1] == U16_FACTORIAL {
                result.pop();  // `!`

                vec![
                    vec![U16_LESS_THAN, U16_SMALL_I, U16_SMALL_M, U16_SMALL_G, U16_SPACE, U16_SMALL_S, U16_SMALL_R, U16_SMALL_C, U16_EQUAL, U16_DOUBLE_QUOTE],
                    undo_html_escapes_safely(&content[ref_begin_index + 1..ref_end_index].to_vec()),
                    vec![U16_DOUBLE_QUOTE, U16_SPACE, U16_SMALL_A, U16_SMALL_L, U16_SMALL_T, U16_EQUAL, U16_DOUBLE_QUOTE],
                    remove_special_characters(&content[curr_index + 1..link_end_index].to_vec()),
                    vec![U16_DOUBLE_QUOTE, U16_SPACE, U16_SMALL_T, U16_SMALL_I, U16_SMALL_T, U16_SMALL_L, U16_SMALL_E, U16_EQUAL, U16_DOUBLE_QUOTE],
                    remove_special_characters(&content[curr_index + 1..link_end_index].to_vec()),
                    vec![U16_DOUBLE_QUOTE, U16_GREATER_THAN]
                ].concat()
            } else {
                vec![
                    vec![U16_LESS_THAN, U16_SMALL_A, U16_SPACE, U16_SMALL_H, U16_SMALL_R, U16_SMALL_E, U16_SMALL_F, U16_EQUAL, U16_DOUBLE_QUOTE],
                    undo_html_escapes_safely(&content[ref_begin_index + 1..ref_end_index].to_vec()),
                    vec![U16_DOUBLE_QUOTE, U16_GREATER_THAN],
                    content[curr_index + 1..link_end_index].to_vec(),
                    vec![U16_LESS_THAN, U16_SLASH, U16_SMALL_A, U16_GREATER_THAN]
                ].concat()
            };

            result = vec![result, link].concat();
            curr_index = ref_end_index;
        }

        else {
            result.push(content[curr_index]);
        }

        curr_index += 1;
    }

    render_italic(&result)
}


fn is_valid_link(content: &Vec<u16>, index: usize) -> bool {

    content[index] == U16_LEFT_SQUARE_BRACKET && match get_bracket_end_index(content, index) {
        None => false,
        Some(i) => {
            i + 1 < content.len() && content[i + 1] == U16_LEFT_PARENTHESIS && match get_parenthesis_end_index(content, i + 1) {
                None => false,
                Some(_) => true
            }
        }
    }
}