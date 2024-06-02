use super::parser::{Node, Parser, Tokenizer};
use ::entity::{decks, entries};
use sea_orm::{
    entity::prelude::Expr,
    sea_query::{Alias, ConditionExpression},
    *,
};
use std::cell::RefCell;

pub struct EntryQueryBuilder<'a, C: ConnectionTrait> {
    db: &'a C,
    query: RefCell<Option<Select<entries::Entity>>>,
}

impl<'a, C: ConnectionTrait> EntryQueryBuilder<'a, C> {
    pub fn new(db: &'a C, query: Select<entries::Entity>) -> Self {
        EntryQueryBuilder {
            db,
            query: RefCell::new(Some(query)),
        }
    }

    pub async fn parse_with_fallback(
        self,
        source: String,
        fallback: String,
    ) -> Result<Vec<entries::Model>, String> {
        let cond = self
            .get_condition(source)
            .or_else(|_| self.get_condition(fallback))
            .map_err(|_| "parsing error".to_string())?;

        self.query.replace_with(|q| {
            let q = q.take()?;
            Some(
                q.join(JoinType::Join, entries::Relation::Decks.def())
                    .filter(cond),
            )
        });

        self.query
            .take()
            .ok_or("query take failed".to_string())?
            .all(self.db)
            .await
            .map_err(|err| err.to_string())
    }

    fn parse_node(&self, node: Box<Node>) -> Option<ConditionExpression> {
        match *node {
            Node::BinaryExpr { op, lhs, rhs } => match op {
                super::parser::Operator::And => self.parse_and(lhs, rhs),
                super::parser::Operator::Colon => self.parse_colon(lhs, rhs),
                super::parser::Operator::Or => self.parse_or(lhs, rhs),
            },
            Node::StringLit(string) => {
                Some(Expr::col(Alias::new("sort_field")).eq(string).into())
            }
        }
    }

    fn parse_and(
        &self,
        lhs: Box<Node>,
        rhs: Box<Node>,
    ) -> Option<ConditionExpression> {
        let lhs_parsed = self.parse_node(lhs);
        let rhs_parsed = self.parse_node(rhs);

        match (lhs_parsed, rhs_parsed) {
            (Some(lhs_cond), Some(rhs_cond)) => {
                Some(Condition::all().add(lhs_cond).add(rhs_cond).into())
            }
            (Some(lhs_cond), None) => Some(lhs_cond),
            (None, Some(rhs_cond)) => Some(rhs_cond),
            (None, None) => None,
        }
    }

    fn parse_colon(
        &self,
        lhs: Box<Node>,
        rhs: Box<Node>,
    ) -> Option<ConditionExpression> {
        let field = match *lhs {
            Node::StringLit(string) => match string.to_lowercase().as_str() {
                "колода" => Some(decks::Column::Name),
                _ => None,
            },
            _ => None,
        }?;
        let value = match *rhs {
            Node::StringLit(string) => Some(string),
            _ => None,
        }?;
        Some(field.eq(value).into())
    }

    fn parse_or(
        &self,
        lhs: Box<Node>,
        rhs: Box<Node>,
    ) -> Option<ConditionExpression> {
        let lhs_parsed = self.parse_node(lhs);
        let rhs_parsed = self.parse_node(rhs);

        match (lhs_parsed, rhs_parsed) {
            (Some(lhs_cond), Some(rhs_cond)) => {
                Some(Condition::any().add(lhs_cond).add(rhs_cond).into())
            }
            (Some(lhs_cond), None) => Some(lhs_cond),
            (None, Some(rhs_cond)) => Some(rhs_cond),
            (None, None) => None,
        }
    }

    fn get_condition(&self, source: String) -> Result<Condition, ()> {
        let tokens = Tokenizer::from(source);
        let nodes = Parser::from(tokens).map_err(|_| ())?;
        let expr = nodes.try_into_expr().ok_or(())?;
        let cond = self.parse_node(Box::new(expr)).ok_or(())?;
        Ok(Condition::all().add(cond))
    }
}
