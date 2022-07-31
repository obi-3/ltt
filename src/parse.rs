use core::panic;

use crate::lexer;
use lexer::*;

#[derive(Debug, Clone)]
pub struct Tree {
    pub token: Token,
    pub left: Option<Box<Tree>>,
    pub right: Option<Box<Tree>>,
}

// fn print_tree(root: Option<Box<Tree>>) {
//     print_tree_n(0, root);
// }
// fn print_tree_n(n: i32, root: Option<Box<Tree>>) {
//     match root {
//         None => return,
//         Some(root) => {
//             print_tree_n(n + 1, root.left);
//             for _i in 0..n {
//                 print!("    ");
//             }
//             println!("{:?}", root.token);
//             print_tree_n(n + 1, root.right);
//         }
//     }
// }

pub struct Parser {
    pub lexer: Lexer,
    // 現在解析中のトークン
    curr_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Parser {
        let curr_token = lexer.get_token();
        Parser { lexer, curr_token }
    }

    // pub fn test_parse(&mut self) {
    //     let root: Option<Box<Tree>>;
    //     root = self.eval_expr();

    //     if self.curr_token == Token::End {
    //         // println!("{:?}", root);
    //         print_tree(root);
    //     } else {
    //         panic!("Parse Error");
    //     }
    // }

    pub fn parse(&mut self) -> Option<Box<Tree>> {
        let root: Option<Box<Tree>>;
        root = self.eval_expr();

        if self.curr_token == Token::End {
            // println!("{:?}", root);
            return root;
        } else {
            panic!("Parse Error");
        }
    }

    fn eval_expr(&mut self) -> Option<Box<Tree>> {
        let mut root: Option<Box<Tree>>;
        root = self.eval_term();

        match self.curr_token {
            Token::Op(Operator::Or) | Token::Op(Operator::Nor) | Token::Op(Operator::Xor) => {
                let token: Token = self.curr_token.clone();
                self.curr_token = self.lexer.get_token();
                root = Some(Box::new(Tree {
                    token,
                    left: root,
                    right: self.eval_expr(),
                }));
            }
            _ => {}
        }

        return root;
    }

    fn eval_term(&mut self) -> Option<Box<Tree>> {
        let mut root: Option<Box<Tree>>;
        root = self.eval_factor();

        match self.curr_token {
            Token::Op(Operator::And) | Token::Op(Operator::Nand) | Token::Op(Operator::Is) => {
                let token: Token = self.curr_token.clone();
                self.curr_token = self.lexer.get_token();
                root = Some(Box::new(Tree {
                    token,
                    left: root,
                    right: self.eval_term(),
                }));
            }
            _ => {}
        }

        return root;
    }

    fn eval_factor(&mut self) -> Option<Box<Tree>> {
        let root: Option<Box<Tree>>;

        match self.curr_token {
            Token::Op(Operator::Not) => {
                let token: Token = self.curr_token.clone();
                self.curr_token = self.lexer.get_token();
                root = Some(Box::new(Tree {
                    token,
                    left: self.eval_primary(),
                    right: None,
                }));
            }
            _ => root = self.eval_primary(),
        }

        return root;
    }

    fn eval_primary(&mut self) -> Option<Box<Tree>> {
        let root: Option<Box<Tree>>;

        match self.curr_token.clone() {
            Token::True | Token::False => {
                let token = self.curr_token.clone();
                self.curr_token = self.lexer.get_token();
                root = Some(Box::new(Tree {
                    token,
                    left: None,
                    right: None,
                }));
            }
            Token::Var(_) => {
                let token: Token = self.curr_token.clone();
                self.curr_token = self.lexer.get_token();
                root = Some(Box::new(Tree {
                    token,
                    left: None,
                    right: None,
                }));
            }
            Token::Lpar => {
                self.curr_token = self.lexer.get_token();
                root = self.eval_expr();
                if self.curr_token != Token::Rpar {
                    panic!("Parence Error");
                }
                self.curr_token = self.lexer.get_token();
            }
            _ => panic!("Parse Error"),
        }

        return root;
    }
}

// pub fn parse_test() {
//     let str = "((X+Y))".to_string();
//     let fstr = format_string(str.clone());
//     let lexer = Lexer::new(fstr.clone());
//     let mut parser = Parser::new(lexer.clone());
//     println!("{:?}", lexer);
//     parser.test_parse();
// }
