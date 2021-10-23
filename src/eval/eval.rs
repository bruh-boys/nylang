use super::super::ast;
use super::super::enve;
use super::super::object;
use std::cell::RefCell;

// actual program runner
pub struct Evaluator {
    pub env: RefCell<Box<enve::enve::Environment>>,
}

impl Evaluator {
    ////////////////////////////////////////////////////////////////////////////////
    ///
    /// // constructors
    ///
    ////////////////////////////////////////////////////////////////////////////////
    pub fn new() -> Evaluator {
        Evaluator {
            env: RefCell::new(Box::new(enve::enve::Environment::new())),
        }
    }

    pub fn from(env: enve::enve::Environment) -> Evaluator {
        Evaluator {
            env: RefCell::new(Box::new(env)),
        }
    }

    ////////////////////////////////////////////////////////////////////////////////
    ///
    /// // environment setter / getter
    ///
    ////////////////////////////////////////////////////////////////////////////////
    pub fn get_env(&self, key: &str) -> Option<object::object::Object> {
        self.env.borrow_mut().get(key)
    }

    pub fn set_env(&mut self, key: String, value: object::object::Object) {
        self.env.borrow_mut().set(key, value);
    }

    ////////////////////////////////////////////////////////////////////////////////
    ///
    /// // evaluator
    ///
    ////////////////////////////////////////////////////////////////////////////////
    pub fn evaluator(&mut self, program: ast::ast::Program) -> object::object::Object {
        let mut evaluated = object::object::Object::Null;

        // loop through the program
        for statement in program.statements.iter() {
            evaluated = self.statement_evaluator(statement.clone());

            if let object::object::Object::ReturnValue(value) = evaluated {
                return *value;
            }
        }
        evaluated
    }

    pub fn prefix_evaluator(&mut self, operator: ast::ast::Prefix, right: object::object::Object) -> object::object::Object {
        match (operator, right) {
            (ast::ast::Prefix::Bang, object::object::Object::Boolean(b_value)) => object::object::Object::Boolean(!b_value),
            (ast::ast::Prefix::Minus, object::object::Object::Integer(b_value)) => object::object::Object::Integer(-1 * b_value),
            _ => object::object::Object::Null
        }
    }

    pub fn infix_evaluator(&mut self, operator: ast::ast::Infix, left: object::object::Object, right: object::object::Object) -> object::object::Object {
        match (operator, left, right) {
            (ast::ast::Infix::Plus, object::object::Object::Integer(l_value), object::object::Object::Integer(r_value)) => object::object::Object::Integer(l_value + r_value),
            (ast::ast::Infix::Minus, object::object::Object::Integer(l_value), object::object::Object::Integer(r_value)) => object::object::Object::Integer(l_value - r_value),
            (ast::ast::Infix::Slash, object::object::Object::Integer(l_value), object::object::Object::Integer(r_value)) => object::object::Object::Integer(l_value / r_value),
            (ast::ast::Infix::Asterisk, object::object::Object::Integer(l_value), object::object::Object::Integer(r_value)) => object::object::Object::Integer(l_value * r_value),
            (ast::ast::Infix::Eq, object::object::Object::Integer(l_value), object::object::Object::Integer(r_value)) => object::object::Object::Boolean(l_value == r_value),
            (ast::ast::Infix::NotEq, object::object::Object::Integer(l_value), object::object::Object::Integer(r_value)) => object::object::Object::Boolean(l_value != r_value),
            (ast::ast::Infix::GreaterThan, object::object::Object::Integer(l_value), object::object::Object::Integer(r_value)) => object::object::Object::Boolean(l_value > r_value),
            (ast::ast::Infix::LessThan, object::object::Object::Integer(l_value), object::object::Object::Integer(r_value)) => object::object::Object::Boolean(l_value < r_value),
            (ast::ast::Infix::Eq, object::object::Object::Boolean(l_value), object::object::Object::Boolean(r_value)) => object::object::Object::Boolean(l_value == r_value),
            (ast::ast::Infix::NotEq, object::object::Object::Boolean(l_value), object::object::Object::Boolean(r_value)) => object::object::Object::Boolean(l_value != r_value),
            (ast::ast::Infix::Plus, object::object::Object::String(l_value), object::object::Object::String(r_value)) => object::object::Object::String(format!("{}{}", l_value, r_value)),
            (ast::ast::Infix::Eq, object::object::Object::String(l_value), object::object::Object::String(r_value)) => object::object::Object::Boolean(l_value == r_value),
            (ast::ast::Infix::NotEq, object::object::Object::String(l_value), object::object::Object::String(r_value)) => object::object::Object::Boolean(l_value != r_value),
            (ast::ast::Infix::Eq, object::object::Object::Null, object::object::Object::Null) => object::object::Object::Boolean(true),
            (ast::ast::Infix::NotEq, object::object::Object::Null, object::object::Object::Null) => object::object::Object::Boolean(false),
            _ => object::object::Object::Null
        }
    }

