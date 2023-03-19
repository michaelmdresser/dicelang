use rand::Rng;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        assert_eq!(
            lex("-d40+2d4+1-3-1d6+3d1").unwrap(),
            vec![
                minus(),
                d(),
                number(String::from("40")),
                plus(),
                number(String::from("2")),
                d(),
                number(String::from("4")),
                plus(),
                number(String::from("1")),
                minus(),
                number(String::from("3")),
                minus(),
                number(String::from("1")),
                d(),
                number(String::from("6")),
                plus(),
                number(String::from("3")),
                d(),
                number(String::from("1")),
                eof(),
            ]
        )
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("-d40+2d4+1-3-1d6+3d1").unwrap(),
            Expr(vec![
                Node::Sub(RootNode::Roll(1, 40)),
                Node::Add(RootNode::Roll(2, 4)),
                Node::Add(RootNode::Constant(1)),
                Node::Sub(RootNode::Constant(3)),
                Node::Sub(RootNode::Roll(1, 6)),
                Node::Add(RootNode::Roll(3, 1)),
            ])
        )
    }

    #[test]
    fn test_eval() {
        let expr = parse("-d40+2d4+1-3-1d6+3d1").unwrap();
        let (result, _) = expr.eval();
        println!("{result}");

        let expr = parse("2d40+3").unwrap();
        let (result, _) = expr.eval();
        println!("{result}");
    }
}

#[derive(Debug, PartialEq, Clone)]
enum TokenKind {
    Plus,
    Minus,
    D,
    Number,
    EOF,
}

#[derive(Debug, PartialEq, Clone)]
struct Token {
    kind: TokenKind,
    s: String,
}

fn plus() -> Token {
    Token {
        kind: TokenKind::Plus,
        s: String::new(),
    }
}
fn minus() -> Token {
    Token {
        kind: TokenKind::Minus,
        s: String::new(),
    }
}
fn d() -> Token {
    Token {
        kind: TokenKind::D,
        s: String::new(),
    }
}
fn number(s: String) -> Token {
    Token {
        kind: TokenKind::Number,
        s: s,
    }
}
fn eof() -> Token {
    Token {
        kind: TokenKind::EOF,
        s: String::new(),
    }
}

pub type LexErr = String;
struct Scanner {
    // TODO: Rewrite scanning as just using .chars() as an iterator?
    source: String,
    tokens: Vec<Token>,
    errors: Vec<LexErr>,

    lexeme_start: usize,
    next: usize,
}

impl Scanner {
    fn scan_tokens(&mut self) {
        while !self.at_end() {
            self.lexeme_start = self.next;
            self.scan_token();
        }
        self.tokens.push(eof());
    }
    fn at_end(&self) -> bool {
        let len = self.source.chars().fold(0, |total, _| total + 1);
        return self.next >= len;
    }
    fn advance(&mut self) -> char {
        // The assumption is that advance will never be called if at end, so
        // unwrap() is okay
        let c = self.source.chars().nth(self.next).unwrap();
        self.next += 1;
        return c;
    }
    // fn match_char(&mut self, expected: char) -> bool {
    //     if self.at_end() {
    //         return false;
    //     }
    //     // We've checked at end, so unwrap should be safe
    //     if self.source.chars().nth(self.next).unwrap() != expected {
    //         return false;
    //     }
    //     self.next += 1;
    //     return true;
    // }
    fn add_token(&mut self, kind: TokenKind) {
        let source = self.source.clone();
        let mut token_str = String::from("");
        source
            .chars()
            .skip(self.lexeme_start)
            .take(self.next - self.lexeme_start)
            .for_each(|c| token_str.push(c));
        match kind {
            TokenKind::Plus => self.tokens.push(plus()),
            TokenKind::Minus => self.tokens.push(minus()),
            TokenKind::D => self.tokens.push(d()),
            TokenKind::Number => self.tokens.push(number(token_str)),
            TokenKind::EOF => self.tokens.push(eof()),
        }
    }
    fn peek(&self) -> char {
        if self.at_end() {
            return '_';
        }
        // We've checked at end, so unwrap should be safe
        return self.source.chars().nth(self.next).unwrap();
    }
    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '+' => self.add_token(TokenKind::Plus),
            '-' => self.add_token(TokenKind::Minus),
            'd' => self.add_token(TokenKind::D),
            _ => self.number(),
        }
    }
    fn number(&mut self) {
        while ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'].contains(&self.peek()) {
            self.advance();
        }
        self.add_token(TokenKind::Number);
    }
}

fn lex(s: &str) -> Result<Vec<Token>, Vec<LexErr>> {
    let mut scan = Scanner {
        source: s.to_string(),

        tokens: Vec::new(),
        errors: Vec::new(),

        lexeme_start: 0,
        next: 0,
    };

    scan.scan_tokens();
    if scan.errors.len() > 0 {
        return Err(scan.errors);
    }
    return Ok(scan.tokens);
}

#[derive(Debug, PartialEq)]
enum RootNode {
    Constant(u32),
    Roll(u32, u32),
}
impl std::fmt::Display for RootNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RootNode::Constant(c) => write!(f, "{c}"),
            RootNode::Roll(count, die) => write!(f, "{count}d{die}"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Node {
    Add(RootNode),
    Sub(RootNode),
}
impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Node::Add(root) => write!(f, "+{root}"),
            Node::Sub(root) => write!(f, "-{root}"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RollResult {
    pub die: u32,
    pub result: u32,
}
impl std::fmt::Display for RollResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "d{}:{}", self.die, self.result)
    }
}

