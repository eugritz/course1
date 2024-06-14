use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenType {
    Identifier,
    Literal,
    Punctuation,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    pub source: String,
    pub token_type: TokenType,
}

pub struct Tokenizer {
    source: String,
    cur: RefCell<usize>,
}

#[derive(Debug, Clone)]
pub struct TokenArr(pub(super) Vec<Token>);

impl Tokenizer {
    pub fn from(source: String) -> TokenArr {
        let tokenizer = Tokenizer {
            source,
            cur: RefCell::new(0),
        };

        tokenizer.tokenize()
    }

    pub fn tokenize(&self) -> TokenArr {
        let mut tokens = Vec::new();
        while let Some((slice, token_type)) = self.next_token() {
            tokens.push(Token {
                source: slice.to_string(),
                token_type,
            });
        }
        TokenArr(tokens)
    }

    pub fn is_quote(ch: char) -> bool {
        ch == '"' || ch == '\''
    }

    pub fn is_punctuation(ch: char) -> bool {
        ch != '_'
            && !ch.is_alphanumeric()
            && !ch.is_ascii_digit()
            && !Tokenizer::is_quote(ch)
    }

    pub fn next_token(&self) -> Option<(&str, TokenType)> {
        let mut start: Option<usize> = None;
        let mut end: Option<usize> = None;

        let mut quote = None;
        let mut escape = false;
        let mut punctuation = false;

        let mut chars = self.source.as_str()[*self.cur.borrow()..].chars();
        while let Some(ch) = chars.next() {
            let len = ch.len_utf8();

            let is_whitespace = ch.is_whitespace();
            let is_punctuation = !is_whitespace && Self::is_punctuation(ch);
            let is_quote = Self::is_quote(ch);

            if end.is_some() {
                if !is_whitespace {
                    break;
                }

                *self.cur.borrow_mut() += len;
                continue;
            }

            if start.is_none() {
                if is_punctuation {
                    start = Some(*self.cur.borrow());
                    punctuation = true;
                } else if is_quote {
                    start = Some(*self.cur.borrow());
                    quote = Some(ch);
                } else if !is_whitespace {
                    start = Some(*self.cur.borrow());
                }

                *self.cur.borrow_mut() += len;
                continue;
            }

            if !punctuation && !escape && ch == '\\' {
                escape = true;
                *self.cur.borrow_mut() += len;
                continue;
            }

            if punctuation {
                if !is_punctuation {
                    end = Some(*self.cur.borrow());
                    break;
                }
            } else if !escape {
                if (is_punctuation || is_quote) && quote.is_none() {
                    end = Some(*self.cur.borrow());
                    break;
                } else if is_whitespace && quote.is_none() {
                    end = Some(*self.cur.borrow());
                } else if is_quote && quote.unwrap() == ch {
                    end = Some(*self.cur.borrow() + len);
                }
            }

            *self.cur.borrow_mut() += len;
            escape = false;
        }

        if start.is_some() && end.is_some() {
            let slice = &self.source.as_str()[start.unwrap()..end.unwrap()];

            if punctuation {
                Some((slice, TokenType::Punctuation))
            } else if quote.is_some() {
                Some((slice, TokenType::Literal))
            } else {
                Some((slice, TokenType::Identifier))
            }
        } else if start.is_some() && end.is_none() && quote.is_none() {
            let slice = &self.source.as_str()[start.unwrap()..];

            if punctuation {
                Some((slice, TokenType::Punctuation))
            } else {
                Some((slice, TokenType::Identifier))
            }
        } else {
            None
        }
    }
}
