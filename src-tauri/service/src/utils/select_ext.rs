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

pub trait ApplyIf {
    fn apply_if_<F>(self, val: bool, f: F) -> Self
    where
        Self: Sized,
        F: FnOnce(Self) -> Self,
    {
        if val {
            f(self)
        } else {
            self
        }
    }
}

impl<E: EntityTrait> ApplyIf for Select<E> {}