    pub fn block_evaluator(&mut self, statements: Vec<ast::ast::Statement>) -> object::object::Object {
        let mut result = object::object::Object::Null;
        for statement in statements.iter() {
            result = self.statement_evaluator(statement.clone());
            if let object::object::Object::ReturnValue(_) = result {
                return result
            }
        }
        result
    }

    pub fn statement_evaluator(
        &mut self,
        statement: ast::ast::Statement,
    ) -> object::object::Object {
        return match statement {
            ast::ast::Statement::Let { identifier, value } => {
                // let indentifier to stringify!
                if let ast::ast::Expression::Ident(stringified_identifier) = identifier {

                    let env_vle = self.expression_evaluator(value);

                    self.set_env(
                        stringified_identifier.clone(),
                        env_vle,
                    );
                    return object::object::Object::Null;
                } else {
                    panic!("not implemented");
                }
            }
            ast::ast::Statement::Return(value) => {
                object::object::Object::ReturnValue(Box::new(self.expression_evaluator(value)))
            }
            ast::ast::Statement::Expression(expression) => self.expression_evaluator(expression),
            ast::ast::Statement::Block(statements) => self.block_evaluator(statements),
        };
    }

    pub fn expression_evaluator(
        &mut self,
        expression: ast::ast::Expression,
    ) -> object::object::Object {
        return match expression {
            ast::ast::Expression::Ident(identifier) => {
                match self.get_env(identifier.as_str()) {
                    Some(value) => value,
                    None => panic!("identifier not found"),
                }
            }
            ast::ast::Expression::Integer(integer) => object::object::Object::Integer(integer),
            ast::ast::Expression::Bool(boolean) => object::object::Object::Boolean(boolean),
            ast::ast::Expression::String(string) => object::object::Object::String(string),
            ast::ast::Expression::Prefix { op, right } => {
                let right = self.expression_evaluator(*right);
                self.prefix_evaluator(op, right)
            }
            ast::ast::Expression::Infix { op, left, right } => {

                let left = self.expression_evaluator(*left);
                let right = self.expression_evaluator(*right);

                return self.infix_evaluator(
                op,
                left,
                right,
            )
        }
            ,
            ast::ast::Expression::If {
                condition,
                consequence,
                alternative,
                ..
            } => {
                if self.expression_evaluator(*condition).bool_check() {
                    self.statement_evaluator(*consequence)
                } else {
                    match alternative {
                        Some(alt) => self.statement_evaluator(*alt),
                        None => object::object::Object::Null,
                    }
                }
            }
            ast::ast::Expression::Function { parameters, body } => {
                object::object::Object::Function {
                    parameters: parameters.clone(),
                    body: *body.clone(),
                    env: enve::enve::Environment::ve(*self.env.clone().into_inner()),
                }
            }
            ast::ast::Expression::Call {
                function,
                arguments,
            } => {
                let mut args = Vec::new();
                for expr in arguments.iter() {
                    args.push(self.expression_evaluator(expr.clone()));
                }

                if let ast::ast::Expression::Ident(func) = *function.clone() {
                    if func == "bark" {
                        println!("{}", args.iter().map(|arg| format!("{} ", arg)).collect::<String>());
                        return object::object::Object::Null
                    }
                }

                let function = self.expression_evaluator(*function);

                if let object::object::Object::Function {
                    parameters,
                    body,
                    env,
                } = function
                {
                    let mut env = Evaluator::from(env);
                    for (ident, arg) in parameters.iter().zip(args.iter()) {
                        if let ast::ast::Expression::Ident(ident) = ident {
                            env.set_env(ident.to_owned(), arg.clone());
                        }
                    }
                    match env.statement_evaluator(body) {
                        object::object::Object::ReturnValue(obj) => *obj,
                        obj => obj,
                    }
                } else {
                    panic!("not implemented");
                }
            }
        };
    }
}