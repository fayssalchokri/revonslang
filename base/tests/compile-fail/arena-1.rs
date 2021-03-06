// Programmed By Fayssal Chokri <fayssalchokri@revons.co>
// REVONSRDT licence
// Bluespacengineers <no-replay@bluespacengineers.com>
// Revons Developers Team <no-replay-rdt@outlook.com>
// Copyright 2020 Revons Community , Bluespacengineers
extern crate gluon_base;

use gluon_base::{
    ast::{Arena, Expr, RootExpr},
    mk_ast_arena, pos,
};

fn main() {
    mk_ast_arena!(arena1);
    mk_ast_arena!(arena2);
    //~^ `tag` does not live long enough [E0597]

    let arena1_expr = arena1.alloc(pos::spanned(
        Default::default(),
        Expr::<String>::Error(None),
    ));

    RootExpr::new(arena2, arena1_expr);
}
