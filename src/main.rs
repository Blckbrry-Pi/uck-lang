#[cfg(debug_assertions)]
use uckc::frontend::parser::parse_str;

#[cfg(debug_assertions)]
use std::io::{self, Read};

fn main() {
    #[cfg(debug_assertions)]
    {
        let mut string = String::new();

        io::stdin().read_to_string(&mut string).unwrap();

        println!("{:#?}", parse_str(&string));
    }
    #[cfg(not(debug_assertions))]
    {
        println!("Compiler for uck-lang coming soon!");
    }
}
