#[cfg(debug_assertions)]
use uckc::frontend::parser::lexer::{get_custom_lexer_from_string};

#[cfg(debug_assertions)]
use std::io::{self, Read};

fn main() {
    #[cfg(debug_assertions)]
    {
        let mut string = String::new();
    
        io::stdin().read_to_string(&mut string).unwrap();
    
    
        for token in get_custom_lexer_from_string(&string) {
            println!("{:?}", token);
        }
    }

    println!("Compiler for uck-lang coming soon!");
}
