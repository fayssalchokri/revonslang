// Programmed By Fayssal Chokri <fayssalchokri@revons.co>
// REVONSRDT licence
// Bluespacengineers <no-replay@bluespacengineers.com>
// Revons Developers Team <no-replay-rdt@outlook.com>
// Copyright 2020 Revons Community , Bluespacengineers
pub use crate::base::types::{ArcType as RcType, Flags, TypeExt, TypePtr};

#[cfg(test)]
mod tests {
    use super::*;

    use crate::base::{
        symbol::Symbol,
        types::{Generic, Type},
    };

    #[test]
    fn flags() {
        let gen = Type::<_, RcType>::generic(Generic::new(Symbol::from("a"), Default::default()));
        assert_eq!(gen.flags(), Flags::HAS_GENERICS);
        assert_eq!(
            Type::function(vec![gen.clone()], gen.clone()).flags(),
            Flags::HAS_GENERICS
        );
    }
}
