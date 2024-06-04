use super::{
    parser::{Node, Parser, Tokenizer},
    select_ext::Apply,
};
use ::entity::{decks, entries, entry_field_values, entry_kind_default_field};
use sea_orm::{
    entity::prelude::{Date, DateTimeUtc, Expr},
    sea_query::{Alias, ConditionExpression, IntoCondition, LikeExpr},
    *,
};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

#[derive(FromQueryResult, Serialize, Deserialize)]
pub struct Entry {
    pub id: i32,
    pub entry_kind_id: i32,
    pub deck_id: i32,
    pub deck_name: String,
    pub color_tag: i32,
    pub progress: f64,
    pub created_at: Date,
    pub last_shown_at: Option<DateTimeUtc>,
    pub next_shown_at: Option<DateTimeUtc>,
    pub sort_field: String,
}

fn wrap_pattern(s: String) -> String {
    let mut escaped = "%".to_string();
    for ch in s.chars() {
        match ch {
            '%' => escaped.push_str("\\%"),
            '_' => escaped.push_str("\\_"),
            ch => escaped.push(ch),
        }
    }
    escaped.push('%');
    escaped
}

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
    ) -> Result<Vec<Entry>, DbErr> {
        let cond = self
            .get_condition(source)
            .or_else(|_| self.get_condition(fallback))
            .map_err(|_| DbErr::Custom("parsing error".to_string()))?;

        self.query.replace_with(|q| {
            let q = q.take()?;
            Some(
                q.join(JoinType::Join, entries::Relation::Decks.def())
                    .column_as(decks::Column::Name, "deck_name")
                    .apply(Self::sort_field)
                    .filter(cond),
            )
        });

        self.query
            .take()
            .ok_or(DbErr::Custom("query take failed".to_string()))?
            .into_model::<Entry>()
            .all(self.db)
            .await
    }

    fn sort_field<E: EntityTrait>(q: Select<E>) -> Select<E> {
        q.join_rev(
            JoinType::Join,
            entry_kind_default_field::Entity::belongs_to(entries::Entity)
                .from(entry_kind_default_field::Column::EntryKindId)
                .to(entries::Column::EntryKindId)
                .into(),
        )
        .join(
            JoinType::Join,
            entries::Relation::EntryFieldValues.def().on_condition(
                |_left, right| {
                    Expr::col((
                        entry_kind_default_field::Entity,
                        entry_kind_default_field::Column::EntryKindFieldId,
                    ))
                    .equals((right, entry_field_values::Column::EntryFieldId))
                    .into_condition()
                },
            ),
        )
        .column_as(entry_field_values::Column::Value, "sort_field")
    }

    fn parse_node(&self, node: Box<Node>) -> Option<ConditionExpression> {
        match *node {
            Node::BinaryExpr { op, lhs, rhs } => match op {
                super::parser::Operator::And => self.parse_and(lhs, rhs),
                super::parser::Operator::Colon => self.parse_colon(lhs, rhs),
                super::parser::Operator::Or => self.parse_or(lhs, rhs),
            },
            Node::StringLit(string) => Some(
                Expr::col(Alias::new("sort_field"))
                    .like(LikeExpr::new(wrap_pattern(string)).escape('\\'))
                    .into(),
            ),
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
