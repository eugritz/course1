use super::{Token, TokenArr, TokenType};
use linked_hash_set::LinkedHashSet;
use std::{cell::RefCell, collections::BinaryHeap, rc::Rc};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum Operator {
    And,
    Colon,
    ColonNot,
    Or,
}

impl Operator {
    pub fn parse_token(token: &Token) -> Option<Operator> {
        match token.token_type {
            TokenType::Identifier => match token.source.to_lowercase().as_str()
            {
                "и" => Some(Operator::And),
                "или" => Some(Operator::Or),
                _ => None,
            },
            TokenType::Punctuation => match token.source.as_str() {
                "&&" => Some(Operator::And),
                ":" => Some(Operator::Colon),
                ":-" => Some(Operator::ColonNot),
                "||" => Some(Operator::Or),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn precedence(&self) -> usize {
        match *self {
            Self::Colon => 1,
            Self::ColonNot => 1,
            Self::And => 2,
            Self::Or => 3,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Node {
    StringLit(String),
    BinaryExpr {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum TokenError {
    UnknownOperator,
    MissingOperand,
    UnknownOperandType,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenParseState {
    Node(Node),
    Operator(Operator),
    Token(Token),
    Error(TokenError),
}

#[derive(Debug)]
pub struct TokenNode {
    id: usize,
    either: RefCell<TokenParseState>,
    priority: RefCell<usize>,
}

impl PartialEq for TokenNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for TokenNode {}

impl std::hash::Hash for TokenNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialOrd for TokenNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.priority
            .partial_cmp(&other.priority)
            .map(|c| c.reverse())
    }
}

impl Ord for TokenNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority).reverse()
    }
}

#[derive(Debug, Clone)]
pub struct NodeArr(pub(super) Vec<Node>);

impl NodeArr {
    pub fn try_into_expr(mut self) -> Option<Node> {
        if self.0.len() == 0 {
            return None;
        }

        let f = self.0.swap_remove(0);
        match f {
            Node::StringLit(string) => Some(Node::StringLit(string)),
            Node::BinaryExpr { op, lhs, rhs } => {
                Some(Node::BinaryExpr { op, lhs, rhs })
            }
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    TokenError(TokenError),
    UnparsedOperator,
    UnparsedToken,
}

pub struct Parser {
    tokens: RefCell<LinkedHashSet<Rc<TokenNode>>>,
}

impl Parser {
    pub fn from(tokens: TokenArr) -> Result<NodeArr, ParseError> {
        let parser = Parser {
            tokens: RefCell::new(LinkedHashSet::from_iter(
                tokens.0.into_iter().enumerate().map(|(i, val)| {
                    Rc::new(TokenNode {
                        id: i,
                        either: RefCell::new(TokenParseState::Token(val)),
                        priority: RefCell::new(0),
                    })
                }),
            )),
        };

        parser.parse();

        let mut nodes = Vec::new();
        for token in parser.tokens.into_inner() {
            let node = match Rc::into_inner(token).unwrap().either.into_inner()
            {
                TokenParseState::Node(node) => node,
                TokenParseState::Operator(_) => {
                    return Err(ParseError::UnparsedOperator)
                }
                TokenParseState::Token(_) => {
                    return Err(ParseError::UnparsedToken)
                }
                TokenParseState::Error(error) => {
                    return Err(ParseError::TokenError(error))
                }
            };

            nodes.push(node);
        }

        Ok(NodeArr(nodes))
    }

    pub fn parse(&self) {
        let mut heap = BinaryHeap::<Rc<TokenNode>>::new();

        for token in self.tokens.borrow().iter() {
            let parsed = self.parse_token(token);

            match &parsed {
                TokenParseState::Operator(operator) => {
                    token.priority.replace(operator.precedence());
                }
                _ => {}
            }

            if *token.priority.borrow() > 0 {
                heap.push(token.clone());
            }

            token.either.replace(parsed);
        }

        while let Some(token_node) = heap.pop() {
            let parsed = self.parse_expr(token_node.clone());
            token_node.either.replace(parsed);
        }
    }

    pub fn parse_token(&self, token_node: &TokenNode) -> TokenParseState {
        match &*token_node.either.borrow() {
            TokenParseState::Token(token) => match token.token_type {
                TokenType::Identifier => {
                    if let Some(operator) = Operator::parse_token(&token) {
                        if let Some(prev) =
                            self.tokens.borrow().get_prev(token_node)
                        {
                            match &*prev.either.borrow() {
                                TokenParseState::Operator(_) => {
                                    return TokenParseState::Node(
                                        Node::StringLit(token.source.clone()),
                                    );
                                }
                                TokenParseState::Token(prev_token) => {
                                    if prev_token.token_type
                                        == TokenType::Punctuation
                                    {
                                        return TokenParseState::Node(
                                            Node::StringLit(
                                                token.source.clone(),
                                            ),
                                        );
                                    }
                                }
                                _ => {}
                            }
                        }

                        if let Some(next) =
                            self.tokens.borrow().get_next(token_node)
                        {
                            match &*next.either.borrow() {
                                TokenParseState::Operator(_) => {
                                    return TokenParseState::Node(
                                        Node::StringLit(token.source.clone()),
                                    );
                                }
                                TokenParseState::Token(next_token) => {
                                    if next_token.token_type
                                        == TokenType::Punctuation
                                    {
                                        return TokenParseState::Node(
                                            Node::StringLit(
                                                token.source.clone(),
                                            ),
                                        );
                                    }
                                }
                                _ => {}
                            }
                        }

                        TokenParseState::Operator(operator)
                    } else {
                        TokenParseState::Node(Node::StringLit(
                            Parser::unwrap_quotes(&Parser::unescape(
                                token.source.clone(),
                            ))
                            .to_string(),
                        ))
                    }
                }
                TokenType::Punctuation => {
                    if let Some(operator) = Operator::parse_token(&token) {
                        TokenParseState::Operator(operator)
                    } else {
                        TokenParseState::Error(TokenError::UnknownOperator)
                    }
                }
                TokenType::Literal => TokenParseState::Node(Node::StringLit(
                    Parser::unwrap_quotes(&Parser::unescape(
                        token.source.clone(),
                    ))
                    .to_string(),
                )),
            },
            _ => unreachable!(),
        }
    }

    pub fn parse_expr(&self, token_node: Rc<TokenNode>) -> TokenParseState {
        match &*token_node.either.borrow() {
            TokenParseState::Operator(operator) => match operator {
                Operator::And
                | Operator::Colon
                | Operator::ColonNot
                | Operator::Or => {
                    let lhs = self.tokens.borrow_mut().take_prev(&token_node);
                    if lhs.is_none() {
                        return TokenParseState::Error(
                            TokenError::MissingOperand,
                        );
                    }

                    let rhs = self.tokens.borrow_mut().take_next(&token_node);
                    if rhs.is_none() {
                        return TokenParseState::Error(
                            TokenError::MissingOperand,
                        );
                    }

                    let (lhs, rhs) = (lhs.unwrap(), rhs.unwrap());

                    match (
                        &*lhs.clone().either.borrow(),
                        &*rhs.clone().either.borrow(),
                    ) {
                        (
                            TokenParseState::Node(lhs),
                            TokenParseState::Node(rhs),
                        ) => TokenParseState::Node(Node::BinaryExpr {
                            op: operator.clone(),
                            lhs: Box::new(lhs.clone()),
                            rhs: Box::new(rhs.clone()),
                        }),
                        (_, _) => TokenParseState::Error(
                            TokenError::UnknownOperandType,
                        ),
                    }
                }
            },
            _ => unreachable!(),
        }
    }

    pub fn unescape(s: String) -> String {
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
                }
            }

            escape = false;
        }

        unescaped
    }

    pub fn unwrap_quotes(s: &str) -> &str {
        let s = s.strip_prefix(|ch| ch == '"' || ch == '\'').unwrap_or(s);
        let s = s.strip_suffix(|ch| ch == '"' || ch == '\'').unwrap_or(s);
        s
    }
}
