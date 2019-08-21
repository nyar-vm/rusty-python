//! Python intermediate representation for Rusty Python
//!
//! This crate provides the intermediate representation (IR) for Python code,
//! which is used for optimization and code generation.

#![warn(missing_docs)]

use oak_python::ast::{BinaryOperator, Expression, Literal, Statement};
use python_types::{PythonError, PythonResult, PythonValue};

/// IR instruction
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // 加载指令
    LoadConst(PythonValue), // 加载常量到寄存器
    LoadLocal(usize),       // 加载局部变量到寄存器
    LoadGlobal(String),     // 加载全局变量到寄存器
    LoadAttr(String),       // 加载属性到寄存器
    LoadIndex(usize),       // 加载索引到寄存器

    // 存储指令
    StoreLocal(usize),   // 存储寄存器值到局部变量
    StoreGlobal(String), // 存储寄存器值到全局变量
    StoreAttr(String),   // 存储寄存器值到属性
    StoreIndex(usize),   // 存储寄存器值到索引

    // 算术指令
    Add,      // 加法
    Sub,      // 减法
    Mul,      // 乘法
    Div,      // 除法
    Mod,      // 取模
    Exp,      // 幂运算
    FloorDiv, // 地板除

    // 比较指令
    Eq,  // 等于
    Neq, // 不等于
    Lt,  // 小于
    Lte, // 小于等于
    Gt,  // 大于
    Gte, // 大于等于

    // 逻辑指令
    And, // 逻辑与
    Or,  // 逻辑或
    Not, // 逻辑非

    // 控制流指令
    Jump(i32),          // 无条件跳转
    JumpIfFalse(i32),   // 条件跳转（如果为假）
    JumpIfTrue(i32),    // 条件跳转（如果为真）
    JumpIfNone(i32),    // 条件跳转（如果为 None）
    JumpIfNotNone(i32), // 条件跳转（如果不为 None）

    // 方法调用指令
    Call(usize), // 调用函数，参数为参数个数
    Return,      // 返回

    // 数组和字典指令
    NewList(usize), // 创建新列表，参数为元素个数
    NewDict(usize), // 创建新字典，参数为键值对个数

    // 特殊指令
    NoneOp,  // 加载 None
    TrueOp,  // 加载 True
    FalseOp, // 加载 False

    // 异常处理指令
    Raise,    // 抛出异常
    TryBegin, // 开始 try 块
    TryEnd,   // 结束 try 块
    Except,   // 异常处理
}

/// IR function
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    /// Function name
    pub name: String,
    /// Parameters
    pub params: Vec<String>,
    /// Instructions
    pub instructions: Vec<Instruction>,
    /// Local variables
    pub locals: Vec<String>,
}

/// IR module
#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    /// Module name
    pub name: String,
    /// Functions
    pub functions: Vec<Function>,
    /// Global variables
    pub globals: Vec<String>,
}

/// Convert AST to IR
pub fn ast_to_ir(ast: &oak_python::PythonRoot) -> PythonResult<Module> {
    let mut module = Module { name: "__main__".to_string(), functions: Vec::new(), globals: Vec::new() };

    // Create a default main function if no functions are defined
    if module.functions.is_empty() {
        let main_function =
            Function { name: "__main__".to_string(), params: Vec::new(), instructions: Vec::new(), locals: Vec::new() };
        module.functions.push(main_function);
    }

    // Process each statement in the program
    for statement in &ast.program.statements {
        match statement {
            Statement::FunctionDef { name, parameters, body, .. } => {
                // Create a new function
                let mut function = Function {
                    name: name.clone(),
                    params: parameters.iter().map(|p| p.name.clone()).collect(),
                    instructions: Vec::new(),
                    locals: parameters.iter().map(|p| p.name.clone()).collect(),
                };

                // Process function body
                for stmt in body {
                    process_statement(stmt, &mut function.instructions, &mut function.locals)?;
                }

                // Add return statement if not present
                if !function.instructions.iter().any(|inst| matches!(inst, Instruction::Return)) {
                    function.instructions.push(Instruction::NoneOp);
                    function.instructions.push(Instruction::Return);
                }

                // Replace the default main function if this is the first function
                if module.functions.len() == 1 && module.functions[0].name == "__main__" {
                    module.functions[0] = function;
                }
                else {
                    module.functions.push(function);
                }
            }
            Statement::Assignment { target, value } => {
                // Process assignment
                process_expression(value, &mut module.functions.last_mut().unwrap().instructions)?;

                // Handle target
                if let Expression::Name(name) = target {
                    // Check if it's a local variable
                    if let Some(func) = module.functions.last_mut() {
                        if func.locals.contains(name) {
                            let index = func.locals.iter().position(|x| x == name).unwrap();
                            func.instructions.push(Instruction::StoreLocal(index));
                        }
                        else {
                            func.instructions.push(Instruction::StoreGlobal(name.clone()));
                            if !module.globals.contains(name) {
                                module.globals.push(name.clone());
                            }
                        }
                    }
                }
            }
            Statement::Expression(expr) => {
                // Process expression
                process_expression(expr, &mut module.functions.last_mut().unwrap().instructions)?;
            }
            Statement::Return(expr) => {
                // Process return value
                if let Some(expr) = expr {
                    process_expression(expr, &mut module.functions.last_mut().unwrap().instructions)?;
                }
                else {
                    module.functions.last_mut().unwrap().instructions.push(Instruction::NoneOp);
                }
                module.functions.last_mut().unwrap().instructions.push(Instruction::Return);
            }
            _ => {
                // TODO: Handle other statement types
            }
        }
    }

    // Add return statement to main function if not present
    if let Some(func) = module.functions.last_mut() {
        if !func.instructions.iter().any(|inst| matches!(inst, Instruction::Return)) {
            func.instructions.push(Instruction::NoneOp);
            func.instructions.push(Instruction::Return);
        }
    }

    Ok(module)
}

