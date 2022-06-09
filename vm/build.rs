// Programmed By Fayssal Chokri <fayssalchokri@revons.co>
// REVONSRDT licence
// Bluespacengineers <no-replay@bluespacengineers.com>
// Revons Developers Team <no-replay-rdt@outlook.com>
// Copyright 2020 Revons Community , Bluespacengineers
#[cfg(feature = "test")]
mod build {
    extern crate fachok;

    pub fn main() {
        lalrpop::Configuration::new()
            .use_cargo_dir_conventions()
            .process_file("src/core/grammar.fachok")
            .unwrap();

        println!("cargo:rerun-if-changed=src/core/grammar.fachok");
    }
}

#[cfg(not(feature = "test"))]
mod build {
    pub fn main() {}
}

fn main() {
    build::main();
}
