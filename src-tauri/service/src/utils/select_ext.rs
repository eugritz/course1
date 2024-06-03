use sea_orm::{query::Select, EntityTrait};

pub trait Apply {
    fn apply<F>(self, f: F) -> Self
    where
        Self: Sized,
        F: FnOnce(Self) -> Self,
    {
        f(self)
    }
}

impl<E: EntityTrait> Apply for Select<E> {}
