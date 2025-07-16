use crate::lang::lexer::{Token, TokenType, LiteralValue};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    // Expressions
    Literal(LiteralValue, SourceLocation),
    Identifier(String, SourceLocation),
    BinaryOp {
        left: Box<AstNode>,
        operator: BinaryOperator,
        right: Box<AstNode>,
        location: SourceLocation,
    },
    UnaryOp {
        operator: UnaryOperator,
        operand: Box<AstNode>,
        location: SourceLocation,
    },
    PropertyAccess {
        object: Box<AstNode>,
        property: String,
        location: SourceLocation,
    },
    IndexAccess {
        object: Box<AstNode>,
        index: Box<AstNode>,
        location: SourceLocation,
    },
    FunctionCall {
        function: Box<AstNode>,
        arguments: Vec<Argument>,
        location: SourceLocation,
    },
    Assignment {
        target: Box<AstNode>,
        operator: AssignmentOperator,
        value: Box<AstNode>,
        location: SourceLocation,
    },
    If {
        condition: Box<AstNode>,
        then_branch: Box<AstNode>,
        else_ifs: Vec<(AstNode, AstNode)>,
        else_branch: Option<Box<AstNode>>,
        location: SourceLocation,
    },
    Match {
        value: Box<AstNode>,
        arms: Vec<MatchArm>,
        location: SourceLocation,
    },
    Try {
        body: Box<AstNode>,
        catch_clause: Option<CatchClause>,
        location: SourceLocation,
    },
    Block {
        statements: Vec<AstNode>,
        location: SourceLocation,
    },
    ListLiteral {
        elements: Vec<AstNode>,
        location: SourceLocation,
    },
    DictLiteral {
        entries: Vec<(AstNode, AstNode)>,
        location: SourceLocation,
    },
    AnonymousFunction {
        parameters: Vec<Parameter>,
        body: Box<AstNode>,
        location: SourceLocation,
    },
    
    // Statements
    ExpressionStatement {
        expression: Box<AstNode>,
        location: SourceLocation,
    },
    VariableDeclaration {
        name: String,
        value: Option<Box<AstNode>>,
        location: SourceLocation,
    },
    FunctionDeclaration {
        visibility: Visibility,
        name: String,
        parameters: Vec<Parameter>,
        body: Box<AstNode>,
        location: SourceLocation,
    },
    ClassDeclaration {
        visibility: Visibility,
        name: String,
        members: Vec<ClassMember>,
        location: SourceLocation,
    },
    ImportStatement {
        path: Vec<String>,
        clause: Option<ImportClause>,
        location: SourceLocation,
    },
    WhileStatement {
        condition: Box<AstNode>,
        body: Box<AstNode>,
        location: SourceLocation,
    },
    ForStatement {
        variable: String,
        index_variable: Option<String>,
        iterable: Box<AstNode>,
        body: Box<AstNode>,
        location: SourceLocation,
    },
    MatchStatement {
        value: Box<AstNode>,
        arms: Vec<MatchArm>,
        location: SourceLocation,
    },
    TryStatement {
        body: Box<AstNode>,
        catch_clause: Option<CatchClause>,
        finally_clause: Option<Box<AstNode>>,
        location: SourceLocation,
    },
    WithStatement {
        bindings: Vec<WithBinding>,
        body: Box<AstNode>,
        location: SourceLocation,
    },
    BreakStatement {
        value: Option<Box<AstNode>>,
        location: SourceLocation,
    },
    ContinueStatement {
        location: SourceLocation,
    },
    ReturnStatement {
        value: Option<Box<AstNode>>,
        location: SourceLocation,
    },
    
    // Module level
    Program {
        items: Vec<AstNode>,
        location: SourceLocation,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    // Arithmetic
    Add, Subtract, Multiply, Divide, Modulo, Power,
    // Comparison
    Equal, NotEqual, Less, Greater, LessEqual, GreaterEqual,
    // Logical
    And, Or,
    // Bitwise
    BitwiseAnd, BitwiseOr, BitwiseXor, LeftShift, RightShift,
    // Membership
    In, Is,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Plus, Minus, Not, BitwiseNot,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignmentOperator {
    Assign, AddAssign, SubtractAssign, MultiplyAssign, DivideAssign, ModuloAssign,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    pub value: AstNode,
    pub spread: bool,
    pub keyword_spread: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub default_value: Option<AstNode>,
    pub variadic: bool,
    pub keyword_variadic: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<AstNode>,
    pub body: AstNode,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Literal(LiteralValue),
    Variable(String),
    Wildcard,
    List(Vec<Pattern>),
    Dict(Vec<(String, Pattern)>),
    Range { start: AstNode, end: AstNode, inclusive: bool },
    Or(Vec<Pattern>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CatchClause {
    pub variable: String,
    pub body: AstNode,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassMember {
    Field {
        visibility: Visibility,
        name: String,
        value: Option<AstNode>,
    },
    Method {
        visibility: Visibility,
        is_static: bool,
        declaration: AstNode, // FunctionDeclaration
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImportClause {
    All,
    Named(Vec<String>),
    Single(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct WithBinding {
    pub name: String,
    pub value: AstNode,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub byte_offset: usize,
}

impl SourceLocation {
    pub fn from_token(token: &Token) -> Self {
        SourceLocation {
            line: token.line,
            column: token.column,
            byte_offset: token.byte_offset,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub location: SourceLocation,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error at {}:{}: {}", 
               self.location.line, self.location.column, self.message)
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<ParseError>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            errors: Vec::new(),
        }
    }
    
    pub fn parse(&mut self) -> Result<AstNode, Vec<ParseError>> {
        let start_location = self.current_location();
        let mut items = Vec::new();
        
        // Skip leading newlines
        self.skip_newlines();
        
        while !self.is_at_end() {
            if let Some(item) = self.parse_module_item() {
                items.push(item);
            }
            self.skip_newlines();
        }
        
        if !self.errors.is_empty() {
            return Err(self.errors.clone());
        }
        
        Ok(AstNode::Program {
            items,
            location: start_location,
        })
    }
    
    fn parse_module_item(&mut self) -> Option<AstNode> {
        match self.peek().token_type {
            TokenType::Use => self.parse_import_statement(),
            TokenType::Fn => self.parse_function_declaration(Visibility::Private),
            TokenType::Pub => {
                self.advance(); // consume 'pub'
                match self.peek().token_type {
                    TokenType::Fn => self.parse_function_declaration(Visibility::Public),
                    TokenType::Class => self.parse_class_declaration(Visibility::Public),
                    _ => {
                        self.error("Expected function or class declaration after 'pub'");
                        None
                    }
                }
            }
            TokenType::Class => self.parse_class_declaration(Visibility::Private),
            TokenType::Var => self.parse_variable_declaration(),
            _ => self.parse_statement(),
        }
    }
    
    fn parse_import_statement(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::Use, "Expected 'use'")?;
        
        let mut path = Vec::new();
        path.push(self.consume_identifier("Expected module name")?);
        
        while self.match_token(&TokenType::DoubleColon) {
            if self.match_token(&TokenType::Star) {
                let clause = ImportClause::All;
                self.consume(TokenType::Semicolon, "Expected ';' after import statement")?;
                return Some(AstNode::ImportStatement {
                    path,
                    clause: Some(clause),
                    location,
                });
            } else if self.check(&TokenType::LeftBrace) {
                self.advance(); // consume '{'
                let mut names = Vec::new();
                names.push(self.consume_identifier("Expected identifier")?);
                
                while self.match_token(&TokenType::Comma) {
                    if self.check(&TokenType::RightBrace) {
                        break;
                    }
                    names.push(self.consume_identifier("Expected identifier")?);
                }
                
                self.consume(TokenType::RightBrace, "Expected '}'")?;
                let clause = ImportClause::Named(names);
                self.consume(TokenType::Semicolon, "Expected ';' after import statement")?;
                return Some(AstNode::ImportStatement {
                    path,
                    clause: Some(clause),
                    location,
                });
            } else {
                let name = self.consume_identifier("Expected identifier")?;
                let clause = ImportClause::Single(name);
                self.consume(TokenType::Semicolon, "Expected ';' after import statement")?;
                return Some(AstNode::ImportStatement {
                    path,
                    clause: Some(clause),
                    location,
                });
            }
        }
        
        self.consume(TokenType::Semicolon, "Expected ';' after import statement")?;
        Some(AstNode::ImportStatement {
            path,
            clause: None,
            location,
        })
    }
    
    fn parse_function_declaration(&mut self, visibility: Visibility) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::Fn, "Expected 'fn'")?;
        let name = self.consume_identifier("Expected function name")?;
        
        self.consume(TokenType::LeftParen, "Expected '(' after function name")?;
        let parameters = self.parse_parameter_list();
        self.consume(TokenType::RightParen, "Expected ')' after parameters")?;
        
        let body = self.parse_block()?;
        
        Some(AstNode::FunctionDeclaration {
            visibility,
            name,
            parameters,
            body: Box::new(body),
            location,
        })
    }
    
    fn parse_class_declaration(&mut self, visibility: Visibility) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::Class, "Expected 'class'")?;
        let name = self.consume_identifier("Expected class name")?;
        
        self.consume(TokenType::LeftBrace, "Expected '{' after class name")?;
        
        let mut members = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if let Some(member) = self.parse_class_member() {
                members.push(member);
            }
            self.skip_newlines();
        }
        
        self.consume(TokenType::RightBrace, "Expected '}' after class body")?;
        
        Some(AstNode::ClassDeclaration {
            visibility,
            name,
            members,
            location,
        })
    }
    
    fn parse_class_member(&mut self) -> Option<ClassMember> {
        let member_visibility = if self.match_token(&TokenType::Pub) {
            Visibility::Public
        } else {
            Visibility::Private
        };
        
        if self.match_token(&TokenType::Static) {
            if self.check(&TokenType::Fn) {
                if let Some(AstNode::FunctionDeclaration { .. }) = self.parse_function_declaration(member_visibility.clone()) {
                    return Some(ClassMember::Method {
                        visibility: member_visibility,
                        is_static: true,
                        declaration: self.parse_function_declaration(member_visibility)?,
                    });
                }
            }
        } else if self.check(&TokenType::Fn) {
            if let Some(declaration) = self.parse_function_declaration(member_visibility.clone()) {
                return Some(ClassMember::Method {
                    visibility: member_visibility,
                    is_static: false,
                    declaration,
                });
            }
        } else if self.check(&TokenType::Var) {
            self.advance(); // consume 'var'
            let name = self.consume_identifier("Expected field name")?;
            let value = if self.match_token(&TokenType::Equal) {
                Some(self.parse_expression()?)
            } else {
                None
            };
            self.consume(TokenType::Semicolon, "Expected ';' after field declaration")?;
            return Some(ClassMember::Field {
                visibility: member_visibility,
                name,
                value,
            });
        }
        
        None
    }
    
    fn parse_parameter_list(&mut self) -> Vec<Parameter> {
        let mut parameters = Vec::new();
        
        if self.check(&TokenType::RightParen) {
            return parameters;
        }
        
        loop {
            let mut variadic = false;
            let mut keyword_variadic = false;
            
            if self.match_token(&TokenType::Star) {
                variadic = true;
            } else if self.match_token(&TokenType::StarStar) {
                keyword_variadic = true;
            }
            
            let name = match self.consume_identifier("Expected parameter name") {
                Some(name) => name,
                None => break,
            };
            
            let default_value = if self.match_token(&TokenType::Equal) {
                Some(self.parse_expression().unwrap_or_else(|| {
                    AstNode::Literal(LiteralValue::Null, self.current_location())
                }))
            } else {
                None
            };
            
            parameters.push(Parameter {
                name,
                default_value,
                variadic,
                keyword_variadic,
            });
            
            if !self.match_token(&TokenType::Comma) {
                break;
            }
            
            if self.check(&TokenType::RightParen) {
                break;
            }
        }
        
        parameters
    }
    
    fn parse_variable_declaration(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::Var, "Expected 'var'")?;
        let name = self.consume_identifier("Expected variable name")?;
        
        let value = if self.match_token(&TokenType::Equal) {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };
        
        self.consume(TokenType::Semicolon, "Expected ';' after variable declaration")?;
        
        Some(AstNode::VariableDeclaration {
            name,
            value,
            location,
        })
    }
    
    fn parse_statement(&mut self) -> Option<AstNode> {
        match self.peek().token_type {
            TokenType::LeftBrace => self.parse_block(),
            TokenType::If => self.parse_if_statement(),
            TokenType::While => self.parse_while_statement(),
            TokenType::For => self.parse_for_statement(),
            TokenType::Match => self.parse_match_statement(),
            TokenType::Try => self.parse_try_statement(),
            TokenType::With => self.parse_with_statement(),
            TokenType::Break => self.parse_break_statement(),
            TokenType::Continue => self.parse_continue_statement(),
            TokenType::Return => self.parse_return_statement(),
            TokenType::Semicolon => {
                self.advance(); // consume semicolon
                None // Empty statement
            }
            _ => self.parse_expression_statement(),
        }
    }
    
    fn parse_block(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::LeftBrace, "Expected '{'")?;
        
        let mut statements = Vec::new();
        
        self.skip_newlines();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.skip_newlines();
        }
        
        self.consume(TokenType::RightBrace, "Expected '}'")?;
        
        Some(AstNode::Block {
            statements,
            location,
        })
    }
    
    fn parse_if_statement(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::If, "Expected 'if'")?;
        
        let condition = Box::new(self.parse_expression()?);
        let then_branch = Box::new(self.parse_block()?);
        
        let mut else_ifs = Vec::new();
        
        while self.match_token(&TokenType::Else) && self.check(&TokenType::If) {
            self.advance(); // consume 'if'
            let else_if_condition = self.parse_expression()?;
            let else_if_body = self.parse_block()?;
            else_ifs.push((else_if_condition, else_if_body));
        }
        
        let else_branch = if self.match_token(&TokenType::Else) {
            Some(Box::new(self.parse_block()?))
        } else {
            None
        };
        
        Some(AstNode::If {
            condition,
            then_branch,
            else_ifs,
            else_branch,
            location,
        })
    }
    
    fn parse_while_statement(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::While, "Expected 'while'")?;
        
        let condition = Box::new(self.parse_expression()?);
        let body = Box::new(self.parse_block()?);
        
        Some(AstNode::WhileStatement {
            condition,
            body,
            location,
        })
    }
    
    fn parse_for_statement(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::For, "Expected 'for'")?;
        
        let variable = self.consume_identifier("Expected loop variable")?;
        
        let index_variable = if self.match_token(&TokenType::Comma) {
            Some(self.consume_identifier("Expected index variable")?)
        } else {
            None
        };
        
        self.consume(TokenType::In, "Expected 'in'")?;
        let iterable = Box::new(self.parse_expression()?);
        let body = Box::new(self.parse_block()?);
        
        Some(AstNode::ForStatement {
            variable,
            index_variable,
            iterable,
            body,
            location,
        })
    }
    
    fn parse_match_statement(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::Match, "Expected 'match'")?;
        
        let value = Box::new(self.parse_expression()?);
        
        self.consume(TokenType::LeftBrace, "Expected '{' after match value")?;
        
        let mut arms = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if let Some(arm) = self.parse_match_arm() {
                arms.push(arm);
            }
            self.skip_newlines();
        }
        
        self.consume(TokenType::RightBrace, "Expected '}' after match arms")?;
        
        Some(AstNode::MatchStatement {
            value,
            arms,
            location,
        })
    }
    
    fn parse_match_arm(&mut self) -> Option<MatchArm> {
        self.consume(TokenType::Case, "Expected 'case'")?;
        let pattern = self.parse_pattern()?;
        
        let guard = if self.match_token(&TokenType::If) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        let body = self.parse_block()?;
        
        Some(MatchArm {
            pattern,
            guard,
            body,
        })
    }
    
    fn parse_pattern(&mut self) -> Option<Pattern> {
        match &self.peek().token_type {
            TokenType::IntegerLiteral | TokenType::FloatLiteral | 
            TokenType::StringLiteral | TokenType::BooleanLiteral | 
            TokenType::NullLiteral => {
                let token = self.advance();
                token.value.map(Pattern::Literal)
            }
            TokenType::Identifier => {
                let name = self.advance().lexeme;
                Some(Pattern::Variable(name))
            }
            TokenType::LeftBracket => {
                self.advance(); // consume '['
                let mut patterns = Vec::new();
                
                while !self.check(&TokenType::RightBracket) && !self.is_at_end() {
                    if let Some(pattern) = self.parse_pattern() {
                        patterns.push(pattern);
                    }
                    
                    if !self.match_token(&TokenType::Comma) {
                        break;
                    }
                }
                
                self.consume(TokenType::RightBracket, "Expected ']'")?;
                Some(Pattern::List(patterns))
            }
            TokenType::LeftBrace => {
                self.advance(); // consume '{'
                let mut entries = Vec::new();
                
                while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
                    if let Some(key_token) = self.match_string_literal() {
                        self.consume(TokenType::Colon, "Expected ':' after dict key")?;
                        if let Some(pattern) = self.parse_pattern() {
                            entries.push((key_token, pattern));
                        }
                    }
                    
                    if !self.match_token(&TokenType::Comma) {
                        break;
                    }
                }
                
                self.consume(TokenType::RightBrace, "Expected '}'")?;
                Some(Pattern::Dict(entries))
            }
            _ => {
                self.error("Invalid pattern");
                None
            }
        }
    }
    
    fn parse_try_statement(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::Try, "Expected 'try'")?;
        
        let body = Box::new(self.parse_block()?);
        
        let catch_clause = if self.match_token(&TokenType::Catch) {
            let variable = self.consume_identifier("Expected catch variable")?;
            let catch_body = self.parse_block()?;
            Some(CatchClause {
                variable,
                body: catch_body,
            })
        } else {
            None
        };
        
        let finally_clause = if self.match_token(&TokenType::Finally) {
            Some(Box::new(self.parse_block()?))
        } else {
            None
        };
        
        Some(AstNode::TryStatement {
            body,
            catch_clause,
            finally_clause,
            location,
        })
    }
    
    fn parse_with_statement(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::With, "Expected 'with'")?;
        
        let mut bindings = Vec::new();
        
        loop {
            let name = self.consume_identifier("Expected binding name")?;
            self.consume(TokenType::Equal, "Expected '=' after binding name")?;
            let value = self.parse_expression()?;
            
            bindings.push(WithBinding { name, value });
            
            if !self.match_token(&TokenType::Comma) {
                break;
            }
        }
        
        let body = Box::new(self.parse_block()?);
        
        Some(AstNode::WithStatement {
            bindings,
            body,
            location,
        })
    }
    
    fn parse_break_statement(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::Break, "Expected 'break'")?;
        
        let value = if !self.check(&TokenType::Semicolon) && !self.check(&TokenType::Newline) {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };
        
        self.consume(TokenType::Semicolon, "Expected ';' after break statement")?;
        
        Some(AstNode::BreakStatement {
            value,
            location,
        })
    }
    
    fn parse_continue_statement(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::Continue, "Expected 'continue'")?;
        self.consume(TokenType::Semicolon, "Expected ';' after continue statement")?;
        
        Some(AstNode::ContinueStatement {
            location,
        })
    }
    
    fn parse_return_statement(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::Return, "Expected 'return'")?;
        
        let value = if !self.check(&TokenType::Semicolon) && !self.check(&TokenType::Newline) {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };
        
        self.consume(TokenType::Semicolon, "Expected ';' after return statement")?;
        
        Some(AstNode::ReturnStatement {
            value,
            location,
        })
    }
    
    fn parse_expression_statement(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        let expression = Box::new(self.parse_expression()?);
        self.consume(TokenType::Semicolon, "Expected ';' after expression")?;
        
        Some(AstNode::ExpressionStatement {
            expression,
            location,
        })
    }
    
    fn parse_expression(&mut self) -> Option<AstNode> {
        self.parse_assignment()
    }
    
    fn parse_assignment(&mut self) -> Option<AstNode> {
        let expr = self.parse_conditional()?;
        
        if let Some(op) = self.match_assignment_operator() {
            let location = self.current_location();
            let value = Box::new(self.parse_assignment()?);
            return Some(AstNode::Assignment {
                target: Box::new(expr),
                operator: op,
                value,
                location,
            });
        }
        
        Some(expr)
    }
    
    fn parse_conditional(&mut self) -> Option<AstNode> {
        let expr = self.parse_logical_or()?;
        
        // Note: Ternary operator ?: not in the grammar, using if expression instead
        Some(expr)
    }
    
    fn parse_logical_or(&mut self) -> Option<AstNode> {
        let mut expr = self.parse_logical_and()?;
        
        while self.match_token(&TokenType::Or) {
            let location = self.current_location();
            let right = self.parse_logical_and()?;
            expr = AstNode::BinaryOp {
                left: Box::new(expr),
                operator: BinaryOperator::Or,
                right: Box::new(right),
                location,
            };
        }
        
        Some(expr)
    }
    
    fn parse_logical_and(&mut self) -> Option<AstNode> {
        let mut expr = self.parse_bitwise_or()?;
        
        while self.match_token(&TokenType::And) {
            let location = self.current_location();
            let right = self.parse_bitwise_or()?;
            expr = AstNode::BinaryOp {
                left: Box::new(expr),
                operator: BinaryOperator::And,
                right: Box::new(right),
                location,
            };
        }
        
        Some(expr)
    }
    
    fn parse_bitwise_or(&mut self) -> Option<AstNode> {
        let mut expr = self.parse_bitwise_xor()?;
        
        while self.match_token(&TokenType::Pipe) {
            let location = self.current_location();
            let right = self.parse_bitwise_xor()?;
            expr = AstNode::BinaryOp {
                left: Box::new(expr),
                operator: BinaryOperator::BitwiseOr,
                right: Box::new(right),
                location,
            };
        }
        
        Some(expr)
    }
    
    fn parse_bitwise_xor(&mut self) -> Option<AstNode> {
        let mut expr = self.parse_bitwise_and()?;
        
        while self.match_token(&TokenType::Caret) {
            let location = self.current_location();
            let right = self.parse_bitwise_and()?;
            expr = AstNode::BinaryOp {
                left: Box::new(expr),
                operator: BinaryOperator::BitwiseXor,
                right: Box::new(right),
                location,
            };
        }
        
        Some(expr)
    }
    
    fn parse_bitwise_and(&mut self) -> Option<AstNode> {
        let mut expr = self.parse_equality()?;
        
        while self.match_token(&TokenType::Ampersand) {
            let location = self.current_location();
            let right = self.parse_equality()?;
            expr = AstNode::BinaryOp {
                left: Box::new(expr),
                operator: BinaryOperator::BitwiseAnd,
                right: Box::new(right),
                location,
            };
        }
        
        Some(expr)
    }
    
    fn parse_equality(&mut self) -> Option<AstNode> {
        let mut expr = self.parse_relational()?;
        
        while let Some(op) = self.match_equality_operator() {
            let location = self.current_location();
            let right = self.parse_relational()?;
            expr = AstNode::BinaryOp {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
                location,
            };
        }
        
        Some(expr)
    }
    
    fn parse_relational(&mut self) -> Option<AstNode> {
        let mut expr = self.parse_shift()?;
        
        while let Some(op) = self.match_relational_operator() {
            let location = self.current_location();
            let right = self.parse_shift()?;
            expr = AstNode::BinaryOp {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
                location,
            };
        }
        
        Some(expr)
    }
    
    fn parse_shift(&mut self) -> Option<AstNode> {
        let mut expr = self.parse_additive()?;
        
        while let Some(op) = self.match_shift_operator() {
            let location = self.current_location();
            let right = self.parse_additive()?;
            expr = AstNode::BinaryOp {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
                location,
            };
        }
        
        Some(expr)
    }
    
    fn parse_additive(&mut self) -> Option<AstNode> {
        let mut expr = self.parse_multiplicative()?;
        
        while let Some(op) = self.match_additive_operator() {
            let location = self.current_location();
            let right = self.parse_multiplicative()?;
            expr = AstNode::BinaryOp {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
                location,
            };
        }
        
        Some(expr)
    }
    
    fn parse_multiplicative(&mut self) -> Option<AstNode> {
        let mut expr = self.parse_exponentiation()?;
        
        while let Some(op) = self.match_multiplicative_operator() {
            let location = self.current_location();
            let right = self.parse_exponentiation()?;
            expr = AstNode::BinaryOp {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
                location,
            };
        }
        
        Some(expr)
    }
    
    fn parse_exponentiation(&mut self) -> Option<AstNode> {
        let expr = self.parse_unary()?;
        
        if self.match_token(&TokenType::StarStar) {
            let location = self.current_location();
            let right = self.parse_exponentiation()?; // Right associative
            return Some(AstNode::BinaryOp {
                left: Box::new(expr),
                operator: BinaryOperator::Power,
                right: Box::new(right),
                location,
            });
        }
        
        Some(expr)
    }
    
    fn parse_unary(&mut self) -> Option<AstNode> {
        if let Some(op) = self.match_unary_operator() {
            let location = self.current_location();
            let operand = Box::new(self.parse_unary()?);
            return Some(AstNode::UnaryOp {
                operator: op,
                operand,
                location,
            });
        }
        
        self.parse_postfix()
    }
    
    fn parse_postfix(&mut self) -> Option<AstNode> {
        let mut expr = self.parse_primary()?;
        
        loop {
            match &self.peek().token_type {
                TokenType::Dot => {
                    self.advance();
                    let location = self.current_location();
                    let property = self.consume_identifier("Expected property name")?;
                    expr = AstNode::PropertyAccess {
                        object: Box::new(expr),
                        property,
                        location,
                    };
                }
                TokenType::LeftBracket => {
                    self.advance();
                    let location = self.current_location();
                    let index = Box::new(self.parse_expression()?);
                    self.consume(TokenType::RightBracket, "Expected ']'")?;
                    expr = AstNode::IndexAccess {
                        object: Box::new(expr),
                        index,
                        location,
                    };
                }
                TokenType::LeftParen => {
                    self.advance();
                    let location = self.current_location();
                    let arguments = self.parse_argument_list();
                    self.consume(TokenType::RightParen, "Expected ')'")?;
                    expr = AstNode::FunctionCall {
                        function: Box::new(expr),
                        arguments,
                        location,
                    };
                }
                _ => break,
            }
        }
        
        Some(expr)
    }
    
    fn parse_argument_list(&mut self) -> Vec<Argument> {
        let mut arguments = Vec::new();
        
        if self.check(&TokenType::RightParen) {
            return arguments;
        }
        
        loop {
            let mut spread = false;
            let mut keyword_spread = false;
            
            if self.match_token(&TokenType::Star) {
                spread = true;
            } else if self.match_token(&TokenType::StarStar) {
                keyword_spread = true;
            }
            
            if let Some(value) = self.parse_expression() {
                arguments.push(Argument {
                    value,
                    spread,
                    keyword_spread,
                });
            }
            
            if !self.match_token(&TokenType::Comma) {
                break;
            }
            
            if self.check(&TokenType::RightParen) {
                break;
            }
        }
        
        arguments
    }
    
    fn parse_primary(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        
        match &self.peek().token_type {
            TokenType::IntegerLiteral | TokenType::FloatLiteral | 
            TokenType::StringLiteral | TokenType::BooleanLiteral | 
            TokenType::NullLiteral => {
                let token = self.advance();
                token.value.map(|value| AstNode::Literal(value, location))
            }
            TokenType::Identifier => {
                let name = self.advance().lexeme;
                Some(AstNode::Identifier(name, location))
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(TokenType::RightParen, "Expected ')'")?;
                Some(expr)
            }
            TokenType::LeftBracket => {
                self.parse_list_literal()
            }
            TokenType::LeftBrace => {
                self.parse_dict_literal()
            }
            TokenType::If => {
                self.parse_if_expression()
            }
            TokenType::Match => {
                self.parse_match_expression()
            }
            TokenType::Try => {
                self.parse_try_expression()
            }
            TokenType::Fn => {
                self.parse_anonymous_function()
            }
            _ => {
                self.error("Unexpected token in expression");
                None
            }
        }
    }
    
    fn parse_list_literal(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::LeftBracket, "Expected '['")?;
        
        let mut elements = Vec::new();
        
        while !self.check(&TokenType::RightBracket) && !self.is_at_end() {
            if let Some(expr) = self.parse_expression() {
                elements.push(expr);
            }
            
            if !self.match_token(&TokenType::Comma) {
                break;
            }
            
            if self.check(&TokenType::RightBracket) {
                break;
            }
        }
        
        self.consume(TokenType::RightBracket, "Expected ']'")?;
        
        Some(AstNode::ListLiteral {
            elements,
            location,
        })
    }
    
    fn parse_dict_literal(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::LeftBrace, "Expected '{'")?;
        
        let mut entries = Vec::new();
        
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            let key = self.parse_expression()?;
            self.consume(TokenType::Colon, "Expected ':' after dict key")?;
            let value = self.parse_expression()?;
            
            entries.push((key, value));
            
            if !self.match_token(&TokenType::Comma) {
                break;
            }
            
            if self.check(&TokenType::RightBrace) {
                break;
            }
        }
        
        self.consume(TokenType::RightBrace, "Expected '}'")?;
        
        Some(AstNode::DictLiteral {
            entries,
            location,
        })
    }
    
    fn parse_if_expression(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::If, "Expected 'if'")?;
        
        let condition = Box::new(self.parse_expression()?);
        let then_branch = Box::new(self.parse_block()?);
        
        let mut else_ifs = Vec::new();
        
        while self.match_token(&TokenType::Else) && self.check(&TokenType::If) {
            self.advance(); // consume 'if'
            let else_if_condition = self.parse_expression()?;
            let else_if_body = self.parse_block()?;
            else_ifs.push((else_if_condition, else_if_body));
        }
        
        let else_branch = if self.match_token(&TokenType::Else) {
            Some(Box::new(self.parse_block()?))
        } else {
            None
        };
        
        Some(AstNode::If {
            condition,
            then_branch,
            else_ifs,
            else_branch,
            location,
        })
    }
    
    fn parse_match_expression(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::Match, "Expected 'match'")?;
        
        let value = Box::new(self.parse_expression()?);
        
        self.consume(TokenType::LeftBrace, "Expected '{'")?;
        
        let mut arms = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if let Some(arm) = self.parse_match_arm() {
                arms.push(arm);
            }
            self.skip_newlines();
        }
        
        self.consume(TokenType::RightBrace, "Expected '}'")?;
        
        Some(AstNode::Match {
            value,
            arms,
            location,
        })
    }
    
    fn parse_try_expression(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::Try, "Expected 'try'")?;
        
        let body = Box::new(self.parse_block()?);
        
        let catch_clause = if self.match_token(&TokenType::Catch) {
            let variable = self.consume_identifier("Expected catch variable")?;
            let catch_body = self.parse_block()?;
            Some(CatchClause {
                variable,
                body: catch_body,
            })
        } else {
            None
        };
        
        Some(AstNode::Try {
            body,
            catch_clause,
            location,
        })
    }
    
    fn parse_anonymous_function(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::Fn, "Expected 'fn'")?;
        
        self.consume(TokenType::LeftParen, "Expected '(' after 'fn'")?;
        let parameters = self.parse_parameter_list();
        self.consume(TokenType::RightParen, "Expected ')'")?;
        
        let body = Box::new(self.parse_block()?);
        
        Some(AstNode::AnonymousFunction {
            parameters,
            body,
            location,
        })
    }
    
    // Helper methods for matching operators
    
    fn match_assignment_operator(&mut self) -> Option<AssignmentOperator> {
        match &self.peek().token_type {
            TokenType::Equal => { self.advance(); Some(AssignmentOperator::Assign) }
            TokenType::PlusEqual => { self.advance(); Some(AssignmentOperator::AddAssign) }
            TokenType::MinusEqual => { self.advance(); Some(AssignmentOperator::SubtractAssign) }
            TokenType::StarEqual => { self.advance(); Some(AssignmentOperator::MultiplyAssign) }
            TokenType::SlashEqual => { self.advance(); Some(AssignmentOperator::DivideAssign) }
            TokenType::PercentEqual => { self.advance(); Some(AssignmentOperator::ModuloAssign) }
            _ => None,
        }
    }
    
    fn match_equality_operator(&mut self) -> Option<BinaryOperator> {
        match &self.peek().token_type {
            TokenType::EqualEqual => { self.advance(); Some(BinaryOperator::Equal) }
            TokenType::BangEqual => { self.advance(); Some(BinaryOperator::NotEqual) }
            _ => None,
        }
    }
    
    fn match_relational_operator(&mut self) -> Option<BinaryOperator> {
        match &self.peek().token_type {
            TokenType::Less => { self.advance(); Some(BinaryOperator::Less) }
            TokenType::Greater => { self.advance(); Some(BinaryOperator::Greater) }
            TokenType::LessEqual => { self.advance(); Some(BinaryOperator::LessEqual) }
            TokenType::GreaterEqual => { self.advance(); Some(BinaryOperator::GreaterEqual) }
            TokenType::In => { self.advance(); Some(BinaryOperator::In) }
            TokenType::Is => { self.advance(); Some(BinaryOperator::Is) }
            _ => None,
        }
    }
    
    fn match_shift_operator(&mut self) -> Option<BinaryOperator> {
        match &self.peek().token_type {
            TokenType::LessLess => { self.advance(); Some(BinaryOperator::LeftShift) }
            TokenType::GreaterGreater => { self.advance(); Some(BinaryOperator::RightShift) }
            _ => None,
        }
    }
    
    fn match_additive_operator(&mut self) -> Option<BinaryOperator> {
        match &self.peek().token_type {
            TokenType::Plus => { self.advance(); Some(BinaryOperator::Add) }
            TokenType::Minus => { self.advance(); Some(BinaryOperator::Subtract) }
            _ => None,
        }
    }
    
    fn match_multiplicative_operator(&mut self) -> Option<BinaryOperator> {
        match &self.peek().token_type {
            TokenType::Star => { self.advance(); Some(BinaryOperator::Multiply) }
            TokenType::Slash => { self.advance(); Some(BinaryOperator::Divide) }
            TokenType::Percent => { self.advance(); Some(BinaryOperator::Modulo) }
            _ => None,
        }
    }
    
    fn match_unary_operator(&mut self) -> Option<UnaryOperator> {
        match &self.peek().token_type {
            TokenType::Plus => { self.advance(); Some(UnaryOperator::Plus) }
            TokenType::Minus => { self.advance(); Some(UnaryOperator::Minus) }
            TokenType::Not => { self.advance(); Some(UnaryOperator::Not) }
            TokenType::Tilde => { self.advance(); Some(UnaryOperator::BitwiseNot) }
            _ => None,
        }
    }
    
    // Utility methods
    
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous().clone()
    }
    
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().token_type == token_type
        }
    }
    
    fn match_token(&mut self, token_type: &TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    fn match_string_literal(&mut self) -> Option<String> {
        if self.check(&TokenType::StringLiteral) {
            let token = self.advance();
            if let Some(LiteralValue::String(s)) = token.value {
                Some(s)
            } else {
                None
            }
        } else {
            None
        }
    }
    
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
    
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }
    
    fn consume(&mut self, token_type: TokenType, message: &str) -> Option<()> {
        if self.check(&token_type) {
            self.advance();
            Some(())
        } else {
            self.error(message);
            None
        }
    }
    
    fn consume_identifier(&mut self, message: &str) -> Option<String> {
        if self.check(&TokenType::Identifier) {
            Some(self.advance().lexeme)
        } else {
            self.error(message);
            None
        }
    }
    
    fn skip_newlines(&mut self) {
        while self.match_token(&TokenType::Newline) {
            // Skip newlines
        }
    }
    
    fn current_location(&self) -> SourceLocation {
        SourceLocation::from_token(self.peek())
    }
    
    fn error(&mut self, message: &str) {
        let location = self.current_location();
        self.errors.push(ParseError {
            message: message.to_string(),
            location,
        });
    }
    
    pub fn errors(&self) -> &[ParseError] {
        &self.errors
    }
}