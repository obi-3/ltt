mod calc;
mod lexer;
mod parse;
use calc::*;
use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let mut str: String = String::new();
        for item in args.iter().skip(1) {
            str.push_str(item);
        }

        match make_truth_table(str) {
            Ok(s) => println!("{s}"),
            Err(why) => println!("{why}"),
        }
    } else {
        println!("helpでヘルプを表示\nexitで終了");
        loop {
            print!(">> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");

            match input.as_str() {
                "\n" => continue,
                "exit\n" => break,
                "help\n" => {
                    println!("変数名 -> [A-Z] もしくは [A-Z]+[0-9] (ex: X  や Y1)");
                }
                _ => {}
            }
            match make_truth_table(input) {
                Ok(s) => println!("{s}"),
                Err(why) => println!("{why}"),
            }
        }
    }
}
