use crate::{
    parser::{
        ast::{
            Expression,
            Type
        }
    },
    codegen::{
        compiler::Compiler
    }
};

#[derive(Debug)]
pub enum CheckerError {
    Unknown,
    TypeMismatch
}

pub type CheckerResult<T> = Result<T, CheckerError>;

pub struct Checker<'c> {
    compiler: &'c Compiler
}

impl<'c> Checker<'c> {
    pub fn new(compiler: &'c Compiler) -> Checker<'c> {
        Checker {
            compiler: compiler
        }
    }

    pub fn check_expr_type(&self, expr: &Expression) -> CheckerResult<Type> {
        Ok(match expr {
            Expression::IntLiteral(_) => Type::Int,
            Expression::FloatLiteral(_) => Type::Float,
            Expression::StringLiteral(_) => Type::String,
            Expression::BoolLiteral(_) => Type::Bool,
            Expression::Variable(name) => {
                self.compiler.type_of_var(name.clone())
                    .map_err(|_| CheckerError::TypeMismatch)?
            }
            Expression::Addition(lhs, rhs) => {
                let lhs_type = self.check_expr_type(lhs)?;
                let rhs_type = self.check_expr_type(rhs)?;
                if lhs_type != rhs_type {
                    return Err(CheckerError::TypeMismatch);
                }
                lhs_type
            },
            Expression::Multiplication(lhs, rhs) => {
                let lhs_type = self.check_expr_type(lhs)?;
                let rhs_type = self.check_expr_type(rhs)?;
                if lhs_type != rhs_type {
                    return Err(CheckerError::TypeMismatch);
                }
                lhs_type
            },
            Expression::Subtraction(lhs, rhs) => {
                let lhs_type = self.check_expr_type(lhs)?;
                let rhs_type = self.check_expr_type(rhs)?;
                if lhs_type != rhs_type {
                    return Err(CheckerError::TypeMismatch);
                }
                lhs_type
            },
            Expression::Division(lhs, rhs) => {
                let lhs_type = self.check_expr_type(lhs)?;
                let rhs_type = self.check_expr_type(rhs)?;
                if lhs_type != rhs_type {
                    return Err(CheckerError::TypeMismatch);
                }
                lhs_type
            },
            Expression::Not(expr) => {
                self.check_expr_type(expr)?
            },
            Expression::Equals(lhs, rhs) => {
                let lhs_type = self.check_expr_type(lhs)?;
                let rhs_type = self.check_expr_type(rhs)?;
                if lhs_type != rhs_type {
                    return Err(CheckerError::TypeMismatch);
                }
                lhs_type
            },
            Expression::NotEquals(lhs, rhs) => {
                let lhs_type = self.check_expr_type(lhs)?;
                let rhs_type = self.check_expr_type(rhs)?;
                if lhs_type != rhs_type {
                    return Err(CheckerError::TypeMismatch);
                }
                lhs_type
            },
            Expression::GreaterThan(lhs, rhs) => {
                let lhs_type = self.check_expr_type(lhs)?;
                let rhs_type = self.check_expr_type(rhs)?;
                if lhs_type != rhs_type {
                    return Err(CheckerError::TypeMismatch);
                }
                lhs_type
            },
            Expression::LessThan(lhs, rhs) => {
                let lhs_type = self.check_expr_type(lhs)?;
                let rhs_type = self.check_expr_type(rhs)?;
                if lhs_type != rhs_type {
                    return Err(CheckerError::TypeMismatch);
                }
                lhs_type
            },
            Expression::GreaterThanEquals(lhs, rhs) => {
                let lhs_type = self.check_expr_type(lhs)?;
                let rhs_type = self.check_expr_type(rhs)?;
                if lhs_type != rhs_type {
                    return Err(CheckerError::TypeMismatch);
                }
                lhs_type
            },
            Expression::LessThanEquals(lhs, rhs) => {
                let lhs_type = self.check_expr_type(lhs)?;
                let rhs_type = self.check_expr_type(rhs)?;
                if lhs_type != rhs_type {
                    return Err(CheckerError::TypeMismatch);
                }
                lhs_type
            },
        })
    } 
}