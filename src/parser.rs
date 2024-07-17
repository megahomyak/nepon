mod s {
    /// Parser input
    #[derive(Clone, Copy, Debug)]
    pub struct S<'a> {
        src: &'a str,
        index: usize,
    }

    impl<'a> From<&'a str> for S<'a> {
        fn from(src: &'a str) -> Self {
            Self { src, index: 0 }
        }
    }

    pub fn row_col_line<'a>(s: &'a S) -> (usize, usize, &'a str) {
        let mut line_start_idx = 0;
        let mut row = 1;
        let mut col = 1;
        let mut char_indices = s.src.char_indices();
        let mut found = false;
        for (i, c) in &mut char_indices {
            if i == s.index {
                found = true;
            }
            if c == '\n' {
                if found {
                    return (row, col, unsafe { s.src.get_unchecked(line_start_idx..i) });
                } else {
                    line_start_idx = i;
                    col = 1;
                    row += 1;
                }
            } else if !found {
                col += 1;
            }
        }
        (row, col, unsafe { s.src.get_unchecked(line_start_idx..) })
    }

    #[rustfmt::skip]
    pub fn next(s: S) -> Option<(char, S)> {
        unsafe { s.src.get_unchecked(s.index..) }
            .chars()
            .next()
            .map(move |c| (c, S { index: s.index + c.len_utf8(), src: s.src}))
    }
}
pub use s::*;

pub fn skip_whitespace(mut s: S) -> S {
    while let Some((c, new_s)) = next(s) {
        if !c.is_whitespace() {
            break;
        }
        s = new_s;
    }
    s
}

/// [blah blah] (...)
pub fn command_name(mut s: S) -> Option<(CommandName, S)> {
    let mut content = String::new();
    let beginning_pos = s;
    while let Some((c, new_s)) = next(s) {
        if "()\n;".contains(c) {
            break;
        }
        content.push(c);
        s = new_s;
    }
    content = content.trim_end_matches(char::is_whitespace).to_owned();
    if content.is_empty() {
        None
    } else {
        content.shrink_to_fit();
        Some((
            CommandName {
                content,
                beginning_pos,
            },
            s,
        ))
    }
}

/// blah blah [(...)]
pub fn input(mut s: S) -> Result<(Input, S), Option<Error>> {
    let mut content = String::new();
    let mut paren_count = 1;
    let opening_paren_pos = s;
    match next(s) {
        Some(('(', new_s)) => s = new_s,
        _ => return Err(None),
    }
    loop {
        let esc_char_pos = s;
        match next(s) {
            Some((c, new_s)) => {
                s = new_s;
                match c {
                    '(' => {
                        paren_count += 1;
                        content.push('(');
                    }
                    ')' => {
                        paren_count -= 1;
                        if paren_count == 0 {
                            content.shrink_to_fit();
                            return Ok((
                                Input {
                                    content,
                                    opening_paren_pos,
                                },
                                s,
                            ));
                        }
                        content.push(')');
                    }
                    '\\' => match next(s) {
                        Some((c, new_s)) => {
                            content.push('\\');
                            content.push(c);
                            s = new_s;
                        }
                        None => {
                            return Err(Some(Error::NothingAfterEscapeCharacter { esc_char_pos }))
                        }
                    },
                    _ => content.push(c),
                }
            }
            None => return Err(Some(Error::MissingClosingParen { opening_paren_pos })),
        }
    }
}

#[derive(Debug)]
pub struct CommandName<'a> {
    pub beginning_pos: S<'a>,
    pub content: String,
}

#[derive(Debug)]
pub struct Input<'a> {
    pub opening_paren_pos: S<'a>,
    pub content: String,
}

#[derive(Debug)]
pub struct Line<'a> {
    pub command_name: CommandName<'a>,
    pub inputs: Vec<Input<'a>>,
}

pub fn line(mut s: S) -> Result<(Line, S), Option<Error>> {
    let beginning_pos = s;
    let command_name = match command_name(s) {
        None => match next(s) {
            None => return Err(None),
            Some((c, _s)) => {
                if c == '(' {
                    return Err(Some(Error::InputWithoutCommandName {
                        opening_paren_pos: beginning_pos,
                    }));
                } else if c == ';' {
                    return Err(None);
                } else {
                    return Err(Some(Error::UnexpectedClosingParen {
                        closing_paren_pos: beginning_pos,
                    }));
                }
            }
        },
        Some((command_name, new_s)) => {
            s = new_s;
            command_name
        }
    };
    let mut inputs = Vec::new();
    loop {
        s = skip_whitespace(s);
        match input(s) {
            Err(Some(e)) => return Err(Some(e)),
            Err(None) => {
                inputs.shrink_to_fit();
                return Ok((
                    Line {
                        command_name,
                        inputs,
                    },
                    s,
                ));
            }
            Ok((input, new_s)) => {
                inputs.push(input);
                s = new_s;
            }
        }
    }
}

#[derive(Debug)]
pub struct Program<'a> {
    pub lines: Vec<Line<'a>>,
}

#[derive(Debug)]
pub enum Error<'a> {
    MissingClosingParen { opening_paren_pos: S<'a> },
    InputWithoutCommandName { opening_paren_pos: S<'a> },
    UnexpectedClosingParen { closing_paren_pos: S<'a> },
    NothingAfterEscapeCharacter { esc_char_pos: S<'a> },
}

pub fn program(mut s: S) -> Result<Program, Error> {
    let mut lines = Vec::new();
    loop {
        s = skip_whitespace(s);
        match line(s) {
            Err(Some(e)) => return Err(e),
            Err(None) => match next(s) {
                None => return Ok(Program { lines }),
                Some((c, new_s)) => {
                    if !"\n;".contains(c) {
                        return Err(Error::UnexpectedClosingParen {
                            closing_paren_pos: s,
                        });
                    }
                    s = new_s;
                }
            },
            Ok((line, new_s)) => {
                lines.push(line);
                s = new_s;
            }
        }
    }
}
