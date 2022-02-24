# H Markdown Engine

This is a markdown engine developed for my personal blog. The syntax is dialect of [gfm](https://github.github.com/gfm), but not subset or superset of it.

*If you're reading this document on github, some elements are not rendered properly.* Please visit my [blog](https://www.google.com/search?channel=fs&q=not+implemented+yet) to see the correct version.

This document is mainly focused on its markdown syntax. If you want to use this engine for your blog, read the [crate's document](https://www.google.com/search?channel=fs&q=not+implemented+yet).

## Things that are similar to gfm

Don't try edge cases! There must be tons of glitches.

- [Thematic breaks](https://github.github.com/gfm/#thematic-breaks)
- [Links](https://github.github.com/gfm/#links)
- [Blockquotes](https://github.github.com/gfm/#block-quotes)
- [ATX headings](https://github.github.com/gfm/#atx-headings)
- [Task list items](https://github.github.com/gfm/#task-list-items-extension-)
  - List syntax is a bit different. Read [here](#Lists).
- [Strikethroughs](https://github.github.com/gfm/#strikethrough-extension-)
- [Images](https://github.github.com/gfm/#images)

## Things that are missing

### Headers

- [Setext headings](https://github.github.com/gfm/#setext-heading) are not supported.

### Code blocks

- [Indented codeblocks](https://github.github.com/gfm/#indented-code-blocks) are not supported.

### Autolinks

Not at all.

### Raw HTMLs

- Inserting html tags is not supported, at all!
- You cannot use HTML character entities (like `&nbsp`, `&lt`, and ...).
  - If you want to type `<`, `>`, or `&`, just type as it is.
  - If you want multiple blank spaces, see [here](#Blank)
- `<details>` tag is not supported, but it'll be added soon.
- HTML comment `<!-- -->` does not work here.
  - If you want to use HTML comment tricks to divide a list, use an empty line instead.

## Things that are different

### Tables

Table rows and delimiter rows resemble that of [gfm's table](https://github.github.com/gfm/#tables-extension-), but a bit more strict. Each cell must be enclosed by two pipes (`|`), including the first and the last cell. You can also set its alignment using colons.

|Left aligned Column |Centered Column |Right aligned Column |
|:-------------------|:--------------:|--------------------:|
|        Left        |     Center     |        Right        |
|        Left        |     Center     |        Right        |
|        Left        |     Center     |        Right        |
|        Left        |     Center     |        Right        |
|        Left        |     Center     |        Right        |

### Code spans

It has limited range of syntax.

- Inline code spans use single backtick character. Multiple backtick characters're not treated as code spans.
- If you want backticks inside a code span, that's impossible.

### Fenced code blocks

- Use backticks for delimiter, not tildes.
 - A code fence should not be indented.
- A line beginning with 3 or more backticks is counted as a fence.
  - Two fences with a different number of backticks can enclose a fenced code block.
- The engine uses [syntect](https://github.com/trishume/syntect) crate for syntax highlighting.
- You can enable line-numbering with `line_num(n)` option. The number `n` designates the first index.

```
 ```rust, line_num(0)
 fn main() {
     println!("Hello World!");
 }

 fn add_one(n: i32) -> bool {
     n + 1
 }
 ```
```

```rust, line_num(0)
fn main() {
    println!("Hello World!");
}

fn add_one(n: i32) -> bool {
    n + 1
}
```

### Hard line breaks

- For [hard line breaks](https://github.github.com/gfm/#hard-line-break), use multiple spaces, not backslashes.

### Emphasis

- This engine only uses `*` for emphasis, not `_`.
  - One for *italic* and two for **bold**.

### Lists

- `*` is not a valid bullet for unordered lists, only `-` is.
- `1.`, `i.`, `I.`, `a.`, and `A.` are the only valid bullets for ordered lists.
  - Which means you cannot set its starting number.
- There must be a space between the bullet and its content.

## Things that are added

Some of these extensions are from [pandoc](https://pandoc.org)'s markdown spec. The others are my custom extensions.

### Underlines

`~_Underlines_~` is rendered to ~_Underlines_~. Underlines may not contain any newline. The first and the last character may not be space.

### Subscript

`H~2~O` is rendered to H~2~O. Subscripts may not contain any space or newline.

### Superscript

`E=mc^2^` is rendered to E=mc^2^. Superscripts may not contain any space or newline.

### Extra tags

HMD has wide variety of tags. It uses double square brackets instead of HTML's angle brackets. All the spaces inside the brackets are ignored.

Some tags have to be properly closed, or it would ruin the rendered html file! The engine does not check whether a tag is closed, so you have to take care of that. Also, be careful not to mix extra tags with other html tags. Most extra tags generate `<div>` or `<span>` tags when rendered. Those rendered tags can be mixed with auto-generated `<p>` tags. To prevent that you should either

- Open and close a tag within a paragraph.
- Don't use a tag as an inline element.

If you do something like below,

```
[[box]] A paragraph

Another paragraph [[/box]]
```

the result would be like below.

```html
<p><div class="box">A paragraph</p>
<p>Another paragraph</div></p>
```

It would look fine on most browsers, but not desirable though.

#### Box

It's rendered to `<div>` tag.

`[[box]][[/box]]` draws a box. You can put (almost) everything inside a box. You can even nest boxes!

[[box]]

[[center]][[big]]A box.[[/big]][[/center]]

|A table inside a box.|
|--------------------|
|[[box]] A box inside a table. [[/box]]|
|But you cannot have a table inside a box inside a table.|
|Because you cannot put a table in another table.|

[[/box]]

#### Font size

It's rendered to `<span>` tag.

|                 HMD code                 |              Rendered result              |
|------------------------------------------|-------------------------------------------|
| `[[big]]Big font[[/big]]`                | [[big]]Big font[[/big]]                   |
| `[[medium]]Medium font[[/medium]]`       | [[medium]]Medium font[[/medium]]          |
| `[[small]]Small font[[/small]]`          | [[small]]Small font[[/small]]             |
| `[[Big]]It's case insensitive![[/Big]]`  | [[Big]]It's case insensitive![[/Big]]     |

#### Font color

It's rendered to `<span>` tag.

|                 HMD code                 |             Rendered result             |
|------------------------------------------|-----------------------------------------|
| `[[red]]rgb(192, 32, 32)[[/red]]`        | [[red]]rgb(192, 32, 32)[[/red]]         |
| `[[orange]]rgb(255, 165, 0)[[/orange]]`  | [[orange]]rgb(255, 165, 0)[[/orange]]   |
| `[[aqua]]rgb(64, 192, 192)[[/aqua]]`     | [[aqua]]rgb(64, 192, 192)[[/aqua]]      |
| `[[green]]rgb(32, 192, 32)[[/green]]`    | [[green]]rgb(32, 192, 32)[[/green]]     |
| `[[blue]]rgb(32, 128, 255)[[/blue]]`     | [[blue]]rgb(32, 128, 255)[[/blue]]      |
| `[[lime]]rgb(0, 255, 0)[[/lime]]`        | [[lime]]rgb(0, 255, 0)[[/lime]]         |
| `[[yellow]]rgb(192, 192, 32)[[/yellow]]` | [[yellow]]rgb(192, 192, 32)[[/yellow]]  |
| `[[violet]]rgb(187, 134, 252)[[/violet]]`| [[violet]]rgb(187, 134, 252)[[/violet]] |
| `[[white]]rgb(255, 255, 255)[[/white]]`  | [[white]]rgb(255, 255, 255)[[/white]]   |

#### Alignment

`[[center]]`, `[[left]]`, `[[right]]` tags align (almost) everything. They're rendered to `<div>` tag. So, don't forget to close them.

[[center]]

A centered text,

[[/center]]
[[right]]

[[box]] and a right aligned box.[[/box]]

[[/right]]

#### Icons

You can insert icons with `[[icon]]` tag, but it's not implemented yet.

#### Blank

`[[blank]]` is rendered to `&nbsp;`.

## TODO

- [ ] Security
  - [X] Escape chars
  - [ ] check links
- [ ] fenced code block within a list
- [ ] SVG
- [ ] Footnotes
- [ ] Auto-index headers
- [X] Render custom shapes for task lists
- [ ] Math formula
- [X] `[[blank]]`
- [ ] *Copy to clipboard* button for fenced codes
- [ ] Better name instead of *extra tag*
- [ ] Tests
  - [ ] More tests