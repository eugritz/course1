use ::entity::entries;
use sea_orm::*;

struct Parser {
    source: String,
    cur: usize,
}

impl Parser {
    fn parse(source: String) {
        let mut parser = Parser {
            source,
            cur: 0
        };

        parser.build();
    }

    fn build(&mut self) {
    }

    fn is_quote(ch: char) -> bool {
        ch == '"' || ch == '\''
    }

    fn unquote(s: String) -> String {
        s.trim_matches(|c| c == '\'' || c == '"').to_string()
    }

    fn unescape(s: String) -> String {
        let mut unescaped = "".to_string();

        let mut escape = false;

        for ch in s.chars() {
            if !escape {
                if ch == '\\' {
                    escape = true;
                } else {
                    unescaped.push(ch);
                }

                continue;
            }

            match ch {
                '\t' => unescaped.push('\t'),
                '\n' => unescaped.push('\n'),
                ' ' => unescaped.push(' '),
                '"' => unescaped.push('"'),
                '\'' => unescaped.push('\''),
                '\\' => unescaped.push('\\'),
                ch => {
                    unescaped.push('\\');
                    unescaped.push(ch);
                },
            }

            escape = false;
        }

        unescaped
    }

    fn next_raw_token(&mut self) -> Option<&str> {
        let mut start = None;
        let mut end = None;

        let mut quote = None;
        let mut escape = false;

        let mut chars = self.source.as_str()[self.cur..].chars();
        while let Some(ch) = chars.next() {
            let len = ch.len_utf8();

            if end.is_some() {
                if !ch.is_whitespace() {
                    break;
                }

                self.cur += len;
                continue;
            }

            if start.is_none() {
                if Parser::is_quote(ch) {
                    start = Some(self.cur);
                    quote = Some(ch);
                } else if !ch.is_whitespace() {
                    start = Some(self.cur);
                }

                self.cur += len;
                continue;
            }

            if !escape && ch == '\\' {
                escape = true;
                self.cur += len;
                continue;
            }

            let is_whitespace = ch.is_whitespace() && quote.is_none();
            let is_quote = Parser::is_quote(ch);

            if !escape {
                if is_quote && quote.is_none() {
                    end = Some(self.cur);
                    break;
                } else if is_quote && quote.unwrap() == ch {
                    end = Some(self.cur + len);
                } else if is_whitespace {
                    end = Some(self.cur);
                }
            }

            self.cur += len;
            escape = false;
        }

        if start.is_some() && end.is_some() {
            Some(&self.source.as_str()[start.unwrap()..end.unwrap()])
        } else if start.is_some() && end.is_none() && quote.is_none() {
            Some(&self.source.as_str()[start.unwrap()..])
        } else {
            None
        }
    }
}

pub struct EntryQueryBuilder {
    query: Select<entries::Entity>,
}

impl EntryQueryBuilder {
    pub fn new(query: Select<entries::Entity>) -> Self {
        EntryQueryBuilder {
            query,
        }
    }

    pub fn parse(self, source: String) {
        Parser::parse(source);
    }
}
