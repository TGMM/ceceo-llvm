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
    HashSymbol(&'input str),
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
                Tok::HashSymbol(_) => Tok::HashSymbol(&self.input[start_idx..new_end_idx]),
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

    fn consume_hash_symbol(&mut self, start_idx: usize) -> Option<LexerItem<'input>> {
        self.consume_while(start_idx, Lexer::is_symbol_char, Tok::HashSymbol(""))
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

    #[allow(clippy::unnecessary_wraps)]
    fn consume_single_char(
        &mut self,
        start_idx: usize,
        tok: Tok<'input>,
    ) -> Option<LexerItem<'input>> {
        _ = self.consume();
        return Some(Ok((start_idx, tok, start_idx + 1)));
    }

    fn consume_left_paren(&mut self, start_idx: usize) -> Option<LexerItem<'input>> {
        self.consume_single_char(start_idx, Tok::LeftParen)
    }

    fn consume_right_paren(&mut self, start_idx: usize) -> Option<LexerItem<'input>> {
        self.consume_single_char(start_idx, Tok::RightParen)
    }

    fn consume_quote(&mut self, start_idx: usize) -> Option<LexerItem<'input>> {
        self.consume_single_char(start_idx, Tok::Quote)
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
    pub const fn is_hash_char(ch: char) -> bool {
        '#' == ch
    }

    #[must_use]
    pub fn is_whitespace(ch: char) -> bool {
        const WHITESPACE_CHARS: [char; 4] = [' ', '\n', '\t', '\r'];
        return WHITESPACE_CHARS.contains(&ch);
    }

    #[must_use]
    pub fn is_left_paren(ch: char) -> bool {
        const LEFT_PAREN_CHARS: [char; 3] = ['(', '[', '{'];
        return LEFT_PAREN_CHARS.contains(&ch);
    }

    #[must_use]
    pub fn is_right_paren(ch: char) -> bool {
        const RIGHT_PAREN_CHARS: [char; 3] = [')', ']', '}'];
        return RIGHT_PAREN_CHARS.contains(&ch);
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = LexerItem<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let ch = self.chars.peek().copied();
            return match ch {
                Some((i, c)) if Lexer::is_left_paren(c) => self.consume_left_paren(i),
                Some((i, c)) if Lexer::is_right_paren(c) => self.consume_right_paren(i),
                Some((i, c)) if Lexer::is_quote(c) => self.consume_quote(i),
                Some((i, c)) if Lexer::is_string_quote(c) => self.consume_string(i),
                Some((i, c)) if Lexer::is_decimal_digit(c) => self.consume_num(i),
                Some((i, c)) if Lexer::is_hash_char(c) => self.consume_hash_symbol(i),
                Some((i, c)) if Lexer::is_symbol_char(c) => self.consume_symbol(i),
                Some((i, c)) if Lexer::is_whitespace(c) => {
                    self.consume_whitespace(i);
                    // This skips the whitespace instead of creating a token for it
                    continue;
                    // return Some(Ok((i, Tok::Whitespace, i + 1)));
                }
                None => None, // End of file
                Some((i, c)) => panic!("Tokenizer: invalid token {c} at {i}"),
            };
        }
    }
}

#[test]
fn lexer_works_properly() {
    let source = "(atom 10 \"string\" '(1 2 3) string-append #true)";

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

    assert_eq!(lex.next().unwrap().unwrap().1, Tok::HashSymbol("#true"));

    assert_eq!(lex.next().unwrap().unwrap().1, Tok::RightParen);

    assert_eq!(Lexer::new(source).count(), 13);
}
