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
        for i in 1..args.len() {
            str.push_str(&args[i]);
        }
        do_calc(str);
    } else {
        println!("helpでヘルプを表示\nexitで終了");
        loop {
            print!(">> ");
            io::stdout().flush().unwrap();

            let mut code = String::new();
            io::stdin()
                .read_line(&mut code)
                .ok()
                .expect("failed to read line");

            if code == "\n" {
                continue;
            }

            if code == "help\n" {
                println!("変数名 -> [A-Z] もしくは [A-Z]+[0-9] (ex: X  や Y1)");
            }
            if code == "exit\n" {
                break;
            }

            do_calc(code);
        }
    }
}
