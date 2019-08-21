//! Bytecode generator for Rusty Python
//!
//! This module is responsible for generating bytecode from AST nodes.

use oak_python::ast::{
    AugmentedOperator, BinaryOperator, BoolOperator, CompareOperator as ComparisonOperator, Comprehension, ExceptHandler, Expression, ImportName, Keyword,
    Literal, Parameter, Statement, Type, UnaryOperator, WithItem,
};
use python_types::PythonResult;
use smallvec::SmallVec;

/// Bytecode instructions
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Instruction {
    // Stack manipulation
    PushNull,
    PushTrue,
    PushFalse,
    PushInteger(i64),
    PushFloat(f64),
    PushString(String),
    PushName(String),
    Pop,
    Dup,
    Rot2,
    Rot3,

    // Arithmetic operations
    Add,
    Subtract,
    Multiply,
    Divide,
    FloorDivide,
    Modulo,
    Power,

    // Bitwise operations
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
    BitwiseNot,

    // Comparison operations
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,

    // Logical operations
    And,
    Or,
    Not,

    // Control flow
    Jump(usize),
    JumpIfTrue(usize),
    JumpIfFalse(usize),
    Return,
    Yield,
    YieldFrom,

    // Function operations
    Call(usize),         // number of arguments
    MakeFunction(usize), // number of parameters
    LoadMethod(String),

    // Attribute operations
    GetAttribute(String),
    SetAttribute(String),

    // Subscript operations
    GetItem,
    SetItem,

    // Object operations
    BuildTuple(usize),
    BuildList(usize),
    BuildDict(usize),
    BuildSet(usize),

    // Exception handling
    SetupExcept(usize),  // jump target
    SetupFinally(usize), // jump target
    EndFinally,
    Raise,

    // Loop operations
    SetupLoop(usize), // jump target
    Break,
    Continue,

    // Context management
    SetupWith(usize), // jump target
}

/// Bytecode function
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BytecodeFunction {
    pub name: String,
    pub instructions: SmallVec<[Instruction; 64]>,
    pub constants: Vec<Literal>,
    pub names: Vec<String>,
    pub varnames: Vec<String>,
    pub argcount: usize,
    pub kwonlyargcount: usize,
    pub nlocals: usize,
    pub stacksize: usize,
}

/// Bytecode generator
pub struct CodeGenerator {
    instructions: SmallVec<[Instruction; 64]>,
    constants: Vec<Literal>,
    names: Vec<String>,
    varnames: Vec<String>,
    current_function: Option<String>,
}

impl CodeGenerator {
    /// Create a new code generator
    pub fn new() -> Self {
        Self { instructions: smallvec![], constants: vec![], names: vec![], varnames: vec![], current_function: None }
    }

    /// Generate bytecode from a list of statements
    pub fn generate(&mut self, statements: Vec<Statement>) -> PythonResult<BytecodeFunction> {
        for stmt in statements {
            self.generate_statement(stmt)?;
        }

        // Add return None at the end
        self.instructions.push(Instruction::PushNull);
        self.instructions.push(Instruction::Return);

        Ok(BytecodeFunction {
            name: self.current_function.clone().unwrap_or_else(|| "<module>".to_string()),
            instructions: self.instructions.clone(),
            constants: self.constants.clone(),
            names: self.names.clone(),
            varnames: self.varnames.clone(),
            argcount: 0,
            kwonlyargcount: 0,
            nlocals: self.varnames.len(),
            stacksize: 0, // TODO: calculate stack size
        })
    }

