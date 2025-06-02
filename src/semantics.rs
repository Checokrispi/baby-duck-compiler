use std::collections::HashMap;
use crate::ast::Expression;
use crate::ast::Operator;
use crate::ast::Program;
 use crate::ast::Statement;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum OperatorType {
    Add,
    Sub,
    Mul,
    Div,
    NotEqual,
    GreaterThan,
    LessThan,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ExpressionType {
    Int,
    Float,
    Bool,
    Error,
    String,
}

pub struct SemanticCube {
    cube: HashMap<(OperatorType, ExpressionType, ExpressionType), ExpressionType>,
}

impl SemanticCube {
    pub fn new() -> Self {
        let mut cube = HashMap::new();
        
        //operaciones aritméticas
        cube.insert((OperatorType::Add, ExpressionType::Int, ExpressionType::Int), ExpressionType::Int);
        cube.insert((OperatorType::Add, ExpressionType::Int, ExpressionType::Float), ExpressionType::Float);
        cube.insert((OperatorType::Add, ExpressionType::Float, ExpressionType::Float), ExpressionType::Float);

        cube.insert((OperatorType::Sub, ExpressionType::Int, ExpressionType::Int), ExpressionType::Int);
        cube.insert((OperatorType::Sub, ExpressionType::Int, ExpressionType::Float), ExpressionType::Float);
        cube.insert((OperatorType::Sub, ExpressionType::Float, ExpressionType::Float), ExpressionType::Float);

        cube.insert((OperatorType::Mul, ExpressionType::Int, ExpressionType::Int), ExpressionType::Int);
        cube.insert((OperatorType::Mul, ExpressionType::Int, ExpressionType::Float), ExpressionType::Float);
        cube.insert((OperatorType::Mul, ExpressionType::Float, ExpressionType::Float), ExpressionType::Float);

        cube.insert((OperatorType::Div, ExpressionType::Int, ExpressionType::Int), ExpressionType::Int);
        cube.insert((OperatorType::Div, ExpressionType::Int, ExpressionType::Float), ExpressionType::Float);
        cube.insert((OperatorType::Div, ExpressionType::Float, ExpressionType::Float), ExpressionType::Float);
        
        
        //operaciones de comparación
        cube.insert((OperatorType::NotEqual, ExpressionType::Int, ExpressionType::Int), ExpressionType::Bool);
        cube.insert((OperatorType::NotEqual, ExpressionType::Int, ExpressionType::Float), ExpressionType::Bool);
        cube.insert((OperatorType::NotEqual, ExpressionType::Float, ExpressionType::Float), ExpressionType::Bool);
        
        cube.insert((OperatorType::GreaterThan, ExpressionType::Int, ExpressionType::Int), ExpressionType::Bool);
        cube.insert((OperatorType::GreaterThan, ExpressionType::Int, ExpressionType::Float), ExpressionType::Bool);
        cube.insert((OperatorType::GreaterThan, ExpressionType::Float, ExpressionType::Float), ExpressionType::Bool);

        cube.insert((OperatorType::LessThan, ExpressionType::Int, ExpressionType::Int), ExpressionType::Bool);
        cube.insert((OperatorType::LessThan, ExpressionType::Int, ExpressionType::Float), ExpressionType::Bool);
        cube.insert((OperatorType::LessThan, ExpressionType::Float, ExpressionType::Float), ExpressionType::Bool);
        
        //operaciones no válidas
        cube.insert((OperatorType::Div, ExpressionType::Int, ExpressionType::String), ExpressionType::Error);
        cube.insert((OperatorType::Div, ExpressionType::Float, ExpressionType::String), ExpressionType::Error);
        cube.insert((OperatorType::Div, ExpressionType::Float, ExpressionType::Bool), ExpressionType::Error);
        cube.insert((OperatorType::Div, ExpressionType::Int, ExpressionType::Bool), ExpressionType::Error);

        cube.insert((OperatorType::Mul, ExpressionType::Int, ExpressionType::String), ExpressionType::Error);
        cube.insert((OperatorType::Mul, ExpressionType::Float, ExpressionType::String), ExpressionType::Error);
        cube.insert((OperatorType::Mul, ExpressionType::Int, ExpressionType::Bool), ExpressionType::Error);
        cube.insert((OperatorType::Mul, ExpressionType::Float, ExpressionType::Bool), ExpressionType::Error);

        cube.insert((OperatorType::Add, ExpressionType::Int, ExpressionType::String), ExpressionType::Error);
        cube.insert((OperatorType::Add, ExpressionType::Float, ExpressionType::String), ExpressionType::Error);
        cube.insert((OperatorType::Add, ExpressionType::Float, ExpressionType::Bool), ExpressionType::Error);
        cube.insert((OperatorType::Add, ExpressionType::Int, ExpressionType::Bool), ExpressionType::Error);

        cube.insert((OperatorType::Sub, ExpressionType::Int, ExpressionType::String), ExpressionType::Error);
        cube.insert((OperatorType::Sub, ExpressionType::Float, ExpressionType::String), ExpressionType::Error);
        cube.insert((OperatorType::Sub, ExpressionType::Int, ExpressionType::Bool), ExpressionType::Error);
        cube.insert((OperatorType::Sub, ExpressionType::Float, ExpressionType::Bool), ExpressionType::Error);

        cube.insert((OperatorType::NotEqual, ExpressionType::Int, ExpressionType::String), ExpressionType::Error);
        cube.insert((OperatorType::NotEqual, ExpressionType::Float, ExpressionType::String), ExpressionType::Error);
        cube.insert((OperatorType::NotEqual, ExpressionType::Int, ExpressionType::Bool), ExpressionType::Error);
        cube.insert((OperatorType::NotEqual, ExpressionType::Float, ExpressionType::Bool), ExpressionType::Error);

        cube.insert((OperatorType::GreaterThan, ExpressionType::Int, ExpressionType::String), ExpressionType::Error);
        cube.insert((OperatorType::GreaterThan, ExpressionType::Float, ExpressionType::String), ExpressionType::Error);
        cube.insert((OperatorType::GreaterThan, ExpressionType::Int, ExpressionType::Bool), ExpressionType::Error);
        cube.insert((OperatorType::GreaterThan, ExpressionType::Float, ExpressionType::Bool), ExpressionType::Error);
        
        cube.insert((OperatorType::LessThan, ExpressionType::Int, ExpressionType::String), ExpressionType::Error);
        cube.insert((OperatorType::Div, ExpressionType::Float, ExpressionType::String), ExpressionType::Error);
        cube.insert((OperatorType::Div, ExpressionType::Int, ExpressionType::Bool), ExpressionType::Error);
        cube.insert((OperatorType::Div, ExpressionType::Float, ExpressionType::Bool), ExpressionType::Error);
        
        SemanticCube { cube }
    }

    pub fn get_result_type(
        &self,
        op: OperatorType,
        left: ExpressionType,
        right: ExpressionType
    ) -> ExpressionType {
        self.cube.get(&(op, left, right)).unwrap_or(&ExpressionType::Error).clone()
    }


    pub fn validate_program(program: &Program, cube: &SemanticCube) {
        for stmt in &program.body {
            match stmt {
                Statement::Assign(assign) => {
                    Self::validate_expression(&assign.value, cube);
                }
                Statement::Print(print_stmt) => {
                    Self::validate_expression(&print_stmt.value, cube);
                }
                Statement::FCall(call) => {
                    for expr in &call.expression {
                        Self::validate_expression(expr, cube);
                    }
                }
                Statement::Condition(cond) => {
                    Self::validate_expression(&cond.condition, cube);
                    if let Some(body) = &cond.body {
                        Self::validate_statement(body, cube);
                    }
                    if let Some(elsebody) = &cond.elsebody {
                        Self::validate_statement(elsebody, cube);
                    }
                }
                Statement::Cycle(cycle) => {
                    Self::validate_expression(&cycle.wcondition, cube);
                    for stmt in &cycle.wbody {
                        Self::validate_statement(stmt, cube);
                    }
                }
            }
        }
    }

    pub fn validate_statement(stmt: &Statement, cube: &SemanticCube) {
        match stmt {
            Statement::Assign(assign) => {
                Self::validate_expression(&assign.value, cube);
            }
            Statement::Print(print_stmt) => {
                Self::validate_expression(&print_stmt.value, cube);
            }
            Statement::FCall(call) => {
                for expr in &call.expression {
                    Self::validate_expression(expr, cube);
                }
            }
            Statement::Condition(cond) => {
                Self::validate_expression(&cond.condition, cube);
                if let Some(body) = &cond.body {
                    Self::validate_statement(body, cube);
                }
                if let Some(elsebody) = &cond.elsebody {
                    Self::validate_statement(elsebody, cube);
                }
            }
            Statement::Cycle(cycle) => {
                Self::validate_expression(&cycle.wcondition, cube);
                for stmt in &cycle.wbody {
                    Self::validate_statement(stmt, cube);
                }
            }
        }
    }

    pub fn validate_expression(expr: &Expression, cube: &SemanticCube) -> ExpressionType {
        match expr {
            Expression::Integer(_) => ExpressionType::Int,
            Expression::Float(_) => ExpressionType::Float,
            Expression::Bool(_) => ExpressionType::Bool,
            Expression::Variable(_) => {
                ExpressionType::String
            }
            Expression::BinaryOperation { lhs, operator, rhs } => {
                let left_type = Self::validate_expression(lhs, cube);
                let right_type = Self::validate_expression(rhs, cube);
                let op_type = match operator {
                    Operator::Add => OperatorType::Add,
                    Operator::Sub => OperatorType::Sub,
                    Operator::Mul => OperatorType::Mul,
                    Operator::Div => OperatorType::Div,
                    Operator::NotEqual => OperatorType::NotEqual,
                    Operator::GreaterThan => OperatorType::GreaterThan,
                    Operator::LessThan => OperatorType::LessThan,
                };

                let result_type = cube.get_result_type(op_type, left_type.clone(), right_type.clone());
                if result_type == ExpressionType::Error {
                    println!(
                        "Semantic Error, invalid operation"
                    );
                } else {
                    println!(
                        "Valid Semantic"
                    );
                }

                result_type
            }
        }
    }



}