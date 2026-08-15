use std::string::ParseError;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LParen,     /// We don't need to hold a value for tokens alike, their enum names tells us everthing we need
    RParen,
    LBrace,
    RBrace,
    LCurl,
    RCurl,
    Plus,
    Minus,
    Mul,
    Div,
    Lt,
    Gt,
    LtEq,
    GtEq,
    EqEq,
    NotEq,
    Comma,
    Colon,
    Let,
    Fn,
    Eq,
    Dot,
    Struct,
    If,
    Else,
    Number(f64),
    Ident(String),
    String(String),
}

/// This lexer will follow a pattern of generating one token at a time, or a vector of tokens.
/// We do need a lifetime annotation since our lexer will be taking a slice of the source code.
/// like a `std::string_view` in C++.
struct Lexer<'a> {
    source: &'a str,
    pos: usize,
}

/// We also need to add the annotations to the impl.
impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source, pos: 0 }
    }

    /// Return the next token in the source code.
    pub fn next(&mut self) -> Option<Token> {
        self.skip_ws();

        if !self.not_eof() {
            return None;
        }

        let cur = self.current();
        match cur {
            b'(' => { self.advance(); Some(Token::LParen) }
            b')' => { self.advance(); Some(Token::RParen) }
            b'[' => { self.advance(); Some(Token::LBrace) }
            b']' => { self.advance(); Some(Token::RBrace) }
            b'{' => { self.advance(); Some(Token::LCurl) }
            b'}' => { self.advance(); Some(Token::RCurl) }
            b'+' => { self.advance(); Some(Token::Plus) }
            b'-' => { self.advance(); Some(Token::Minus) }
            b'*' => { self.advance(); Some(Token::Mul) }
            b'/' => { self.advance(); Some(Token::Div) }
            b',' => { self.advance(); Some(Token::Comma) }
            b':' => { self.advance(); Some(Token::Colon) }
            b'.' => { self.advance(); Some(Token::Dot) }
            b'=' => {
                self.advance();
                if self.not_eof() && self.current() == b'=' {
                    self.advance();
                    Some(Token::EqEq)
                } else {
                    Some(Token::Eq)
                }
            }
            b'!' => {
                self.advance();
                if self.not_eof() && self.current() == b'=' {
                    self.advance();
                    Some(Token::NotEq)
                } else {
                    None
                }
            }
            b'<' => {
                self.advance();
                if self.not_eof() && self.current() == b'=' {
                    self.advance();
                    Some(Token::LtEq)
                } else {
                    Some(Token::Lt)
                }
            }
            b'>' => {
                self.advance();
                if self.not_eof() && self.current() == b'=' {
                    self.advance();
                    Some(Token::GtEq)
                } else {
                    Some(Token::Gt)
                }
            }
            _ => {
                if char::is_alphabetic(cur as char) {
                    let start = self.pos;
                    while self.not_eof()
                        && char::is_alphanumeric(self.current() as char)
                    {
                        self.advance();
                    }

                    return Some(match self.source[start..self.pos].as_ref() {
                        "let"       => Token::Let,
                        "fn"        => Token::Fn,
                        "struct"    => Token::Struct,
                        "if"        => Token::If,
                        "else"      => Token::Else,
                        s           => Token::Ident(s.to_string())
                    });
                } else if char::is_digit(cur as char, 10) {
                    let start = self.pos;
                    let mut flt = false;
                    while self.not_eof() {
                        let b = self.current();
                        if char::is_digit(b as char, 10) {
                            self.advance();
                        } else if b == b'.' && !flt {
                            flt = true;
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    return Some(Token::Number(self.source[start..self.pos].parse().unwrap()));
                } else if cur == b'"' {
                    self.advance();
                    let start = self.pos;

                    while self.not_eof() {
                        match self.current() {
                            b'\\' => {
                                self.advance();
                                if self.not_eof() {
                                    self.advance();
                                }
                            }
                            b'"' => break,
                            _ => self.advance(),
                        }
                    }

                    let value = self.source[start..self.pos].to_string();
                    self.advance();

                    return Some(Token::String(value));
                }
                None
            }
        }
    }

    /// Return the whole source code as tokens, in a Vec.
    pub fn all(&mut self) -> Vec<Token> {
        std::iter::from_fn(|| self.next()).collect()
    }

    fn current(&self) -> u8 {
        self.source.as_bytes()[self.pos]
    }

    fn advance(&mut self) {
        if self.not_eof() { self.pos += 1; }
    }

    fn skip_ws(&mut self) {
        while self.not_eof() && (self.current() as char).is_ascii_whitespace() {
            self.advance();
        }
    }

    fn not_eof(&self) -> bool {
        self.pos < self.source.len()
    }

}

/// An enum for binary operations
#[derive(Debug, PartialEq, Clone)]
pub enum BinOps {
    Add, Sub, Mul, Div,
    Lt, Gt, LtEq, GtEq, EqEq, NotEq,
}

/// Allow the programmer to use a type system
#[derive(Debug, PartialEq)]
pub enum TypeExpr {
    Named(String),
    Function(Vec<TypeExpr>, Box<TypeExpr>),
}

/// Here is a simple AST enum variant.
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Expr {
    Number(f64),
    String(String),
    Ident(String),
    List(Vec<Expr>),
    BinOp { op: BinOps, lhs: Box<Expr>, rhs: Box<Expr> },
    Call { callee: String, args: Vec<Expr> },
    FieldAccess { receiver: Box<Expr>, field: String },
    If {
        cond: Box<Expr>,
        then: Vec<Stmt>,
        else_: Option<Vec<Stmt>>,
    },
    Lambda {
        params: Option<Vec<Expr>>,
        value: Box<Expr>
    }
}

#[derive(Debug, PartialEq)]
pub struct Field {
    pub name: String,
    pub ty: TypeExpr,
}

#[derive(Debug, PartialEq)]
pub struct Param {
    pub name: String,
    pub ty: Option<TypeExpr>,
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Let {
        name: String,
        value: Expr,
        ty: Option<TypeExpr>,
    },
    FnDecl {
        name: String,
        params: Vec<Param>,
        body: Vec<Stmt>,
        ret: Option<TypeExpr>
    },
    Struct {
        name: String,
        fields: Vec<Field>
    }
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub stmts: Vec<Stmt>
}

/// It's good to define a custom error enum to handle errors cleanly.
#[derive(Debug)]
pub enum ParseErr {
    Unexpected { expected: Token, found: Token, msg: &'static str },
    UnexpectedToken(Token),
    UnexpectedEof,
}

/// Now we need to pass the same lifetime annotation through to the parser and lexer
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Option<Token>
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut p = Self { lexer: Lexer::new(source), current: None };
        p.consume();
        p
    }

    /// Take the current token we have, advance then return the previous token we were at
    fn consume(&mut self) -> Option<Token> {
        let pr = self.current.take();
        self.current = self.lexer.next();
        pr
    }

    /// Expect the current token to match a token of our choice, if it doesn't an error will be returned
    /// If the current token matches, we'll be returned the consumed token and the parser will advance
    fn expect(&mut self, tk: Token, msg: &'static str) -> Result<Token, ParseErr> {
        match &self.current {
            Some(c) if *c == tk => Ok(self.consume().unwrap()),
            Some(c) => Err(ParseErr::Unexpected { expected: tk, found: c.clone(), msg }),
            None => Err(ParseErr::UnexpectedEof)
        }
    }

    /// The main parser driver function, it parses all statements
    /// and returns a program.
    pub fn parse(&mut self) -> Result<Program, ParseErr> {
        let mut stmts: Vec<Stmt> = vec![];
        while self.current.is_some() {
            stmts.push(self.parse_stmt()?);
        }

        Ok(Program { stmts })
    }

    /// Now we're adding the ability to parse statements
    /// Expr statements are just expressions as statements, so wrap parse_expr.
    fn parse_stmt(&mut self) -> Result<Stmt, ParseErr> {
        match self.current {
            Some(Token::Let) => self.parse_let(),
            Some(Token::Fn) => self.parse_fn_decl(),
            Some(Token::Struct) => self.parse_struct(),
            _ => Ok(Stmt::Expr(self.parse_expr()?)),
        }
    }

    /// This is the first parse function of the parser,
    /// Grammar being parsed: expr        <- comparison
    fn parse_expr(&mut self) -> Result<Expr, ParseErr> {
        self.parse_comparison()
    }

    /// This is the second parse function of the parser,
    /// Grammar being parsed: term    <- factor   (_ [*/] _ factor)*
    fn parse_comparison(&mut self) -> Result<Expr, ParseErr> {
        let mut lhs = self.parse_additive()?;

        while let Some(t) = &self.current {
            let op = match t {
                Token::Lt    => BinOps::Lt,
                Token::Gt    => BinOps::Gt,
                Token::LtEq  => BinOps::LtEq,
                Token::GtEq  => BinOps::GtEq,
                Token::EqEq  => BinOps::EqEq,
                Token::NotEq => BinOps::NotEq,
                _ => break,
            };

            self.consume();
            let rhs = self.parse_term()?;
            lhs = Expr::BinOp { op, lhs: Box::new(lhs), rhs: Box::new(rhs) };
        }

        Ok(lhs)
    }

    /// This is the third parse function of the parser,
    /// Gramamr being parsed: <- term ([+-] term)*
    fn parse_additive(&mut self) -> Result<Expr, ParseErr> {
        let mut lhs = self.parse_term()?;

        while let Some(t) = &self.current {
            let op = match t {
                Token::Plus => BinOps::Add,
                Token::Minus => BinOps::Sub,
                _ => break
            };

            self.consume();
            let rhs = self.parse_term()?;
            lhs = Expr::BinOp { op, lhs: Box::new(lhs), rhs: Box::new(rhs) };
        }

        Ok(lhs)
    }

    /// This is the third parse function of the parser,
    /// Grammar being parsed: term    <- factor   (_ [*/] _ factor)*
    fn parse_term(&mut self) -> Result<Expr, ParseErr> {
        // the reason why we're calling parse_factor her is to get the left side
        // postfix needs to wrap that result before */
        // the chain of parsing looks like this: factor -> postfix -> term
        // term is using postfix expression as lhs
        let primary = self.parse_factor()?;
        let mut lhs = self.parse_postfix(primary)?;

        while let Some(t) = &self.current {
            // Because we're parsing an expression,
            // We want to find * or /
            let op = match t {
                Token::Mul => BinOps::Mul,
                Token::Div => BinOps::Div,
                _ => break,
            };

            self.consume();
            let factor = self.parse_factor()?;
            let rhs = self.parse_postfix(factor)?;
            lhs = Expr::BinOp { op, lhs: Box::new(lhs), rhs: Box::new(rhs) };
        }

        Ok(lhs)
    }

    /// This is the fourth parse function of the parser,
    /// Grammar being parsed: factor  <- '(' _ expr _ ')' / number
    fn parse_factor(&mut self) -> Result<Expr, ParseErr> {
        match self.consume() {
            Some(Token::Number(n))  => Ok(Expr::Number(n)),
            Some(Token::String(s))  => Ok(Expr::String(s)),
            Some(Token::LBrace)     => Ok(self.parse_list()?),
            Some(Token::If)         => Ok(self.parse_if()?),
            Some(Token::Ident(s))   => {
                if matches!(self.peek(), Some(Token::LParen)) {
                    self.parse_call(s)
                } else {
                    Ok(Expr::Ident(s))
                }
            },
            Some(Token::LParen)     => {
                // Here we're parsing: '(' _ expr _ ')'
                let expr = self.parse_expr()?; // this is what makes this recursive descent
                self.expect(Token::RParen, "expected ')'")?;
                Ok(expr)
            },
            Some(Token::Fn)         => {
                self.expect(Token::LParen, "expected '(' after `fn` to create an anonymous function")?;
                let params = if self.peek() == Some(&Token::RParen) {
                    self.consume();
                    None
                } else {
                    Some(self.parse_list_of_exprs(Token::RParen)?)
                };

                self.expect(Token::RParen, "expected ')' after params")?;
                self.expect(Token::LCurl, "expected '{' after function params")?;

                let value = self.parse_expr()?;

                self.expect(Token::RCurl, "expected '}' to close function body")?;

                Ok(Expr::Lambda {
                    params,
                    value: Box::new(value)
                })
            },
            Some(t) => Err(ParseErr::UnexpectedToken(t)),
            None    => Err(ParseErr::UnexpectedEof),
        }
    }

    /// This is the fifth parse function of the parser,
    /// It handles postfix operations on a primary expr
    /// Grammar: postfix <- primary ('.' ident)*
    fn parse_postfix(&mut self, mut expr: Expr) -> Result<Expr, ParseErr> {
        while matches!(self.peek(), Some(Token::Dot)) {
            self.consume();
            let field = self.get_identifier_value()?;
            expr = Expr::FieldAccess { receiver: Box::new(expr), field };
        }

        Ok(expr)
    }

    /// Parse a function call
    /// Grammar being parsed: call  <- ident '(' (_ expr ',' _)* ')'
    fn parse_call(&mut self, callee: String) -> Result<Expr, ParseErr> {
        self.expect(Token::LParen, "expected '('")?;

        let args = self.parse_list_of_exprs(Token::RParen)?;

        // Expect the closing parenthesis
        self.expect(Token::RParen, "expected ')'")?;

        Ok(Expr::Call { callee, args })
    }

    /// Parse a list
    /// Grammar being parsed: list  <- '[' (_ expr ',' _)* ']'
    fn parse_list(&mut self) -> Result<Expr, ParseErr> {
        let elements = self.parse_list_of_exprs(Token::RBrace)?;
        self.expect(Token::RBrace, "expected ']'")?;

        Ok(Expr::List(elements))
    }

    /// Parse an if expression
    /// Grammar being parsed: if <- 'if' expr '{' stmt* '}' ('else' '{' stmt* '}')?
    fn parse_if(&mut self) -> Result<Expr, ParseErr> {
        // if is already consumed so we dont need to expect it here
        let cond = self.parse_expr()?;
        let then = self.parse_block()?;
        let else_ = if matches!(self.current, Some(Token::Else)) {
            self.consume();
            Some(self.parse_block()?)
        } else {
            None
        };

        Ok(Expr::If {
            cond: Box::new(cond),
            then,
            else_
        })
    }

    /// Parse a let decl
    /// Grammar: let    <- 'let' ident '=' expr
    fn parse_let(&mut self) -> Result<Stmt, ParseErr> {
        self.expect(Token::Let, "expected 'let'")?;

        // the next token would be an identifier, so expect it.
        let name = self.get_identifier_value()?;

        // we're parsing an optional type annotation,
        // if we see a colon, consume it and parse a type expression
        let ty = if matches!(self.current, Some(Token::Colon)) {
            self.consume();
            Some(self.parse_type_expr()?)
        } else {
            None
        };

        // expect '=' after let name.
        self.expect(Token::Eq, "expected '=' after name")?;

        // now simply parse an expression for the value
        let value = self.parse_expr()?;

        Ok(Stmt::Let {
            name,
            value,
            ty
        })
    }

    /// Parse a function declaration and body
    /// Grammar: func_decl <- 'fn' ident '(' (ident ':' type (',' ident ':' type)*)? ')' type? '{' stmt* '}'
    fn parse_fn_decl(&mut self) -> Result<Stmt, ParseErr> {
        self.expect(Token::Fn, "expected 'fn'")?;

        // the next token would be an identifier, so expect it.
        let name = self.get_identifier_value()?;

        self.expect(Token::LParen, "expected '(' after name")?;

        let mut params = vec![];
        while !matches!(self.current, Some(Token::RParen) | None) {
            let param_name = self.get_identifier_value()?;
            self.expect(Token::Colon, "expected ':' after param name")?;
            let ty = self.parse_type_expr()?;
            params.push(Param { name: param_name, ty: Some(ty) });

            if !matches!(self.current, Some(Token::RParen)) {
                self.expect(Token::Comma, "expected ','")?;
            }
        }

        // expect the closing parenthesis
        self.expect(Token::RParen, "expected ')'")?;

        let ret_ty = if matches!(self.current, Some(Token::Ident(_))) {
            Some(self.parse_type_expr()?)
        } else {
            None
        };

        let body = self.parse_block()?;

        Ok(Stmt::FnDecl {
            name,
            params,
            body,
            ret: ret_ty
        })
    }

    /// Parser a structure declaration
    /// Grammar: struct <- 'struct' ident '{' (ident ':' type (',' ident ':' type)*)? '}'
    fn parse_struct(&mut self) -> Result<Stmt, ParseErr> {
        self.consume();

        let name = self.get_identifier_value()?;

        self.expect(Token::LCurl, "expected '{' after struct name")?;

        let mut fields = vec![];

        loop {
            if self.current.as_ref() == Some(&Token::RCurl) { break; }

            let name = self.get_identifier_value()?;
            self.expect(Token::Colon, "expected ':' after field name")?;
            let ty = self.parse_type_expr()?;

            fields.push(Field { name, ty });

            if self.current.as_ref() == Some(&Token::RCurl) { break; }
            self.expect(Token::Comma, "expected ','")?;
        }

        self.expect(Token::RCurl, "expected '}'")?;

        Ok(Stmt::Struct { name, fields })
    }

    fn parse_list_of_exprs(&mut self, delim: Token) -> Result<Vec<Expr>, ParseErr> {
        let mut exprs: Vec<Expr> = vec![];

        if self.current.as_ref() == Some(&delim) {
            return Ok(exprs);
        }

        loop {
            exprs.push(self.parse_expr()?);

            if self.current.as_ref() == Some(&delim) {
                break;
            }

            self.expect(Token::Comma, "expected ','")?;
        }

        Ok(exprs)
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, ParseErr> {
        self.expect(Token::LCurl, "expected '{'")?;

        let mut stmts = vec![];
        // So while we're not matching against a '}' currently...
        while !matches!(self.current, Some(Token::RCurl)) {
            if self.current.is_none() {
                return Err(ParseErr::UnexpectedEof);
            }

            stmts.push(self.parse_stmt()?);
        }

        self.expect(Token::RCurl, "expected '}'")?;

        Ok(stmts)
    }

    /// Simply gets an identifier value and returns a named TypeExpr
    fn parse_type_expr(&mut self) -> Result<TypeExpr, ParseErr> {
        let name = self.get_identifier_value()?;
        Ok(TypeExpr::Named(name))
    }

    fn get_identifier_value(&mut self) -> Result<String, ParseErr> {
        match self.consume() {
            Some(Token::Ident(s)) => Ok(s),
            Some(t) => Err(ParseErr::UnexpectedToken(t)),
            None => Err(ParseErr::UnexpectedEof),
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.current.as_ref()
    }
}