use crate::consts::*;


pub fn drop_while(v: &Vec<u16>, c: u16) -> Vec<u16> {

    let mut index = 0;

    while index < v.len() {

        if v[index] != c {
            break;
        }

        index += 1;
    }

    v[index..].to_vec()
}


pub fn take_while(v: &Vec<u16>, c: u16) -> Vec<u16> {

    let mut index = 0;

    while index < v.len() {

        if v[index] != c {
            break;
        }

        index += 1;
    }

    v[0..index].to_vec()
}


pub fn take_and_drop_while(v: &Vec<u16>, c: u16) -> (Vec<u16>, Vec<u16>) {

    let mut index = 0;

    while index < v.len() {

        if v[index] != c {
            break;
        }

        index += 1;
    }

    (v[0..index].to_vec(), v[index..].to_vec())
}


pub fn get_bracket_end_index(v: &Vec<u16>, index: usize) -> Option<usize> {
    get_partner_index(v, index, U16_LEFT_SQUARE_BRACKET, U16_RIGHT_SQUARE_BRACKET)
}


pub fn get_parenthesis_end_index(v: &Vec<u16>, index: usize) -> Option<usize> {
    get_partner_index(v, index, U16_LEFT_PARENTHESIS, U16_RIGHT_PARENTHESIS)
}


fn get_partner_index(v: &Vec<u16>, begin_index: usize, s: u16, p: u16) -> Option<usize> {

    let mut stack: i32 = 0;

    for index in begin_index..v.len() {

        if v[index] == s {
            stack += 1;
        }

        else if v[index] == p {
            stack -= 1;

            if stack == 0 {
                return Some(index);
            }

        }

    }

    None
}


pub fn remove_special_characters(line: &Vec<u16>) -> Vec<u16> {
    line.iter().filter(
        |c| &47 < *c && *c < &58 ||
        &64 < *c && *c < &91 ||
        &96 < *c && *c < &123 ||
        &44031 < *c && *c < &55203
    ).map(|c| *c).collect()
}


pub fn lowercase(c: u16) -> u16 {

    if U16_BIG_A <= c && c <= U16_BIG_Z {
        c + 32
    }

    else {
        c
    }

}


#[cfg(test)]
mod tests {

    #[test]
    fn partner_test() {
        use crate::utils::{get_parenthesis_end_index, get_bracket_end_index};
        let org = "[name](link)".encode_utf16().collect::<Vec<u16>>();

        assert_eq!(get_bracket_end_index(&org, 0), Some(5));
        assert_eq!(get_bracket_end_index(&org, 1), None);
        assert_eq!(get_parenthesis_end_index(&org, 6), Some(11));
        assert_eq!(get_parenthesis_end_index(&org, 7), None);
    }

}