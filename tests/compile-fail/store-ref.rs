// Programmed By Fayssal Chokri <fayssalchokri@revons.co>
// REVONSRDT licence
// Bluespacengineers <no-replay@bluespacengineers.com>
// Revons Developers Team <no-replay-rdt@outlook.com>
// Copyright 2020 Revons Community , Bluespacengineers
#[macro_use]
extern crate revonslang_vm;
extern crate revonslang;
extern craterevonslang_codegen;

use std::{fmt, sync::Mutex};

use revonslang::{
    import::add_extern_module,
    new_vm,
    vm::{
        api::{primitive_f, Userdata, VmType},
        gc::Trace,
        thread::{Status, Thread},
        ExternModule,
    },
};

#[derive(revonslang_codegen::Trace)]
#revonslang_trace(skip)]
struct Test<'vm>(Mutex<&'vm str>);

impl Userdata for Test<'static> {}

impl<'vm> fmt::Debug for Test<'vm> {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl<'vm> VmType for Test<'vm> {
    type Type = Test<'static>;
}

fn f<'vm>(test: &'vm Test<'vm>, s: &'vm str) {
    *test.0.lock().unwrap() = s;
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn main() {
    let vm = new_vm();
    add_extern_module(&vm, "test", |vm| {
        ExternModule::new(vm, primitive!(2, f))
        //~^ `thread` has lifetime `'thread` but it needs to satisfy a `'static` lifetime requirement
    });
}
