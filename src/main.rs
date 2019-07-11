use std::time;

struct EditorSyntax {
    file_match: Vec<String>,
    keywords: Vec<String>,
    singleline_comment_start: String,
    multiline_comment_start: String,
    multiline_comment_end: String,
}

struct Row {
    index: usize,
    size: usize,
    rendered_size: usize,
    chars: String,  // actual content
    render: String, // "rendered" content
    hilight: Vec<char>,
    hilight_open_comment_check: bool, // does the row have open comment?
}

struct Colour {
    red: u8,
    green: u8,
    blue: u8,
}

struct Position {
    x: usize,
    y: usize,
}

struct EditorConfig {
    curson_posistion: Position,
    row_offset: usize,
    column_offset: usize,
    screen_rows: usize,
    screen_columns: usize,
    number_rows: usize,
    raw_mode: bool,
    rows: Vec<Row>,
    dirty: bool,
    filename: String,
    status_message: String,
    status_message_time: time::SystemTime,
    syntax: EditorSyntax,
}

enum KeyAction {
    KeyNull = 0,
    CtrlC = 3,
    CtrlD = 4,
    CtrlF = 6,
    CtrlH = 8,
    Tab = 9,
    CtrlL = 12,
    Enter = 13,
    CtrlQ = 17,
    CtrlS = 19,
    CtrlU = 21,
    Esc = 27,
    Backspace = 127,
    ArrowLeft = 1000,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    DelKey,
    HomeKey,
    EndKey,
    PageUp,
    PageDown,
}

const HilightNormal: u8 = 0;
const HilightNonPrint: u8 = 0;
const HilightComment: u8 = 0;
const HilightMultiComment: u8 = 0;
const HilightKeyword: u8 = 0;
const HilightString: u8 = 0;
const HilightNumber: u8 = 0;
const HilightMatch: u8 = 0; // Search match
                            // We support Rust and files with no explicit extension

impl EditorSyntax {
    fn new(
        file_match: Vec<String>,
        keywords: Vec<String>,
        singleline_comment_start: String,
        multiline_comment_start: String,
        multiline_comment_end: String,
    ) -> Self {
        Self {
            file_match,
            keywords,
            singleline_comment_start,
            multiline_comment_start,
            multiline_comment_end,
        }
    }
}

fn main() {
    // Yes.
    // I hate it too.
    let supported_hilight_extensions = vec![".rs".to_owned(), "".to_owned()];
    let keywords = vec![
        "as".to_owned(),
        "break".to_owned(),
        "const".to_owned(),
        "continue".to_owned(),
        "crate".to_owned(),
        "else".to_owned(),
        "enum".to_owned(),
        "extern".to_owned(),
        "false".to_owned(),
        "fn".to_owned(),
        "for".to_owned(),
        "if".to_owned(),
        "impl".to_owned(),
        "in".to_owned(),
        "let".to_owned(),
        "loop".to_owned(),
        "match".to_owned(),
        "mod".to_owned(),
        "move".to_owned(),
        "mut".to_owned(),
        "pub".to_owned(),
        "ref".to_owned(),
        "return".to_owned(),
        "self".to_owned(),
        "Self".to_owned(),
        "static".to_owned(),
        "struct".to_owned(),
        "super".to_owned(),
        "trait".to_owned(),
        "true".to_owned(),
        "type".to_owned(),
        "unsafe".to_owned(),
        "use".to_owned(),
        "where".to_owned(),
        "while".to_owned(),
        "abstract".to_owned(),
        "become".to_owned(),
        "box".to_owned(),
        "do".to_owned(),
        "final".to_owned(),
        "macro".to_owned(),
        "override".to_owned(),
        "priv".to_owned(),
        "typeof".to_owned(),
        "unsized".to_owned(),
        "virtual".to_owned(),
        "yield".to_owned(),
        "async".to_owned(),
        "await".to_owned(),
        "try".to_owned(),
    ];
    let special_characters = vec!["//".to_owned(), "/*".to_owned(), "*/".to_owned()];

    let syntax = EditorSyntax::new(
        supported_hilight_extensions,
        keywords,
        special_characters.get(0).unwrap().clone(),
        special_characters.get(1).unwrap().clone(),
        special_characters.get(2).unwrap().clone(),
    );
    println!("Hello, world!");
}
