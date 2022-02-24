mod render;

use lazy_static::lazy_static;
use std::collections::HashMap;


lazy_static! {
    static ref ICONS: HashMap<Vec<u16>, Vec<u16>> = self::render::data();
}


pub fn get_icon(name: &Vec<u16>, size: usize, r: u8, g: u8, b: u8) -> Option<Vec<u16>> {

    match ICONS.get(name) {
        Some(s) => Some(self::render::format(s, size, r, g, b)),
        _ => None
    }

}


#[cfg(test)]
mod tests {

    #[test]
    fn icon_test() {
        panic!("{}", String::from_utf16_lossy(&crate::markdown::icon::get_icon(&vec![99], 20, 0, 0, 0).unwrap()));
    }

}