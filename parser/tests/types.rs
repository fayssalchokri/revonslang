// Programmed By Fayssal Chokri <fayssalchokri@revons.co>
// REVONSRDT licence
// Bluespacengineers <no-replay@bluespacengineers.com>
// Revons Developers Team <no-replay-rdt@outlook.com>
// Copyright 2020 Revons Community , Bluespacengineers
extern crate revonslang_base as base;
extern crate revonslang_parser as parser;

use crate::base::{ast::Expr, mk_ast_arena, types::TypeContext};
use crate::support::{clear_span, parse, typ};

mod support;

#[test]
fn function_type() {
    let _ = env_logger::try_init();

    let input = "let _ : Int -> Float -> String = 1 in 1";
    let expr = parse(input).unwrap_or_else(|err| panic!("{}", err.1));
    mk_ast_arena!(arena);
    let mut arena = (*arena).borrow();
    match clear_span(expr).expr().value {
        Expr::LetBindings(ref bindings, _) => {
            assert_eq!(
                bindings[0].typ,
                Some(arena.function(
                    vec![typ(arena, "Int"), typ(arena, "Float")],
                    typ(arena, "String"),
                ),)
            );
        }
        _ => panic!("Expected let"),
    }
}
