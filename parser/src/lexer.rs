use std::{iter::Peekable, str::CharIndices};

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;
type LexerItem<'input> = Spanned<Tok<'input>, usize, LexicalError>;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Tok<'input> {
    Whitespace,
    LeftParen,
    RightParen,
    Quote,
    Symbol(&'input str),
    Str(&'input str),
    Num(&'input str),
    Op(&'input str),
}

#[derive(Debug, PartialEq, Eq)]
pub enum LexicalError {
    // Not possible
}

pub struct Lexer<'input> {
    chars: Peekable<CharIndices<'input>>,
    input: &'input str,
}

impl<'input> Lexer<'input> {
    #[must_use]
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.char_indices().peekable(),
            input,
        }
    }

    fn consume(&mut self) -> Option<(usize, char)> {
        return self.chars.next();
    }

    fn consume_while(
        &mut self,
        start_idx: usize,
        condition: impl Fn(char) -> bool,
        tok_type: Tok,
    ) -> Option<LexerItem<'input>> {
        let mut end_idx = start_idx + 1;

        let output_lexer_item = |new_end_idx: usize| -> Option<LexerItem<'input>> {
            let tok = match tok_type {
                Tok::Symbol(_) => Tok::Symbol(&self.input[start_idx..new_end_idx]),
                Tok::Str(_) => Tok::Str(&self.input[start_idx..new_end_idx]),
                Tok::Num(_) => Tok::Num(&self.input[start_idx..new_end_idx]),
                Tok::Op(_) => Tok::Op(&self.input[start_idx..new_end_idx]),
                Tok::Whitespace => Tok::Whitespace,
                Tok::LeftParen => Tok::LeftParen,
                Tok::RightParen => Tok::RightParen,
                Tok::Quote => Tok::Quote,
            };

            return Some(Ok((start_idx, tok, new_end_idx)));
        };

        loop {
            match self.chars.peek() {
                // Condition fulfilled
                Some((idx, c)) if !condition(*c) => {
                    end_idx = *idx;
                    return output_lexer_item(end_idx);
                }
                // String end
                None => {
                    end_idx += 1;
                    return output_lexer_item(end_idx);
                }
                // Skip char
                Some((idx, _)) => {
                    end_idx = *idx;
                    self.consume()?;
                }
            }
        }
    }

    fn consume_whitespace(&mut self, start_idx: usize) -> Option<LexerItem<'input>> {
        self.consume_while(start_idx, Lexer::is_whitespace, Tok::Whitespace)
    }

    fn consume_symbol(&mut self, start_idx: usize) -> Option<LexerItem<'input>> {
        self.consume_while(start_idx, Lexer::is_symbol_char, Tok::Symbol(""))
    }

    fn consume_num(&mut self, start_idx: usize) -> Option<LexerItem<'input>> {
        self.consume_while(start_idx, Lexer::is_decimal_digit, Tok::Num(""))
    }

    fn consume_string(&mut self, start_idx: usize) -> Option<LexerItem<'input>> {
        _ = self.consume()?; // consume leading "
        let res = self.consume_while(
            start_idx + 1,
            |c: char| -> bool { !Lexer::is_string_quote(c) },
            Tok::Str(""),
        );
        _ = self.consume()?; // consume trailing "

        return res;
    }

    // In case we change our minds later
    #[must_use]
    pub fn is_symbol_char(ch: char) -> bool {
        !Self::is_whitespace(ch)
            && !Self::is_quote(ch)
            && !Self::is_string_quote(ch)
            && ch != '('
            && ch != ')'
    }

    #[must_use]
    pub const fn is_decimal_digit(ch: char) -> bool {
        ch.is_ascii_digit()
    }

    #[must_use]
    pub const fn is_string_quote(ch: char) -> bool {
        '"' == ch
    }

    #[must_use]
    pub const fn is_quote(ch: char) -> bool {
        '\'' == ch
    }

    #[must_use]
    pub fn is_whitespace(c: char) -> bool {
        const WHITESPACE_CHARS: [char; 3] = [' ', '\n', '\t'];
        return WHITESPACE_CHARS.contains(&c);
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = LexerItem<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let ch = self.chars.peek().copied();
            match ch {
                Some((i, '(')) => {
                    _ = self.consume();
                    return Some(Ok((i, Tok::LeftParen, i + 1)));
                }
                Some((i, ')')) => {
                    _ = self.consume();
                    return Some(Ok((i, Tok::RightParen, i + 1)));
                }
                Some((i, '\'')) => {
                    _ = self.consume();
                    return Some(Ok((i, Tok::Quote, i + 1)));
                }
                Some((i, c)) if Lexer::is_string_quote(c) => return self.consume_string(i),
                Some((i, c)) if Lexer::is_decimal_digit(c) => return self.consume_num(i),
                Some((i, c)) if Lexer::is_symbol_char(c) => return self.consume_symbol(i),
                Some((i, c)) if Lexer::is_whitespace(c) => {
                    self.consume_whitespace(i);
                    // This skips the whitespace instead of creating a token for it
                    continue;
                    // return Some(Ok((i, Tok::Whitespace, i + 1)));
                }
                None => return None, // End of file
                Some((i, c)) => panic!("Tokenizer: invalid token {c} at {i}"),
            }
        }
    }
}

#[test]
fn lexer_works_properly() {
    let source = "(atom 10 \"string\" '(1 2 3) string-append)";

    Lexer::new(source).for_each(|t| println!("{:?}", t.unwrap().1));

    let mut lex = Lexer::new(source);
    assert_eq!(lex.next().unwrap().unwrap().1, Tok::LeftParen);
    assert_eq!(lex.next().unwrap().unwrap().1, Tok::Symbol("atom"));
    assert_eq!(lex.next().unwrap().unwrap().1, Tok::Num("10"));
    assert_eq!(lex.next().unwrap().unwrap().1, Tok::Str("string"));

    assert_eq!(lex.next().unwrap().unwrap().1, Tok::Quote);
    assert_eq!(lex.next().unwrap().unwrap().1, Tok::LeftParen);
    assert_eq!(lex.next().unwrap().unwrap().1, Tok::Num("1"));
    assert_eq!(lex.next().unwrap().unwrap().1, Tok::Num("2"));
    assert_eq!(lex.next().unwrap().unwrap().1, Tok::Num("3"));
    assert_eq!(lex.next().unwrap().unwrap().1, Tok::RightParen);

    assert_eq!(lex.next().unwrap().unwrap().1, Tok::Symbol("string-append"));

    assert_eq!(lex.next().unwrap().unwrap().1, Tok::RightParen);

    assert_eq!(Lexer::new(source).count(), 12);
}
