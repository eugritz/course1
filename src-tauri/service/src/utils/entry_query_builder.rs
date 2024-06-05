use super::{
    parser::{Node, Parser, Tokenizer},
    select_ext::{Apply, ApplyIf},
};
use ::entity::{
    cards, decks, entries, entry_field_values, entry_kind_default_field,
    entry_kinds, entry_tags, tags,
};
use futures::TryStreamExt;
use sea_orm::{
    entity::prelude::{Date, DateTimeUtc, Expr},
    sea_query::{Alias, ConditionExpression, IntoCondition, LikeExpr},
    *,
};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub id: i32,
    pub sort_field: String,
    pub entry_kind_id: i32,
    pub entry_kind_name: String,
    pub deck_id: i32,
    pub deck_name: String,
    pub card_count: i32,
    pub tags: Vec<String>,
    pub color_tag: i32,
    pub progress: f64,
    pub created_at: Date,
    pub last_shown_at: Option<DateTimeUtc>,
    pub next_shown_at: Option<DateTimeUtc>,
}

#[derive(FromQueryResult, Serialize, Deserialize)]
pub struct Card {
    pub id: i32,
    pub sort_field: String,
    pub entry_kind_id: i32,
    pub deck_id: i32,
    pub deck_name: String,
    pub card_id: i32,
    pub card_name: String,
    pub color_tag: i32,
    pub progress: f64,
    pub created_at: Date,
    pub last_shown_at: Option<DateTimeUtc>,
    pub next_shown_at: Option<DateTimeUtc>,
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

pub struct EntryQueryBuilder<'a, C: ConnectionTrait + StreamTrait> {
    db: &'a C,
    query: RefCell<Option<Select<entries::Entity>>>,
    is_entries: RefCell<bool>,
    filter_entry_kind: RefCell<bool>,
    filter_tags: RefCell<bool>,
}

impl<'a, C: ConnectionTrait + StreamTrait> EntryQueryBuilder<'a, C> {
    pub fn new(db: &'a C, query: Select<entries::Entity>) -> Self {
        EntryQueryBuilder {
            db,
            query: RefCell::new(Some(query)),
            is_entries: RefCell::new(false),
            filter_entry_kind: RefCell::new(false),
            filter_tags: RefCell::new(false),
        }
    }

    pub async fn try_parse_entries(
        self,
        source: String,
        fallback: String,
    ) -> Result<Vec<Entry>, DbErr> {
        #[derive(FromQueryResult, Serialize, Deserialize)]
        pub struct FlatEntry {
            pub id: i32,
            pub sort_field: String,
            pub entry_kind_id: i32,
            pub entry_kind_name: String,
            pub deck_id: i32,
            pub deck_name: String,
            pub card_count: i32,
            pub tag_name: Option<String>,
            pub color_tag: i32,
            pub progress: f64,
            pub created_at: Date,
            pub last_shown_at: Option<DateTimeUtc>,
            pub next_shown_at: Option<DateTimeUtc>,
        }

        self.is_entries.replace(true);

        let cond = self
            .get_condition(source)
            .or_else(|_| self.get_condition(fallback))
            .map_err(|_| DbErr::Custom("parsing error".to_string()))?;

        self.query.replace_with(|q| {
            let q = q.take()?;
            Some(
                self.entry_kind(q)
                    .apply(Self::sort_field)
                    .apply(Self::cards)
                    .apply_if_(!*self.filter_tags.borrow(), |q| {
                        q.group_by(entries::Column::Id)
                    })
                    .apply_if_(!*self.filter_tags.borrow(), |q| {
                        q.join(
                            JoinType::FullOuterJoin,
                            entries::Relation::EntryTags.def(),
                        )
                        .join(
                            JoinType::FullOuterJoin,
                            entry_tags::Relation::Tags.def(),
                        )
                    })
                    .join(JoinType::Join, entries::Relation::Decks.def())
                    .column_as(decks::Column::Name, "deck_name")
                    .column_as(cards::Column::Id.count(), "card_count")
                    .column_as(tags::Column::Name, "tag_name")
                    .filter(cond),
            )
        });

        self.filter_entry_kind.replace(true);

        let mut stream = self
            .query
            .take()
            .ok_or(DbErr::Custom("query take failed".to_string()))?
            .into_model::<FlatEntry>()
            .stream(self.db)
            .await?;

        let mut entries = Vec::<Entry>::new();
        while let Some(entry) = stream.try_next().await? {
            if entry.tag_name.is_some() {
                if let Some(last) = entries.last_mut() {
                    if last.id == entry.id {
                        last.tags.push(entry.tag_name.unwrap());
                        continue;
                    }
                }
            }

            entries.push(Entry {
                id: entry.id,
                sort_field: entry.sort_field,
                entry_kind_id: entry.entry_kind_id,
                entry_kind_name: entry.entry_kind_name,
                deck_id: entry.deck_id,
                deck_name: entry.deck_name,
                card_count: entry.card_count,
                tags: entry.tag_name.map(|x| vec![x]).unwrap_or(vec![]),
                color_tag: entry.color_tag,
                progress: entry.progress,
                created_at: entry.created_at,
                last_shown_at: entry.last_shown_at,
                next_shown_at: entry.next_shown_at,
            });
        }

        Ok(entries)
    }