    /// Generate bytecode for a statement
    fn generate_statement(&mut self, stmt: Statement) -> PythonResult<()> {
        match stmt {
            Statement::FunctionDef { name, parameters, return_type, body, decorators } => {
                self.generate_function_def(name, parameters, return_type, body, decorators)?;
            }
            Statement::ClassDef { name, bases, body, decorators } => {
                self.generate_class_def(name, bases, body, decorators)?;
            }
            Statement::If { test, body, orelse } => {
                self.generate_if(Box::new(test), body, orelse)?;
            }
            Statement::For { target, iter, body, orelse } => {
                self.generate_for(target, iter, body, orelse)?;
            }
            Statement::While { test, body, orelse } => {
                self.generate_while(Box::new(test), body, orelse)?;
            }
            Statement::Try { body, handlers, orelse, finalbody } => {
                self.generate_try(body, handlers, orelse, finalbody)?;
            }
            Statement::With { items, body } => {
                self.generate_with(items, body)?;
            }
            Statement::Return(value) => {
                self.generate_return(value)?;
            }
            Statement::Break => {
                self.instructions.push(Instruction::Break);
            }
            Statement::Continue => {
                self.instructions.push(Instruction::Continue);
            }
            Statement::Pass => {
                // No operation
            }
            Statement::Raise { exc, cause } => {
                self.generate_raise(exc, cause)?;
            }
            Statement::Assert { test, msg } => {
                self.generate_assert(test, msg)?;
            }
            Statement::Import { names } => {
                let names = names.into_iter().map(|name| (name.name, name.asname)).collect();
                self.generate_import(names)?;
            }
            Statement::ImportFrom { module, names } => {
                let names = names.into_iter().map(|name| (name.name, name.asname)).collect();
                self.generate_import_from(module, names, 0)?;
            }
            Statement::Global { names } => {
                self.generate_global(names)?;
            }
            Statement::Nonlocal { names } => {
                self.generate_nonlocal(names)?;
            }
            Statement::Assignment { target, value } => {
                self.generate_assign(target, value)?;
            }
            Statement::AugmentedAssignment { target, operator, value } => {
                self.generate_aug_assign(target, operator, value)?;
            }
            Statement::Delete { targets } => {
                self.generate_del(targets)?;
            }
            Statement::Expression(value) => {
                self.generate_expression(value)?;
                self.instructions.push(Instruction::Pop);
            }
            _ => todo!("Implement other statement types"),
        }
        Ok(())
    }

    /// Generate bytecode for an expression
    fn generate_expression(&mut self, expr: Expression) -> PythonResult<()> {
        match expr {
            Expression::Literal(literal) => {
                self.generate_literal(literal);
            }
            Expression::Name(name) => {
                self.instructions.push(Instruction::PushName(name));
            }
            Expression::Attribute { value, attr } => {
                self.generate_expression(*value)?;
                self.instructions.push(Instruction::GetAttribute(attr));
            }
            Expression::Subscript { value, slice } => {
                self.generate_expression(*value)?;
                self.generate_expression(*slice)?;
                self.instructions.push(Instruction::GetItem);
            }
            Expression::Call { func, args, keywords } => {
                for arg in &args {
                    self.generate_expression(arg.clone())?;
                }
                for kw in &keywords {
                    self.generate_expression(kw.value.clone())?;
                }
                self.generate_expression(*func)?;
                self.instructions.push(Instruction::Call(args.len() + keywords.len()));
            }
            Expression::BinaryOp { left, operator, right } => {
                self.generate_expression(*left)?;
                self.generate_expression(*right)?;
                self.generate_binary_op(operator);
            }
            Expression::UnaryOp { operator, operand } => {
                self.generate_expression(*operand)?;
                self.generate_unary_op(operator);
            }
            Expression::Lambda { args, body } => {
                self.generate_lambda(args, *body)?;
            }
            Expression::ListComp { elt, generators } => {
                self.generate_list_comp(elt, generators.clone())?;
            }
            Expression::DictComp { key, value, generators } => {
                self.generate_dict_comp(*key, *value, generators.clone())?;
            }
            Expression::SetComp { elt, generators } => {
                self.generate_set_comp(elt, generators.clone())?;
            }
            Expression::GeneratorExp { elt, generators } => {
                self.generate_generator_exp(elt, generators.clone())?;
            }

            Expression::Tuple { elts } => {
                let len = elts.len();
                for expr in elts {
                    self.generate_expression(expr.clone())?;
                }
                self.instructions.push(Instruction::BuildTuple(len));
            }
            Expression::List { elts } => {
                let len = elts.len();
                for expr in elts {
                    self.generate_expression(expr.clone())?;
                }
                self.instructions.push(Instruction::BuildList(len));
            }
            Expression::Dict { keys, values } => {
                let len = values.len();
                for (key, value) in keys.iter().zip(values.iter()) {
                    if let Some(key) = key {
                        self.generate_expression(key.clone())?;
                    }
                    self.generate_expression(value.clone())?;
                }
                self.instructions.push(Instruction::BuildDict(len));
            }
            Expression::Set { elts } => {
                let len = elts.len();
                for expr in elts {
                    self.generate_expression(expr.clone())?;
                }
                self.instructions.push(Instruction::BuildSet(len));
            }
            Expression::Starred { value, is_double: _ } => {
                self.generate_expression(*value)?;
                // TODO: Handle is_double
            }
            Expression::Await(value) => {
                self.generate_expression(*value)?;
            }
            Expression::Yield(value) => {
                if let Some(value) = value {
                    self.generate_expression(*value)?;
                }
                else {
                    self.instructions.push(Instruction::PushNull);
                }
                self.instructions.push(Instruction::Yield);
            }
            Expression::YieldFrom(value) => {
                self.generate_expression(*value)?;
                self.instructions.push(Instruction::YieldFrom);
            }
            Expression::Compare { left, ops, comparators } => {
                self.generate_compare(*left, ops.into_iter().zip(comparators).collect())?;
            }
            Expression::BoolOp { operator, values } => {
                self.generate_bool_op(operator, values)?;
            }
            _ => todo!("Implement other expression types"),
        }
        Ok(())
    }

