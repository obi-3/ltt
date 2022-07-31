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
        do_calc(str);
    } else {
        println!("helpでヘルプを表示\nexitで終了");
        loop {
            print!(">> ");
            io::stdout().flush().unwrap();

            let mut code = String::new();
            io::stdin()
                .read_line(&mut code)
                .expect("failed to read line");

            match code.as_str() {
                "\n" => continue,
                "exit\n" => break,
                "help\n" => {
                    println!("変数名 -> [A-Z] もしくは [A-Z]+[0-9] (ex: X  や Y1)");
                }
                _ => {}
            }

            do_calc(code);
        }
    }
}
