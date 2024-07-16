/// Parser input
#[derive(Clone, Copy)]
struct S<'a> {
    src: &'a str,
    index: usize,
}

#[rustfmt::skip]
fn next(s: S) -> Option<(char, S)> {
    unsafe { s.src.get_unchecked(s.index..) }
        .chars()
        .next()
        .map(move |c| (c, S { index: s.index + c.len_utf8(), src: s.src}))
}

fn escaped<const N: usize>(s: S, is_bad: impl FnOnce(char) -> bool) -> Option<(char, S)> {
    match next(s) {
        Some(('\\', s)) => next(s),
        r @ Some((c, _)) => {
            if is_bad(c) {
                None
            } else {
                r
            }
        }
        r @ None => r,
    }
}

fn word(mut s: S) -> Option<(String, S)> {
    let mut result = String::new();
    while let Some((c, new_s)) = escaped(s, |c| "()".contains(c)) {
        if c.is_whitespace() {
            break;
        }
        s = new_s;
    }
    if result.is_empty() {}
}

pub struct Word {
    pub content: String,
}

pub struct Expression {
    pub function_name: Vec<Word>,
    pub inputs: Vec<String>,
}

pub struct Program {
    pub expressions: Vec<Expression>,
}

pub enum Error {}

pub fn parse(program: &str) -> Result<Program, Error> {}