    /// Generate bytecode for a literal
    fn generate_literal(&mut self, literal: Literal) {
        match literal {
            Literal::Integer(i) => {
                self.instructions.push(Instruction::PushInteger(i));
            }
            Literal::Float(f) => {
                self.instructions.push(Instruction::PushFloat(f));
            }
            Literal::String(s) => {
                self.instructions.push(Instruction::PushString(s));
            }
            Literal::Bytes(b) => {
                // TODO: implement bytes literal
            }
            Literal::Boolean(b) => {
                if b {
                    self.instructions.push(Instruction::PushTrue);
                }
                else {
                    self.instructions.push(Instruction::PushFalse);
                }
            }
            Literal::None => {
                self.instructions.push(Instruction::PushNull);
            }
        }
    }

    /// Generate bytecode for a binary operator
    fn generate_binary_op(&mut self, op: BinaryOperator) {
        match op {
            BinaryOperator::Add => self.instructions.push(Instruction::Add),
            BinaryOperator::Sub => self.instructions.push(Instruction::Subtract),
            BinaryOperator::Mult => self.instructions.push(Instruction::Multiply),
            BinaryOperator::Div => self.instructions.push(Instruction::Divide),
            BinaryOperator::FloorDiv => self.instructions.push(Instruction::FloorDivide),
            BinaryOperator::Mod => self.instructions.push(Instruction::Modulo),
            BinaryOperator::Pow => self.instructions.push(Instruction::Power),
            BinaryOperator::LShift => self.instructions.push(Instruction::LeftShift),
            BinaryOperator::RShift => self.instructions.push(Instruction::RightShift),
            BinaryOperator::BitAnd => self.instructions.push(Instruction::BitwiseAnd),
            BinaryOperator::BitOr => self.instructions.push(Instruction::BitwiseOr),
            BinaryOperator::BitXor => self.instructions.push(Instruction::BitwiseXor),
            _ => todo!("Implement other binary operators"),
        }
    }

