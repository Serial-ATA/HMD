mod markdown;
mod consts;
mod utils;

use std::fs::File;
use std::io::{Read, Write};


pub fn from_file(input: &str, output: &str) {

    let mut f = File::open(input).unwrap();
    let mut st = String::new();

    f.read_to_string(&mut st).unwrap();
    let result = crate::markdown::render(&st).unwrap();

    let mut f = File::create(output).unwrap();
    f.write_all(result.as_bytes()).unwrap();
}


pub fn render(input: String) -> String {
    crate::markdown::render(&input).unwrap()
}


#[cfg(test)]
mod tests {

    #[test]
    fn header_test() {
        let md = "# header1\n## header2\n####### not-header".to_string();
        let html = "<h1 id=\"header1\">header1</h1>\n<h2 id=\"header2\">header2</h2>\n<p>####### not-header</p>";

        assert_eq!(
            html.trim_end_matches('\n'),
            crate::markdown::render(&md).unwrap().trim_end_matches('\n')
        );
    }

    #[test]
    fn thematic_break_test() {
        let md = "---\n---\n***\na".to_string();
        let html = "<hr />\n<hr />\n<hr />\n<p>a</p>";

        assert_eq!(
            html.trim_end_matches('\n'),
            crate::markdown::render(&md).unwrap().trim_end_matches('\n')
        );
    }

    #[test]
    fn escape_test() {
        let md = "<tag & test \\\\ \\a \\b \\c> '".to_string();
        let html = "<p>&lt;tag &amp; test \\ a b c> &apos;</p>";

        assert_eq!(
            html.trim_end_matches('\n'),
            crate::markdown::render(&md).unwrap().trim_end_matches('\n')
        );
    }

    #[test]
    fn paragraph_test() {
        let md = "p1\np1\n\np2\np2  \np2\n\np3\np3".to_string();
        let html = "<p>p1 p1</p>\n\n<p>p2 p2  <br />p2</p>\n\n<p>p3 p3</p>";

        assert_eq!(
            html.trim_end_matches('\n'),
            crate::markdown::render(&md).unwrap().trim_end_matches('\n')
        );
    }

    #[test]
    fn code_fence_test() {
        let md = "```\nfn main() {\n\tprintln!(\"Hello World!\");\n}\n```".to_string();
        let html = "<pre><code>fn main() {\n    println!(&quot;Hello World!&quot;);\n}</code></pre>";

        assert_eq!(
            html.trim_end_matches('\n'),
            crate::markdown::render(&md).unwrap().trim_end_matches('\n')
        );
    }

    #[test]
    fn blockquote_test() {
        let md = "> 1\n> 1\n>> 2\n>> 2\n 2\n>> 2\n>>> 3\n\n > 1\n1\n > 1".to_string();
        let html = "<blockquote>1 1 <blockquote>2 2 2 2 <blockquote>3 </blockquote></blockquote></blockquote>\n\n<blockquote>1 1 1 </blockquote>";

        assert_eq!(
            html.trim_end_matches('\n'),
            crate::markdown::render(&md).unwrap().trim_end_matches('\n')
        );
    }

    #[test]
    fn codespan_test() {
        let md = "You cannot use HTML character entities (like `&nbsp`, `&lt`, and ...)".to_string();
        let html = "<p>You cannot use HTML character entities (like <code class=\"short\">&amp;nbsp</code>, <code class=\"short\">&amp;lt</code>, and ...)</p>";

        assert_eq!(
            html.trim_end_matches('\n'),
            crate::markdown::render(&md).unwrap().trim_end_matches('\n')
        );
    }

    #[test]
    fn list_test() {
        let md = "- 1\n- 1\n  - 2\n  - 2\n    - 3\n    - 3\n  - 2\n  - 2\n- 1\n  - 2\n  - 2\n    - 3\n    - 3\n- 1\n- 1\n\n1. a\n1. a\n  i. b\n  i. b\n    I. c\n    I. c\n  i. b\n    - c\n    - c\n1. a".to_string();
        let html = "<ul><li>1</li><li>1<ul><li>2</li><li>2<ul><li>3</li><li>3</li></ul></li><li>2</li><li>2</li></ul></li><li>1<ul><li>2</li><li>2<ul><li>3</li><li>3</li></ul></li></ul></li><li>1</li><li>1</li></ul>\n\n<ol type=\"1\"><li>a</li><li>a<ol type=\"i\"><li>b</li><li>b<ol type=\"I\"><li>c</li><li>c</li></ol></li><li>b<ul><li>c</li><li>c</li></ul></li></ol></li><li>a</li></ol>";

        assert_eq!(
            html.trim_end_matches('\n'),
            crate::markdown::render(&md).unwrap().trim_end_matches('\n')
        );
    }

    #[test]
    fn inline_test() {
        let md = "*italic* **bold** ~~del~~ ^sup^ ~sub~ ~_underline_~".to_string();
        let html = "<p><em>italic</em> <strong>bold</strong> <del>del</del> <sup>sup</sup> <sub>sub</sub> <u>underline</u></p>";

        assert_eq!(
            html.trim_end_matches('\n'),
            crate::markdown::render(&md).unwrap().trim_end_matches('\n')
        );

        let md = "* invalid emphasis * ^invalid sup^ ~invalid sub~".to_string();
        let html = "<p>* invalid emphasis * ^invalid sup^ ~invalid sub~</p>";

        assert_eq!(
            html.trim_end_matches('\n'),
            crate::markdown::render(&md).unwrap().trim_end_matches('\n')
        );
    }

    #[test]
    fn file_test() {

        use std::fs::File;
        use std::io::{Read, Write};

        let mut f = File::open("README.md").unwrap();
        let mut st = String::new();

        f.read_to_string(&mut st).unwrap();
        let result = crate::markdown::render(&st).unwrap();

        let mut f = File::create("result.html").unwrap();
        f.write_all(result.as_bytes()).unwrap();
    }

}
