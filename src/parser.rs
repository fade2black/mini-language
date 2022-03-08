use crate::ast::{Ast, ExprNode, Function, Prototype};
use crate::error_logger::ErrorLogger;
use crate::lexer::Lexer;
use crate::operator::Operator;
use crate::token::Token;

pub struct Parser<T>
where
    T: std::io::Read,
{
    lexer: Lexer<T>,
    token: Token,
    asts: Vec<Ast>,
    err_logger: ErrorLogger,
}

impl<T> Parser<T>
where
    T: std::io::Read,
{
    pub fn new(src: T) -> Self {
        Self {
            lexer: Lexer::new(src),
            token: Token::None,
            err_logger: ErrorLogger::new(),
            asts: vec![],
        }
    }

    fn get_token(&mut self) {
        self.token = self.lexer.get_token();
    }

    pub fn get_asts(&self) -> &Vec<Ast> {
        &self.asts
    }

    pub fn get_error_logger(&self) -> &ErrorLogger {
        &self.err_logger
    }

    pub fn lexeme(&mut self) -> String {
        self.lexer.lexeme.clone()
    }

    pub fn push_error(&mut self, msg: &str) {
        self.err_logger.push(self.lexer.get_line_numer(), msg);
    }

    fn parse_expression(&mut self) -> Option<ExprNode> {
        let mut node = self.parse_subexpression()?;

        while self.token.is_comparison_operator() {
            let op = Operator::from(&self.token);

            self.get_token();

            let rhs = self.parse_subexpression()?;
            node = ExprNode::create_binary_op(op, node, rhs);
        }
        Some(node)
    }

    fn parse_subexpression(&mut self) -> Option<ExprNode> {
        let mut node = self.parse_term()?;

        while self.token.is_addition_operator() {
            let op = Operator::from(&self.token);

            self.get_token();

            let rhs = self.parse_term()?;
            node = ExprNode::create_binary_op(op, node, rhs);
        }

        Some(node)
    }

    fn parse_term(&mut self) -> Option<ExprNode> {
        let mut node = self.parse_factor()?;

        while self.token.is_multiplication_operator() {
            let op = Operator::from(&self.token);

            self.get_token();

            let rhs = self.parse_factor()?;
            node = ExprNode::create_binary_op(op, node, rhs);
        }

        Some(node)
    }

    fn parse_factor(&mut self) -> Option<ExprNode> {
        match self.token {
            Token::Minus => self.parse_neg_expr(),
            Token::Identifier => self.parse_identifier_expr(),
            Token::Lpar => self.parse_paren_expr(),
            Token::Number => self.parse_number_expr(),
            _ => {
                self.push_error("Expected identifier or number");
                None
            }
        }
    }

    fn parse_identifier_expr(&mut self) -> Option<ExprNode> {
        let id_name = self.lexeme();

        self.get_token();
        if self.token != Token::Lpar {
            return Some(ExprNode::Variable(id_name));
        }

        self.get_token();

        let mut args = vec![];
        if self.token != Token::Rpar {
            loop {
                if let Some(expr_node) = self.parse_expression() {
                    args.push(expr_node);
                } else {
                    return None;
                }

                if self.token == Token::Rpar {
                    break;
                }

                if self.token != Token::Comma {
                    self.push_error("Expected ')' or ',' in argument list");

                    return None;
                }
                self.get_token();
            }
        }

        self.get_token();

        Some(ExprNode::create_call(id_name, args))
    }

    fn parse_number_expr(&mut self) -> Option<ExprNode> {
        let node = ExprNode::Number(self.lexeme().parse().unwrap());
        self.get_token();
        Some(node)
    }

    fn parse_paren_expr(&mut self) -> Option<ExprNode> {
        self.get_token();

        let node = self.parse_expression()?;

        if self.token != Token::Rpar {
            self.push_error("Missing ')'");

            return None;
        }
        self.get_token();

        Some(node)
    }

    fn parse_neg_expr(&mut self) -> Option<ExprNode> {
        self.get_token();

        let node = self.parse_expression()?;
        Some(ExprNode::create_unary_op(Operator::Neg, node))
    }

    fn parse_if_expr(&mut self) -> Option<ExprNode> {
        self.get_token();
        let cond = self.parse_expression()?;

        if self.token != Token::Then {
            self.push_error("Expected 'then'");
            return None;
        }

        self.get_token();
        let then_branch = self.parse_expression()?;

        if self.token != Token::Else {
            self.push_error("Expected 'else'");
            return None;
        }

        self.get_token();
        let else_branch = self.parse_expression()?;

        Some(ExprNode::create_if_then_else(
            cond,
            then_branch,
            else_branch,
        ))
    }

    fn parse_definition(&mut self) -> Option<Function> {
        self.get_token();
        let proto = self.parse_prototype()?;

        let expr_node = match self.token {
            Token::If => self.parse_if_expr()?,
            _ => self.parse_expression()?,
        };

        Some(Function::new(proto, expr_node))
    }

    fn parse_prototype(&mut self) -> Option<Prototype> {
        if self.token != Token::Identifier {
            self.push_error("Expected function name in prototype");
            return None;
        }

        let id_name = self.lexeme();

        self.get_token();
        if self.token != Token::Lpar {
            self.push_error("Expected '(' in prototype");
            return None;
        }

        self.get_token();

        let mut args = vec![];
        while self.token == Token::Identifier {
            args.push(self.lexeme());
            self.get_token();
        }

        if self.token != Token::Rpar {
            self.push_error("Expected ')' in prototype");
            return None;
        }

        self.get_token();
        Some(Prototype::new(id_name, args))
    }

    fn synchronize(&mut self, tokens: Vec<Token>) {
        while !tokens.contains(&self.token) {
            self.get_token();
        }
    }

    fn handle_definition(&mut self) {
        if let Some(node) = self.parse_definition() {
            self.asts.push(Ast::Definition(node))
        } else {
            self.synchronize(vec![Token::Eof, Token::Semicolon]);
        }
    }

    pub fn main_loop(&mut self) {
        self.get_token();

        loop {
            match self.token {
                Token::Eof => break,
                Token::Define => self.handle_definition(),
                _ => {
                    self.push_error("Expected 'def'");
                    self.synchronize(vec![Token::Eof, Token::Semicolon]);
                }
            }

            if self.token != Token::Semicolon {
                self.push_error("Missing ';'");
            } else {
                self.get_token();
            }
        }
    }
}