fn roll(die: &u32) -> u32 {
    return rand::thread_rng().gen_range(1..=*die);
}

#[derive(Debug, PartialEq)]
pub struct Expr(Vec<Node>);
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let acc = self
            .0
            .iter()
            .fold(String::new(), |acc, node| acc + &node.to_string());

        let result = match acc.strip_prefix("+") {
            Some(stripped) => stripped,
            None => &acc,
        };
        write!(f, "{}", result)
    }
}
impl Expr {
    pub fn eval(&self) -> (i64, Vec<RollResult>) {
        let mut total = 0;
        let mut roll_results: Vec<RollResult> = Vec::new();
        self.0.iter().for_each(|node| {
            let (factor, root) = match node {
                Node::Add(root) => (1, root),
                Node::Sub(root) => (-1, root),
            };
            match root {
                RootNode::Constant(c) => {
                    total += factor * (*c as i64);
                }
                RootNode::Roll(count, die) => {
                    for _ in 0..*count {
                        let result = roll(&die);
                        total += factor * (result as i64);
                        roll_results.push(RollResult {
                            die: *die,
                            result: result,
                        });
                    }
                }
            }
        });

        return (total, roll_results);
    }
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}
impl Parser {
    fn at_end(&self) -> bool {
        match self.peek().kind {
            TokenKind::EOF => true,
            _ => false,
        }
    }
    fn advance(&mut self) -> Token {
        if !self.at_end() {
            self.current += 1
        }
        return self.previous();
    }
    fn previous(&self) -> Token {
        return self.tokens[self.current - 1].clone();
    }
    fn match_kind(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            return true;
        }
        return false;
    }
    fn check(&self, kind: TokenKind) -> bool {
        if self.at_end() {
            return false;
        }
        let next = self.peek();
        return kind == next.kind;
    }
    fn peek(&self) -> Token {
        return self.tokens[self.current].clone();
    }
    fn consume(&mut self, kind: TokenKind, message: &str) -> Result<Token, String> {
        if self.check(kind) {
            return Ok(self.advance());
        }
        return Err(message.to_string());
    }
    // TODO: synchronize
    fn expr(&mut self) -> Result<Expr, String> {
        let mut e: Expr = Expr(Vec::new());
        while !self.at_end() {
            let n = self.node()?;
            e.0.push(n);
        }
        return Ok(e);
    }
    fn node(&mut self) -> Result<Node, String> {
        if self.match_kind(TokenKind::Plus) {
            return Ok(Node::Add(self.root_node()?));
        } else if self.match_kind(TokenKind::Minus) {
            return Ok(Node::Sub(self.root_node()?));
        }
        return Ok(Node::Add(self.root_node()?));
    }
    fn root_node(&mut self) -> Result<RootNode, String> {
        if self.check(TokenKind::Number) {
            let num = self.consume(TokenKind::Number, "checking a number must consume number")?;
            if self.match_kind(TokenKind::D) {
                let die = self.consume(TokenKind::Number, "d must be followed by number")?;
                match (num, die) {
                    (
                        Token {
                            kind: TokenKind::Number,
                            s: num_s,
                        },
                        Token {
                            kind: TokenKind::Number,
                            s: die_s,
                        },
                    ) => {
                        let num_u: u32 = match num_s.parse() {
                            Ok(u) => u,
                            Err(e) => {
                                return Err(format!("parsing {num_s} to int: {}", e));
                            }
                        };
                        let die_u: u32 = match die_s.parse() {
                            Ok(u) => u,
                            Err(e) => {
                                return Err(format!("parsing {die_s} to int: {}", e));
                            }
                        };
                        return Ok(RootNode::Roll(num_u, die_u));
                    }
                    (x, y) => return Err(format!("invalid XdY setup, {:#?}, {:#?}", x, y)),
                }
            } else {
                match num {
                    Token {
                        kind: TokenKind::Number,
                        s: num_s,
                    } => {
                        let num_u: u32 = match num_s.parse() {
                            Ok(u) => u,
                            Err(e) => {
                                return Err(format!("parsing {num_s} to int: {}", e));
                            }
                        };
                        return Ok(RootNode::Constant(num_u));
                    }
                    x => return Err(format!("invalid const setup, {:#?}", x)),
                }
            }
        } else if self.match_kind(TokenKind::D) {
            let die = self.consume(TokenKind::Number, "d must be followed by number")?;
            match die {
                Token {
                    kind: TokenKind::Number,
                    s: die_s,
                } => {
                    let die_u: u32 = match die_s.parse() {
                        Ok(u) => u,
                        Err(e) => {
                            return Err(format!("parsing {die_s} to int: {}", e));
                        }
                    };
                    return Ok(RootNode::Roll(1, die_u));
                }
                x => return Err(format!("invalid dX setup, {:#?}", x)),
            }
        }
        return Err("parser implementation error".to_string());
    }
}

pub fn parse(s: &str) -> Result<Expr, String> {
    let tokens = match lex(s.trim()) {
        Ok(tokens) => tokens,
        Err(errs) => {
            let s = errs.iter().fold(String::new(), |acc, e| acc + e);
            return Err(s);
        }
    };
    let mut p = Parser {
        tokens: tokens,
        current: 0,
    };
    let expr = p.expr()?;
    return Ok(expr);
}
