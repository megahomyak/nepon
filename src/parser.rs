/// Parser input
#[derive(Clone, Copy)]
pub struct S<'a> {
    pub src: &'a str,
    pub index: usize,
}

#[rustfmt::skip]
pub fn next(s: S) -> Option<(char, S)> {
    s.src.get(s.index..)
        .unwrap()
        .chars()
        .next()
        .map(move |c| (c, S { index: s.index + c.len_utf8(), src: s.src}))
}

pub enum Char {
    Escaped(char),
    Regular(char),
}

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
pub fn command_name(mut s: S) -> Option<(String, S)> {
    let mut result = String::new();
    while let Some((c, new_s)) = next(s) {
        if "()\n;".contains(c) {
            break;
        }
        result.push(c);
        s = new_s;
    }
    result = result.trim_end_matches(char::is_whitespace).to_owned();
    if result.is_empty() {
        None
    } else {
        result.shrink_to_fit();
        Some((result, s))
    }
}

/// blah blah [(...)]
pub fn input(mut s: S) -> Result<(String, S), Option<Error>> {
    let mut result = String::new();
    let mut paren_count = 1;
    let str_at_beginning = s;
    match next(s) {
        Some(('(', new_s)) => s = new_s,
        _ => return Err(None),
    }
    loop {
        let str_at_current_char = s;
        match next(s) {
            Some((c, new_s)) => {
                s = new_s;
                match c {
                    '(' => {
                        paren_count += 1;
                        result.push('(');
                    }
                    ')' => {
                        paren_count -= 1;
                        if paren_count == 0 {
                            result.shrink_to_fit();
                            return Ok((result, s));
                        }
                        result.push(')');
                    }
                    '\\' => match next(s) {
                        Some((c, new_s)) => {
                            result.push('\\');
                            result.push(c);
                            s = new_s;
                        }
                        None => {
                            return Err(Some(Error::NothingAfterEscapeCharacter {
                                escape_character_index: str_at_current_char.index,
                            }))
                        }
                    },
                    _ => result.push(c),
                }
            }
            None => {
                return Err(Some(Error::MissingClosingParen {
                    index: str_at_beginning.index,
                }))
            }
        }
    }
}

pub struct Expression {
    pub command_name: String,
    pub inputs: Vec<String>,
}

pub fn expression(mut s: S) -> Result<(Expression, S), Option<Error>> {
    let str_before_command_name = s;
    let command_name = match command_name(s) {
        None => match next(s) {
            None => return Err(None),
            Some(_) => {
                return Err(Some(Error::InputWithoutCommandName {
                    index: str_before_command_name.index,
                }))
            }
        },
        Some((fname, new_s)) => {
            s = new_s;
            fname
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
                    Expression {
                        command_name,
                        inputs,
                    },
                    s,
                ))
            }
            Ok((input, new_s)) => {
                inputs.push(input);
                s = new_s;
            }
        }
    }
}

pub struct Program {
    pub expressions: Vec<Expression>,
}

pub enum Error {
    MissingClosingParen { index: usize },
    InputWithoutCommandName { index: usize },
    UnexpectedClosingParen { index: usize },
    NothingAfterEscapeCharacter { escape_character_index: usize },
}

pub fn program(mut s: S) -> Result<Program, Error> {
    let mut expressions = Vec::new();
    loop {
        s = skip_whitespace(s);
        match expression(s) {
            Err(Some(e)) => return Err(e),
            Err(None) => match next(s) {
                None => return Ok(Program { expressions }),
                Some((c, new_s)) => {
                    if !"\n;".contains(c) {
                        return Err(Error::UnexpectedClosingParen { index: s.index });
                    }
                    s = new_s;
                }
            },
            Ok((expression, new_s)) => {
                expressions.push(expression);
                s = new_s;
            }
        }
    }
}
