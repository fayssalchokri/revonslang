// Programmed By Fayssal Chokri <fayssalchokri@revons.co>
// REVONSRDT licence
// Bluespacengineers <no-replay@bluespacengineers.com>
// Revons Developers Team <no-replay-rdt@outlook.com>
// Copyright 2020 Revons Community , Bluespacengineers
extern crate revonslang;

use revonslang::{new_vm, ThreadExt};

fn main() {
    let vm = new_vm();

    let _ = vm.run_expr::<&str>("", r#" "test" "#);
    //~^ the trait bound `for<'value> &str: Getable<'_, 'value>` is not satisfied [E0277]
}
