use std::io::{BufReader, BufRead};
use std::fs::File;
use std::iter::Peekable;
use std::str::Chars;
use std::slice::Iter;

enum Token {
    Number(u64),
    Plus,
    Times,
    Expression(Vec<Token>),
}

fn tokenise(chars: &mut Peekable<Chars>, depth: usize) -> Token {
    let mut result = vec![];

    while let Some(c) = chars.next() {
        let token = match c {
            ' ' => continue,
            '+' => Token::Plus,
            '*' => Token::Times,
            '(' => tokenise(chars, depth + 1),
            ')' => {
                if depth == 0 {
                    panic!("Closing bracket without open");
                } else {
                    return Token::Expression(result)
                }
            },
            '0'..='9' => {
                let mut num_str = String::new();
                num_str.push(c);
                while let Some(next_c) = chars.peek() {
                    match next_c {
                        '0'..='9' => {
                            let c = chars.next().unwrap();
                            num_str.push(c);
                        }
                        _ => break,
                    }
                }
                let num = num_str.parse().expect("Could not parse number token");
                Token::Number(num)
            }
            _ => panic!("Unexpected character when tokenising: {}", c)
        };
        result.push(token);
    }

    Token::Expression(result)
}

enum Node {
    Number(u64),
    Plus(Vec<Node>),
    Times(Vec<Node>),
}

impl Node {
    fn evaluate(&self) -> u64 {
        match self {
            Node::Plus(nodes) => {
                nodes.iter().map(|n| n.evaluate()).sum()
            },
            Node::Times(nodes) => {
                nodes.iter().map(|n| n.evaluate()).product()
            },
            Node::Number(n) => *n
        }
    }
}

enum NodeMode {
    Plus,
    Times,
}

fn treeify_expr(token: &Token) -> Node {
    if let Token::Expression(tokens) = token {
        create_tree_ltr(&mut tokens.iter().peekable())
    } else {
        panic!("Can only treeify Expression tokens");
    }
}

fn create_tree_ltr(tokens: &mut Peekable<Iter<Token>>) -> Node {
    let first_token = tokens.next().expect("No more tokens for first token");
    let first_node = match first_token {
        Token::Number(n) => Node::Number(*n),
        Token::Expression(_) => treeify_expr(first_token),
        _ => panic!("Operator as value"),
    };

    let mut lhs_node = first_node;

    loop {
        if tokens.peek().is_none() {
            return lhs_node;
        }

        let op_token = tokens.next().expect("No more tokens for operator");
        let op_node_mode = match op_token {
            Token::Plus => NodeMode::Plus,
            Token::Times => NodeMode::Times,
            _ => panic!("Non-operator token in operator position"),
        };

        let next_token = tokens.next().expect("No more tokens for second operand");
        let next_node = match next_token {
            Token::Expression(children) => create_tree_ltr(&mut children.iter().peekable()),
            Token::Number(n) => Node::Number(*n),
            Token::Plus => panic!("Plus cannot be second operand"),
            Token::Times => panic!("Times cannot be second operand"),
        };

        lhs_node = match op_node_mode {
            NodeMode::Plus => Node::Plus(vec![lhs_node, next_node]),
            NodeMode::Times => Node::Times(vec![lhs_node, next_node]),
        };
    }
}

fn treeify_expr2(token: &Token) -> Node {
    if let Token::Expression(tokens) = token {
        treeify2(&mut tokens.iter())
    } else {
        panic!("Can only treeify Expression tokens");
    }
}

fn treeify2(token_iter: &mut Iter<Token>) -> Node {
    let mut factors = vec![];
    'outer: loop {
        let mut adds = vec![];

        while let Some(val_token) = token_iter.next() {
            let val = match val_token {
                Token::Number(n) => Node::Number(*n),
                Token::Expression(_) => treeify_expr2(val_token),
                _ => panic!("Operator as LHS"),
            };
            adds.push(val);

            if let Some(op_token) = token_iter.next() {
                match op_token {
                    Token::Plus => continue,
                    Token::Times => {
                        if adds.len() > 1 {
                            factors.push(Node::Plus(adds));
                        } else {
                            factors.push(adds.remove(0));
                        }
                        break;
                    },
                    _ => panic!("Expected operator"),
                }
            } else {
                if adds.len() > 1 {
                    factors.push(Node::Plus(adds));
                } else {
                    factors.push(adds.remove(0));
                }
                break 'outer;
            }
        }
    }
    Node::Times(factors)
}

fn main() {
    let reader = BufReader::new(File::open("input")
        .expect("Cannot open input"));

    let mut answers1 = vec![];
    let mut answers2 = vec![];
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let expr = tokenise(&mut line.chars().peekable(), 0);

        let root = treeify_expr(&expr);
        answers1.push(root.evaluate());

        let root = treeify_expr2(&expr);
        answers2.push(root.evaluate());
    }
    println!("{}", answers1.iter().sum::<u64>());
    println!("{}", answers2.iter().sum::<u64>());
}