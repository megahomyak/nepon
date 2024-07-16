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

pub fn escaped(s: S, is_bad: impl FnOnce(char) -> bool) -> Option<(Char, S)> {
    match next(s) {
        Some(('\\', s)) => next(s).map(|(c, s)| (Char::Escaped(c), s)),
        Some((c, s)) => {
            if is_bad(c) {
                None
            } else {
                Some((Char::Regular(c), s))
            }
        }
        None => None,
    }
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
    while let Some((c, new_s)) = escaped(s, |c| "()\n;".contains(c)) {
        result.push(match c {
            Char::Escaped(c) => c,
            Char::Regular(c) => c,
        });
        s = new_s;
    }
    result = result.trim_end_matches(char::is_whitespace).to_owned();
    if result.is_empty() {
        None
    } else {
        Some((result, s))
    }
}

/// blah blah [(...)]
pub fn input(mut s: S) -> Result<(String, S), Option<Error>> {
    let mut result = String::new();
    let mut paren_count = 1;
    let str_before_next = s;
    match next(s) {
        Some(('(', new_s)) => s = new_s,
        _ => return Err(None),
    }
    loop {
        match escaped(s, |c| "()".contains(c)) {
            Some((c, new_s)) => {
                match c {
                    Char::Regular(c) => result.push(c),
                    Char::Escaped(c) => {
                        result.push('\\');
                        result.push(c);
                    }
                }
                s = new_s;
            }
            None => match next(s) {
                Some((c, new_s)) => {
                    if c == '(' {
                        paren_count += 1;
                        result.push('(');
                    } else {
                        paren_count -= 1;
                        if paren_count == 0 {
                            return Ok((result, new_s));
                        }
                        result.push(')');
                    }
                    s = new_s;
                }
                None => {
                    return Err(Some(Error::MissingClosingParen {
                        index: str_before_next.index,
                    }))
                }
            },
        }
    }
}

pub struct Expression {
    pub command_name: String,
    pub inputs: Vec<String>,
}

pub fn expression(s: S) -> Result<(Expression, S), Option<Error>> {
    let mut s = skip_whitespace(s);
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
            Err(None) => return Ok((Expression { command_name, inputs }, s)),
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
}

pub fn program(s: S) -> Result<Program, Error> {
    let mut expressions = Vec::new();
    // '\n' ';'
    // check for something left behind. if any, that's UnexpectedClosingParen
}
