#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub id: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    And,
    Nand,
    Or,
    Nor,
    Xor,
    Not,
    Is,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Var(Variable),
    Op(Operator),
    True,
    False,
    Rpar,
    Lpar,
    End,
    Error,
}

type Charkind = u8;
fn match_charkind(c: char) -> Charkind {
    if c.is_uppercase() || c.is_numeric() {
        return 0;
    };
    if c.is_lowercase() {
        return 1;
    };
    match c {
        //一文字で有意なもの
        '(' | ')' | '~' | '!' => return 2,
        _ => return 3,
    }
}

pub fn format_string(str: String) -> Vec<String> {
    let s: String = str.split_ascii_whitespace().collect();
    //println!("s is {:?}", s);
    let cs: Vec<char> = s.chars().collect();

    let mut index: usize = 0;
    let mut num: usize = 0;
    let mut strs: Vec<String> = Vec::new();

    let mut prev = match_charkind(cs[index]);
    strs.push(String::new());
    strs[num].push(cs[index]);
    index += 1;

    while index < cs.len() {
        let c = cs[index];
        if prev != 2 && prev == match_charkind(c) {
            strs[num].push(c);
        } else {
            num += 1;
            strs.push(String::new());
            strs[num].push(c);
        }
        index += 1;
        prev = match_charkind(c);
    }
    //println!("strs is {:?}", strs);
    return strs;
}
#[derive(Debug, Clone)]
pub struct Lexer {
    pub strs: Vec<String>,
    pub position: usize,
    pub vars: Vec<String>,
    pub vnum: i32,
}

impl Lexer {
    pub fn new(strs: Vec<String>) -> Lexer {
        Lexer {
            strs,
            position: 0,
            vars: Vec::new(),
            vnum: 0,
        }
    }
    pub fn get_token(&mut self) -> Token {
        if self.position == self.strs.len() {
            return Token::End;
        }
        let str = self.strs[self.position].clone();
        let mut token = Token::Error;
        match str.as_str() {
            "(" => token = Token::Lpar,
            ")" => token = Token::Rpar,
            "TRUE" | "1" => token = Token::True,
            "FALSE" | "0" => token = Token::False,
            "and" | "*" => token = Token::Op(Operator::And),
            "nand" => token = Token::Op(Operator::Nand),
            "or" | "+" => token = Token::Op(Operator::Or),
            "nor" => token = Token::Op(Operator::Nor),
            "xor" => token = Token::Op(Operator::Xor),
            "is" | "->" => token = Token::Op(Operator::Is),
            "~" | "!" => token = Token::Op(Operator::Not),
            _ => {
                let mut same_flag = 0;
                if let Some(same_index) = self.vars.iter().position(|x| *x == str) {
                    token = Token::Var(Variable {
                        name: self.vars[same_index].clone(),
                        id: same_index as i32,
                    });
                    same_flag = 1;
                }
                if same_flag == 0 {
                    let var = Variable {
                        name: str,
                        id: self.vnum,
                    };
                    token = Token::Var(var.clone());
                    self.vars.push(var.name);
                    self.vnum += 1;
                }
            }
        }
        //println!("{:?}", token);
        match token {
            Token::Error => panic!("Lexer Error!"),
            _ => {}
        };
        self.position += 1;
        return token;
    }
}

// pub fn test_lexer() {
//     let str = "!(X and !Y1) * !(Y2 nor W)".to_string();
//     println!("str is {:?}", str);
//     let fstr = format_string(str);
//     let mut lexer = Lexer::new(fstr.clone());
//     for _i in 0..fstr.len() {
//         println!("token -> {:?}", lexer.get_token());
//     }
//     println!("lexer is {:?}", lexer);
// }