    /// Generate bytecode for a unary operator
    fn generate_unary_op(&mut self, op: UnaryOperator) {
        match op {
            UnaryOperator::UAdd => todo!("Implement positive operator"),
            UnaryOperator::USub => self.instructions.push(Instruction::Subtract),
            UnaryOperator::Not => self.instructions.push(Instruction::Not),
            UnaryOperator::Invert => self.instructions.push(Instruction::BitwiseNot),
        }
    }

    /// Generate bytecode for a function definition
    fn generate_function_def(
        &mut self,
        name: String,
        parameters: Vec<Parameter>,
        return_type: Option<Type>,
        body: Vec<Statement>,
        decorators: Vec<Expression>,
    ) -> PythonResult<()> {
        // TODO: Implement function definition
        let _ = (name, parameters, return_type, body, decorators);
        Ok(())
    }

    /// Generate bytecode for a class definition
    fn generate_class_def(
        &mut self,
        name: String,
        bases: Vec<Expression>,
        body: Vec<Statement>,
        decorators: Vec<Expression>,
    ) -> PythonResult<()> {
        // TODO: Implement class definition
        Ok(())
    }

    /// Generate bytecode for an if statement
    fn generate_if(&mut self, test: Box<Expression>, body: Vec<Statement>, orelse: Vec<Statement>) -> PythonResult<()> {
        // Generate test expression
        self.generate_expression(*test)?;

        // Jump to orelse if test is false
        let orelse_jump = self.instructions.len();
        self.instructions.push(Instruction::JumpIfFalse(0)); // Placeholder

        // Generate body
        for stmt in body {
            self.generate_statement(stmt)?;
        }

        // Jump to end if body is executed
        let end_jump = self.instructions.len();
        self.instructions.push(Instruction::Jump(0)); // Placeholder

        // Update orelse jump target
        self.instructions[orelse_jump] = Instruction::JumpIfFalse(self.instructions.len());

        // Generate orelse
        for stmt in orelse {
            self.generate_statement(stmt)?;
        }

        // Update end jump target
        self.instructions[end_jump] = Instruction::Jump(self.instructions.len());

        Ok(())
    }

    /// Generate bytecode for a for statement
    fn generate_for(
        &mut self,
        target: Expression,
        iter: Expression,
        body: Vec<Statement>,
        orelse: Vec<Statement>,
    ) -> PythonResult<()> {
        // TODO: Implement for loop
        let _ = (target, iter, body, orelse);
        Ok(())
    }

    /// Generate bytecode for a while statement
    fn generate_while(&mut self, test: Box<Expression>, body: Vec<Statement>, orelse: Vec<Statement>) -> PythonResult<()> {
        // TODO: Implement while loop
        Ok(())
    }

    /// Generate bytecode for a try statement
    fn generate_try(
        &mut self,
        body: Vec<Statement>,
        handlers: Vec<ExceptHandler>,
        orelse: Vec<Statement>,
        finalbody: Vec<Statement>,
    ) -> PythonResult<()> {
        // TODO: Implement try statement
        Ok(())
    }

    /// Generate bytecode for a with statement
    fn generate_with(&mut self, items: Vec<WithItem>, body: Vec<Statement>) -> PythonResult<()> {
        // TODO: Implement with statement
        let _ = (items, body);
        Ok(())
    }

    /// Generate bytecode for a return statement
    fn generate_return(&mut self, value: Option<Expression>) -> PythonResult<()> {
        if let Some(value) = value {
            self.generate_expression(value)?;
        }
        else {
            self.instructions.push(Instruction::PushNull);
        }
        self.instructions.push(Instruction::Return);
        Ok(())
    }

    /// Generate bytecode for a raise statement
    fn generate_raise(&mut self, exc: Option<Expression>, cause: Option<Expression>) -> PythonResult<()> {
        // TODO: Implement raise statement
        let _ = (exc, cause);
        self.instructions.push(Instruction::Raise);
        Ok(())
    }

    /// Generate bytecode for an assert statement
    fn generate_assert(&mut self, test: Expression, msg: Option<Expression>) -> PythonResult<()> {
        // TODO: Implement assert statement
        let _ = (test, msg);
        Ok(())
    }

