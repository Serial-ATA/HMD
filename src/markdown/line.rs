use crate::consts::*;
use crate::utils::*;
use crate::markdown::predicates::get_type;


#[derive(Clone, Debug, PartialEq)]
pub enum LineType {
    Paragraph,
    Header,
    ThematicBreak,
    CodeFence,
    TableDelimiter,
    FencedCode,
    RenderedTable,
    Blockquote(usize),
    UnorderedList,
    Tag,
    OrderedList,
    Empty
}


#[derive(Clone)]
pub struct Line {
    pub content: Vec<u16>,
    pub indent: usize,
    pub line_type: LineType
}


impl Line {

    pub fn from_raw(raw: &Vec<u16>) -> Line {

        let mut indent = 0;
        let mut index = 0;

        while index < raw.len() {

            if raw[index] == U16_SPACE {
                indent += 1;
            }

            else if raw[index] == U16_TAB {
                indent += 4;
            }

            else {
                break;
            }

            index += 1;
        }

        Line {
            content: raw[index..].to_vec(),
            indent,
            line_type: LineType::Paragraph
        }
    }

    pub fn into_raw(&self) -> Vec<u16> {

        vec![vec![U16_SPACE;self.indent], self.content.clone()].concat()
    }

    pub fn render(&self) -> Vec<u16> {

        match self.line_type {
            LineType::Paragraph => self.render_paragraph(),
            LineType::Header => self.render_header(),
            LineType::ThematicBreak => vec![U16_LESS_THAN, U16_SMALL_H, U16_SMALL_R, U16_SPACE, U16_SLASH, U16_GREATER_THAN],
            LineType::Empty => vec![],
            LineType::FencedCode => self.render_fenced_code(),
            LineType::Blockquote(_) => self.render_as_it_is(),
            LineType::OrderedList => self.render_as_it_is(),
            LineType::Tag => self.render_as_it_is(),
            LineType::UnorderedList => self.render_as_it_is(),
            LineType::RenderedTable => self.render_as_it_is(),
            _ => panic!("{:?}", self.line_type)
        }

    }

    fn render_paragraph(&self) -> Vec<u16> {

        vec![
            vec![U16_LESS_THAN, U16_SMALL_P, U16_GREATER_THAN],
            self.content.clone(),
            vec![U16_LESS_THAN, U16_SLASH, U16_SMALL_P, U16_GREATER_THAN],
        ].concat()
    }

    fn render_as_it_is(&self) -> Vec<u16> {
        self.content.clone()
    }

    fn render_header(&self) -> Vec<u16> {

        let (sharps, sharps_removed) = take_and_drop_while(&self.content, U16_SHARP);
        let indents_removed = drop_while(&sharps_removed, U16_SPACE);

        vec![
            vec![U16_LESS_THAN, U16_SMALL_H, sharps.len() as u16 + 48, U16_SPACE, U16_SMALL_I, U16_SMALL_D, U16_EQUAL, U16_DOUBLE_QUOTE],
            remove_special_characters(&indents_removed),
            vec![U16_DOUBLE_QUOTE, U16_GREATER_THAN],
            indents_removed,
            vec![U16_LESS_THAN, U16_SLASH, U16_SMALL_H, sharps.len() as u16 + 48, U16_GREATER_THAN]
        ].concat()
    }

    fn render_fenced_code(&self) -> Vec<u16> {

        vec![
            vec![U16_LESS_THAN, U16_SMALL_P, U16_SMALL_R, U16_SMALL_E, U16_GREATER_THAN, U16_LESS_THAN, U16_SMALL_C, U16_SMALL_O, U16_SMALL_D, U16_SMALL_E, U16_GREATER_THAN],
            self.content.clone(),
            vec![U16_LESS_THAN, U16_SLASH, U16_SMALL_C, U16_SMALL_O, U16_SMALL_D, U16_SMALL_E, U16_GREATER_THAN, U16_LESS_THAN, U16_SLASH, U16_SMALL_P, U16_SMALL_R, U16_SMALL_E, U16_GREATER_THAN]
        ].concat()
    }

}


pub fn code_to_lines(code: &Vec<u16>) -> Vec<Line> {

    code.split(
        |c|
        *c == U16_NEWLINE
    ).map(
        |ln| {
            let mut untyped = Line::from_raw(&ln.to_vec());

            let line_type = get_type(&untyped);
            untyped.line_type = line_type;

            untyped
        }
    ).collect::<Vec<Line>>()
}


pub fn render_lines(lines: &Vec<Line>) -> Vec<u16> {

    let mut result = Vec::with_capacity(lines.len() * 2);

    for ln in lines.iter() {
        result.push(ln.render());
        result.push(vec![U16_NEWLINE]);
    }

    result.concat()
}