/// Process a statement and generate IR instructions
fn process_statement(stmt: &Statement, instructions: &mut Vec<Instruction>, locals: &mut Vec<String>) -> PythonResult<()> {
    match stmt {
        Statement::Assignment { target, value } => {
            // Process value expression
            process_expression(value, instructions)?;

            // Handle target
            if let Expression::Name(name) = target {
                if !locals.contains(name) {
                    locals.push(name.clone());
                }
                let index = locals.iter().position(|x| x == name).unwrap();
                instructions.push(Instruction::StoreLocal(index));
            }
        }
        Statement::Expression(expr) => {
            process_expression(expr, instructions)?;
        }
        Statement::Return(expr) => {
            if let Some(expr) = expr {
                process_expression(expr, instructions)?;
            }
            else {
                instructions.push(Instruction::NoneOp);
            }
            instructions.push(Instruction::Return);
        }
        _ => {
            // TODO: Handle other statement types
        }
    }
    Ok(())
}

/// Process an expression and generate IR instructions
fn process_expression(expr: &Expression, instructions: &mut Vec<Instruction>) -> PythonResult<()> {
    match expr {
        Expression::Literal(lit) => {
            match lit {
                Literal::Integer(i) => {
                    instructions.push(Instruction::LoadConst(PythonValue::Integer(*i)));
                }
                Literal::Float(f) => {
                    instructions.push(Instruction::LoadConst(PythonValue::Float(*f)));
                }
                Literal::String(s) => {
                    instructions.push(Instruction::LoadConst(PythonValue::String(s.clone())));
                }
                Literal::Boolean(b) => {
                    if *b {
                        instructions.push(Instruction::TrueOp);
                    }
                    else {
                        instructions.push(Instruction::FalseOp);
                    }
                }
                Literal::None => {
                    instructions.push(Instruction::NoneOp);
                }
                _ => {
                    // TODO: Handle other literals
                }
            }
        }
        Expression::Name(name) => {
            // TODO: Check if it's a local or global variable
            instructions.push(Instruction::LoadGlobal(name.clone()));
        }
        Expression::BinaryOp { left, operator, right } => {
            process_expression(left, instructions)?;
            process_expression(right, instructions)?;

            match operator {
                BinaryOperator::Add => instructions.push(Instruction::Add),
                BinaryOperator::Sub => instructions.push(Instruction::Sub),
                BinaryOperator::Mult => instructions.push(Instruction::Mul),
                BinaryOperator::Div => instructions.push(Instruction::Div),
                _ => {
                    // TODO: Handle other binary operators
                }
            }
        }
        Expression::Call { func, args, .. } => {
            // Process arguments
            for arg in args {
                process_expression(arg, instructions)?;
            }

            // Process function
            process_expression(func, instructions)?;

            // Call function
            instructions.push(Instruction::Call(args.len() as usize));
        }
        _ => {
            // TODO: Handle other expression types
        }
    }
    Ok(())
}

/// Optimize IR
pub fn optimize_ir(ir: &Module) -> PythonResult<Module> {
    let mut optimized_module = Module { name: ir.name.clone(), functions: Vec::new(), globals: ir.globals.clone() };

    // Optimize each function
    for func in &ir.functions {
        let mut optimized_func = Function {
            name: func.name.clone(),
            params: func.params.clone(),
            instructions: optimize_instructions(&func.instructions),
            locals: func.locals.clone(),
        };
        optimized_module.functions.push(optimized_func);
    }

    Ok(optimized_module)
}

/// Optimize a list of instructions
fn optimize_instructions(instructions: &[Instruction]) -> Vec<Instruction> {
    let mut optimized = Vec::new();
    let mut i = 0;

    while i < instructions.len() {
        // TODO: Implement optimization passes
        // For now, just copy the instructions
        optimized.push(instructions[i].clone());
        i += 1;
    }

    optimized
}