    /// Generate bytecode for an import statement
    fn generate_import(&mut self, names: Vec<(String, Option<String>)>) -> PythonResult<()> {
        // TODO: Implement import statement
        Ok(())
    }

    /// Generate bytecode for an import from statement
    fn generate_import_from(
        &mut self,
        module: Option<String>,
        names: Vec<(String, Option<String>)>,
        level: usize,
    ) -> PythonResult<()> {
        // TODO: Implement import from statement
        Ok(())
    }

    /// Generate bytecode for a global statement
    fn generate_global(&mut self, names: Vec<String>) -> PythonResult<()> {
        // TODO: Implement global statement
        Ok(())
    }

    /// Generate bytecode for a nonlocal statement
    fn generate_nonlocal(&mut self, names: Vec<String>) -> PythonResult<()> {
        // TODO: Implement nonlocal statement
        Ok(())
    }

    /// Generate bytecode for an assign statement
    fn generate_assign(&mut self, target: Expression, value: Expression) -> PythonResult<()> {
        // TODO: Implement assign statement
        let _ = (target, value);
        Ok(())
    }

    /// Generate bytecode for an augmented assign statement
    fn generate_aug_assign(&mut self, target: Expression, op: AugmentedOperator, value: Expression) -> PythonResult<()> {
        // TODO: Implement augmented assign statement
        let _ = (target, op, value);
        Ok(())
    }

    /// Generate bytecode for a del statement
    fn generate_del(&mut self, targets: Vec<Expression>) -> PythonResult<()> {
        // TODO: Implement del statement
        let _ = targets;
        Ok(())
    }

    /// Generate bytecode for a lambda expression
    fn generate_lambda(&mut self, params: Vec<Parameter>, body: Expression) -> PythonResult<()> {
        // TODO: Implement lambda expression
        Ok(())
    }

    /// Generate bytecode for a list comprehension
    fn generate_list_comp(&mut self, expr: Box<Expression>, comprehensions: Vec<Comprehension>) -> PythonResult<()> {
        // TODO: Implement list comprehension
        Ok(())
    }

    /// Generate bytecode for a dict comprehension
    fn generate_dict_comp(
        &mut self,
        key: Expression,
        value: Expression,
        comprehensions: Vec<Comprehension>,
    ) -> PythonResult<()> {
        // TODO: Implement dict comprehension
        Ok(())
    }

    /// Generate bytecode for a set comprehension
    fn generate_set_comp(&mut self, expr: Box<Expression>, comprehensions: Vec<Comprehension>) -> PythonResult<()> {
        // TODO: Implement set comprehension
        Ok(())
    }

    /// Generate bytecode for a generator expression
    fn generate_generator_exp(&mut self, expr: Box<Expression>, comprehensions: Vec<Comprehension>) -> PythonResult<()> {
        // TODO: Implement generator expression
        Ok(())
    }

    /// Generate bytecode for a compare expression
    fn generate_compare(&mut self, expr: Expression, ops: Vec<(ComparisonOperator, Expression)>) -> PythonResult<()> {
        // TODO: Implement compare expression
        Ok(())
    }

    /// Generate bytecode for a bool op expression
    fn generate_bool_op(&mut self, op: BoolOperator, values: Vec<Expression>) -> PythonResult<()> {
        // TODO: Implement bool op expression
        Ok(())
    }
}

/// Convert IR to VM instructions
pub fn ir_to_vm_instructions(_ir: &python_ir::Module) -> PythonResult<BytecodeFunction> {
    // TODO: Implement IR to VM instructions conversion
    Ok(BytecodeFunction {
        name: "<module>".to_string(),
        instructions: smallvec![],
        constants: vec![],
        names: vec![],
        varnames: vec![],
        argcount: 0,
        kwonlyargcount: 0,
        nlocals: 0,
        stacksize: 0,
    })
}