    pub async fn try_parse_cards(
        self,
        source: String,
        fallback: String,
    ) -> Result<Vec<Card>, DbErr> {
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
                    .apply(Self::cards)
                    .filter(cond),
            )
        });

        self.query
            .take()
            .ok_or(DbErr::Custom("query take failed".to_string()))?
            .into_model::<Card>()
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

    fn cards<E: EntityTrait>(q: Select<E>) -> Select<E> {
        q.join_rev(
            JoinType::Join,
            cards::Entity::belongs_to(entries::Entity)
                .from(cards::Column::EntryKindId)
                .to(entries::Column::EntryKindId)
                .into(),
        )
        .column_as(cards::Column::Id, "card_id")
        .column_as(cards::Column::Name, "card_name")
    }

    fn entry_kind<E: EntityTrait>(&self, q: Select<E>) -> Select<E> {
        if *self.filter_entry_kind.borrow() {
            return q;
        }
        self.filter_entry_kind.replace(true);

        q.join(JoinType::Join, entries::Relation::EntryKinds.def())
            .column_as(entry_kinds::Column::Name, "entry_kind_name")
    }

    fn tags<E: EntityTrait>(&self, q: Select<E>) -> Select<E> {
        if *self.filter_tags.borrow() {
            return q;
        }
        self.filter_tags.replace(true);

        let is_entries = *self.is_entries.borrow();
        q.join(JoinType::FullOuterJoin, entries::Relation::EntryTags.def())
            .join(JoinType::FullOuterJoin, entry_tags::Relation::Tags.def())
            .apply_if_(is_entries, |q| q.group_by(entries::Column::Id))
            .apply_if_(!is_entries, |q| {
                q.group_by(entries::Column::Id).group_by(cards::Column::Id)
            })
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
        let value = match *rhs {
            Node::StringLit(string) => Some(string),
            _ => None,
        }?;
        match *lhs {
            Node::StringLit(string) => match string.to_lowercase().as_str() {
                "колода" => Some(decks::Column::Name.eq(value).into()),
                "вид" => {
                    self.query.replace_with(|q| {
                        let q = q.take()?;
                        Some(self.entry_kind(q))
                    });

                    Some(entry_kinds::Column::Name.eq(value).into())
                }
                "метка" => {
                    self.query.replace_with(|q| {
                        let q = q.take()?;
                        Some(self.tags(q))
                    });

                    if value == "" {
                        Some(entry_tags::Column::TagId.is_null().into())
                    } else {
                        Some(tags::Column::Name.eq(value).into())
                    }
                }
                _ => None,
            },
            _ => None,
        }
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
