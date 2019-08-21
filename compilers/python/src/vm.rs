//! Virtual machine for Rusty Python
//!
//! This module implements the bytecode interpreter that executes Python bytecode.

use crate::{
    codegen::{BytecodeFunction, Instruction},
    jit::{JIT, JITExt},
};
use python_ir::Module;
use python_types::{PythonError, PythonResult, PythonValue};
use smallvec::SmallVec;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

/// 函数类型别名
pub type PythonFunction = Box<dyn Fn(&mut Context, Vec<Arc<PythonValue>>) -> PythonResult<Arc<PythonValue>>>;

/// 类方法映射类型别名
pub type ClassMethods = HashMap<String, PythonFunction>;

/// VM上下文
pub struct Context {
    globals: HashMap<String, Arc<PythonValue>>,
    class_methods: HashMap<String, ClassMethods>,
}

impl Context {
    /// 创建新的上下文
    pub fn new() -> Self {
        Self { globals: HashMap::new(), class_methods: HashMap::new() }
    }

    /// 定义类
    pub fn define_class(&mut self, name: &str) {
        self.class_methods.insert(name.to_string(), HashMap::new());
    }

    /// 定义函数
    pub fn define_function(
        &mut self,
        name: &str,
        func: Box<dyn Fn(&mut Context, Vec<Arc<PythonValue>>) -> PythonResult<Arc<PythonValue>>>,
    ) {
        self.globals.insert(name.to_string(), Arc::new(PythonValue::Function(name.to_string())));
    }

    /// 获取类方法
    pub fn get_class_methods(&mut self, class: &str) -> Option<&mut ClassMethods> {
        self.class_methods.get_mut(class)
    }

    /// 获取全局变量
    pub fn get_global(&self, name: &str) -> Option<Arc<PythonValue>> {
        self.globals.get(name).cloned()
    }

    /// 设置全局变量
    pub fn set_global(&mut self, name: &str, value: Arc<PythonValue>) {
        self.globals.insert(name.to_string(), value);
    }
}

/// 栈帧结构
pub struct Frame {
    locals: HashMap<String, PythonValue>,
    instructions: SmallVec<[Instruction; 64]>,
    pc: usize,
    return_address: Option<usize>,
}

/// Virtual machine state
pub struct VirtualMachine {
    stack: SmallVec<[PythonValue; 64]>,
    globals: HashMap<String, PythonValue>,
    frames: Vec<Frame>,
    instructions: SmallVec<[Instruction; 64]>,
    pc: usize, // program counter
    context: Mutex<Context>,
    jit: Option<JIT>,
    ffi: crate::ffi::FFI,
}

impl VirtualMachine {
    /// Create a new virtual machine
    pub fn new() -> Self {
        let jit = JIT::new().ok();
        Self {
            stack: smallvec![],
            globals: HashMap::new(),
            frames: Vec::new(),
            instructions: smallvec![],
            pc: 0,
            context: Mutex::new(Context::new()),
            jit,
            ffi: crate::ffi::FFI::new(),
        }
    }

    /// Get the context
    pub fn context(&self) -> &Mutex<Context> {
        &self.context
    }

    /// Get the JIT instance
    pub fn jit(&self) -> Option<&JIT> {
        self.jit.as_ref()
    }

    /// Get the mutable JIT instance
    pub fn jit_mut(&mut self) -> Option<&mut JIT> {
        self.jit.as_mut()
    }

    /// Get the FFI instance (immutable)
    pub fn ffi(&self) -> &crate::ffi::FFI {
        &self.ffi
    }

    /// Get the FFI instance (mutable)
    pub fn ffi_mut(&mut self) -> &mut crate::ffi::FFI {
        &mut self.ffi
    }

    /// Execute a bytecode function
    pub fn execute(&mut self, func: &BytecodeFunction) -> PythonResult<PythonValue> {
        // 创建新的栈帧
        let new_frame = Frame { locals: HashMap::new(), instructions: func.instructions.clone(), pc: 0, return_address: None };

        // 保存当前帧状态
        let current_frame = Frame {
            locals: HashMap::new(), // 主帧没有局部变量
            instructions: self.instructions.clone(),
            pc: self.pc,
            return_address: None,
        };

        // 压入新帧并执行
        self.frames.push(current_frame);
        self.instructions = new_frame.instructions;
        self.pc = new_frame.pc;

        while self.pc < self.instructions.len() {
            let instruction = self.instructions[self.pc].clone();
            self.execute_instruction(&instruction)?;
            self.pc += 1;
        }

        // 弹出栈帧，恢复之前的状态
        if let Some(prev_frame) = self.frames.pop() {
            self.instructions = prev_frame.instructions;
            self.pc = prev_frame.pc;
        }

        // Return the top of the stack
        if let Some(result) = self.stack.pop() { Ok(result) } else { Ok(PythonValue::None) }
    }

    /// Execute a single instruction
    fn execute_instruction(&mut self, instruction: &Instruction) -> PythonResult<()> {
        match instruction {
            // Stack manipulation
            Instruction::PushNull => {
                self.stack.push(PythonValue::None);
            }
            Instruction::PushTrue => {
                self.stack.push(PythonValue::Boolean(true));
            }
            Instruction::PushFalse => {
                self.stack.push(PythonValue::Boolean(false));
            }
            Instruction::PushInteger(i) => {
                self.stack.push(PythonValue::Integer(*i));
            }
            Instruction::PushFloat(f) => {
                self.stack.push(PythonValue::Float(*f));
            }
            Instruction::PushString(s) => {
                self.stack.push(PythonValue::String(s.clone()));
            }
            Instruction::PushName(name) => {
                // 首先检查当前栈帧的局部变量
                if let Some(frame) = self.frames.last_mut() {
                    if let Some(obj) = frame.locals.get(name) {
                        self.stack.push(obj.clone());
                        return Ok(());
                    }
                }
                // 然后检查全局变量
                if let Some(obj) = self.get_global_from_context(name) {
                    self.stack.push(obj.as_ref().clone());
                }
                else {
                    return Err(PythonError::NameError(format!("name '{}' is not defined", name)));
                }
            }
            Instruction::Pop => {
                self.stack.pop();
            }
            Instruction::Dup => {
                if let Some(top) = self.stack.last() {
                    self.stack.push(top.clone());
                }
            }
            Instruction::Rot2 => {
                if self.stack.len() >= 2 {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a);
                    self.stack.push(b);
                }
            }
            Instruction::Rot3 => {
                if self.stack.len() >= 3 {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    let c = self.stack.pop().unwrap();
                    self.stack.push(a);
                    self.stack.push(c);
                    self.stack.push(b);
                }
            }

            // Arithmetic operations
            Instruction::Add => {
                self.execute_binary_op(|a, b| a.add(&b))?;
            }
            Instruction::Subtract => {
                self.execute_binary_op(|a, b| a.sub(&b))?;
            }
            Instruction::Multiply => {
                self.execute_binary_op(|a, b| a.mul(&b))?;
            }
            Instruction::Divide => {
                self.execute_binary_op(|a, b| a.div(&b))?;
            }
            Instruction::FloorDivide => {
                self.execute_binary_op(|a, b| match (a, b) {
                    (PythonValue::Integer(a), PythonValue::Integer(b)) => {
                        if b == 0 {
                            Err(PythonError::ZeroDivisionError("integer division or modulo by zero".to_string()))
                        }
                        else if (a < 0) != (b < 0) {
                            Ok(PythonValue::Integer((a as f64 / b as f64).floor() as i64))
                        }
                        else {
                            Ok(PythonValue::Integer(a / b))
                        }
                    }
                    (PythonValue::Integer(a), PythonValue::Float(b)) => {
                        if b == 0.0 {
                            Err(PythonError::ZeroDivisionError("float division by zero".to_string()))
                        }
                        else {
                            Ok(PythonValue::Float((a as f64 / b).floor()))
                        }
                    }
                    (PythonValue::Float(a), PythonValue::Integer(b)) => {
                        if b == 0 {
                            Err(PythonError::ZeroDivisionError("float division by zero".to_string()))
                        }
                        else {
                            Ok(PythonValue::Float((a / b as f64).floor()))
                        }
                    }
                    (PythonValue::Float(a), PythonValue::Float(b)) => {
                        if b == 0.0 {
                            Err(PythonError::ZeroDivisionError("float division by zero".to_string()))
                        }
                        else {
                            Ok(PythonValue::Float((a / b).floor()))
                        }
                    }
                    _ => Err(PythonError::TypeError("unsupported operand type(s) for //".to_string())),
                })?;
            }
            Instruction::Modulo => {
                self.execute_binary_op(|a, b| match (a, b) {
                    (PythonValue::Integer(a), PythonValue::Integer(b)) => {
                        if b == 0 {
                            Err(PythonError::ZeroDivisionError("integer division or modulo by zero".to_string()))
                        }
                        else {
                            Ok(PythonValue::Integer(a % b))
                        }
                    }
                    (PythonValue::Integer(a), PythonValue::Float(b)) => {
                        if b == 0.0 {
                            Err(PythonError::ZeroDivisionError("float division by zero".to_string()))
                        }
                        else {
                            Ok(PythonValue::Float((a as f64).rem_euclid(b)))
                        }
                    }
                    (PythonValue::Float(a), PythonValue::Integer(b)) => {
                        if b == 0 {
                            Err(PythonError::ZeroDivisionError("float division by zero".to_string()))
                        }
                        else {
                            Ok(PythonValue::Float(a.rem_euclid(b as f64)))
                        }
                    }
                    (PythonValue::Float(a), PythonValue::Float(b)) => {
                        if b == 0.0 {
                            Err(PythonError::ZeroDivisionError("float division by zero".to_string()))
                        }
                        else {
                            Ok(PythonValue::Float(a.rem_euclid(b)))
                        }
                    }
                    _ => Err(PythonError::TypeError("unsupported operand type(s) for %".to_string())),
                })?;
            }
            Instruction::Power => {
                self.execute_binary_op(|a, b| match (a, b) {
                    (PythonValue::Integer(a), PythonValue::Integer(b)) => {
                        if b < 0 {
                            Ok(PythonValue::Float((a as f64).powf(b as f64)))
                        }
                        else {
                            Ok(PythonValue::Integer(a.pow(b as u32)))
                        }
                    }
                    (PythonValue::Integer(a), PythonValue::Float(b)) => Ok(PythonValue::Float((a as f64).powf(b))),
                    (PythonValue::Float(a), PythonValue::Integer(b)) => Ok(PythonValue::Float(a.powf(b as f64))),
                    (PythonValue::Float(a), PythonValue::Float(b)) => Ok(PythonValue::Float(a.powf(b))),
                    _ => Err(PythonError::TypeError("unsupported operand type(s) for **".to_string())),
                })?;
            }

            // Bitwise operations
            Instruction::BitwiseAnd => {
                self.execute_binary_op(|a, b| match (a, b) {
                    (PythonValue::Integer(a), PythonValue::Integer(b)) => Ok(PythonValue::Integer(a & b)),
                    _ => Err(PythonError::TypeError("unsupported operand type(s) for &".to_string())),
                })?;
            }
            Instruction::BitwiseOr => {
                self.execute_binary_op(|a, b| match (a, b) {
                    (PythonValue::Integer(a), PythonValue::Integer(b)) => Ok(PythonValue::Integer(a | b)),
                    _ => Err(PythonError::TypeError("unsupported operand type(s) for |".to_string())),
                })?;
            }
            Instruction::BitwiseXor => {
                self.execute_binary_op(|a, b| match (a, b) {
                    (PythonValue::Integer(a), PythonValue::Integer(b)) => Ok(PythonValue::Integer(a ^ b)),
                    _ => Err(PythonError::TypeError("unsupported operand type(s) for ^".to_string())),
                })?;
            }
            Instruction::LeftShift => {
                self.execute_binary_op(|a, b| match (a, b) {
                    (PythonValue::Integer(a), PythonValue::Integer(b)) => {
                        if b < 0 {
                            Err(PythonError::ValueError("negative shift count".to_string()))
                        }
                        else {
                            Ok(PythonValue::Integer(a << b))
                        }
                    }
                    _ => Err(PythonError::TypeError("unsupported operand type(s) for <<".to_string())),
                })?;
            }
            Instruction::RightShift => {
                self.execute_binary_op(|a, b| match (a, b) {
                    (PythonValue::Integer(a), PythonValue::Integer(b)) => {
                        if b < 0 {
                            Err(PythonError::ValueError("negative shift count".to_string()))
                        }
                        else {
                            Ok(PythonValue::Integer(a >> b))
                        }
                    }
                    _ => Err(PythonError::TypeError("unsupported operand type(s) for >>".to_string())),
                })?;
            }
            Instruction::BitwiseNot => {
                if let Some(obj) = self.stack.pop() {
                    match obj {
                        PythonValue::Integer(i) => {
                            self.stack.push(PythonValue::Integer(!i));
                        }
                        _ => {
                            return Err(PythonError::TypeError("unsupported operand type(s) for ~".to_string()));
                        }
                    }
                }
            }

            // Comparison operations
            Instruction::Equal => {
                self.execute_binary_op(|a, b| Ok(PythonValue::Boolean(a.eq(&b))))?;
            }
            Instruction::NotEqual => {
                self.execute_binary_op(|a, b| Ok(PythonValue::Boolean(!a.eq(&b))))?;
            }
            Instruction::LessThan => {
                self.execute_binary_op(|a, b| a.lt(&b).map(PythonValue::Boolean))?;
            }
            Instruction::LessThanOrEqual => {
                self.execute_binary_op(|a, b| match (a, b) {
                    (PythonValue::Integer(a), PythonValue::Integer(b)) => Ok(PythonValue::Boolean(a <= b)),
                    (PythonValue::Integer(a), PythonValue::Float(b)) => Ok(PythonValue::Boolean((a as f64) <= b)),
                    (PythonValue::Float(a), PythonValue::Integer(b)) => Ok(PythonValue::Boolean(a <= (b as f64))),
                    (PythonValue::Float(a), PythonValue::Float(b)) => Ok(PythonValue::Boolean(a <= b)),
                    (PythonValue::String(a), PythonValue::String(b)) => Ok(PythonValue::Boolean(a <= b)),
                    _ => Err(PythonError::TypeError("unorderable types".to_string())),
                })?;
            }
            Instruction::GreaterThan => {
                self.execute_binary_op(|a, b| a.gt(&b).map(PythonValue::Boolean))?;
            }
            Instruction::GreaterThanOrEqual => {
                self.execute_binary_op(|a, b| match (a, b) {
                    (PythonValue::Integer(a), PythonValue::Integer(b)) => Ok(PythonValue::Boolean(a >= b)),
                    (PythonValue::Integer(a), PythonValue::Float(b)) => Ok(PythonValue::Boolean((a as f64) >= b)),
                    (PythonValue::Float(a), PythonValue::Integer(b)) => Ok(PythonValue::Boolean(a >= (b as f64))),
                    (PythonValue::Float(a), PythonValue::Float(b)) => Ok(PythonValue::Boolean(a >= b)),
                    (PythonValue::String(a), PythonValue::String(b)) => Ok(PythonValue::Boolean(a >= b)),
                    _ => Err(PythonError::TypeError("unorderable types".to_string())),
                })?;
            }

            // Logical operations
            Instruction::And => {
                if let Some(b) = self.stack.pop() {
                    if !b.to_bool() {
                        self.stack.push(b);
                    }
                    else {
                        if let Some(a) = self.stack.pop() {
                            self.stack.push(a);
                        }
                        else {
                            self.stack.push(b);
                        }
                    }
                }
            }
            Instruction::Or => {
                if let Some(b) = self.stack.pop() {
                    if b.to_bool() {
                        self.stack.push(b);
                    }
                    else {
                        if let Some(a) = self.stack.pop() {
                            self.stack.push(a);
                        }
                        else {
                            self.stack.push(b);
                        }
                    }
                }
            }
            Instruction::Not => {
                if let Some(obj) = self.stack.pop() {
                    self.stack.push(PythonValue::Boolean(!obj.to_bool()));
                }
            }

            // Control flow
            Instruction::Jump(target) => {
                self.pc = *target - 1; // Subtract 1 because pc will be incremented
            }
            Instruction::JumpIfTrue(target) => {
                if let Some(obj) = self.stack.pop() {
                    if obj.to_bool() {
                        self.pc = *target - 1;
                    }
                }
            }
            Instruction::JumpIfFalse(target) => {
                if let Some(obj) = self.stack.pop() {
                    if !obj.to_bool() {
                        self.pc = *target - 1;
                    }
                }
            }
            Instruction::Return => {
                // Return from function
                return Ok(());
            }
            Instruction::Yield => {
                // TODO: Implement yield
            }
            Instruction::YieldFrom => {
                // TODO: Implement yield from
            }

            // Function operations
            Instruction::Call(arg_count) => {
                // 从栈中弹出参数和函数
                let mut args = Vec::new();
                for _ in 0..*arg_count {
                    if let Some(arg) = self.stack.pop() {
                        args.insert(0, arg);
                    }
                }
                if let Some(func) = self.stack.pop() {
                    match func {
                        PythonValue::Function(name) => {
                            // 尝试使用 JIT 执行
                            if let Some(result) = self.try_jit_execute(&name, args.into_iter().map(Arc::new).collect()) {
                                match result {
                                    Ok(value) => {
                                        self.stack.push(value.as_ref().clone());
                                    }
                                    Err(err) => {
                                        return Err(err);
                                    }
                                }
                            }
                            else {
                                // 回退到解释执行
                                // TODO: 实现解释执行函数调用
                                self.stack.push(PythonValue::None);
                            }
                        }
                        _ => {
                            return Err(PythonError::TypeError(
                                "'{}' object is not callable".to_string().replace("{}", &func.to_string()),
                            ));
                        }
                    }
                }
            }
            Instruction::MakeFunction(param_count) => {
                // 从栈中弹出函数体和名称
                // TODO: 实现函数创建
                self.stack.push(PythonValue::Function("anonymous".to_string()));
            }
            Instruction::LoadMethod(name) => {
                // 从栈中弹出对象
                if let Some(obj) = self.stack.pop() {
                    // TODO: 实现方法加载
                    let obj_clone = obj.clone();
                    self.stack.push(obj);
                    self.stack.push(PythonValue::Function(format!("{}.{}", obj_clone.to_string(), name)));
                }
            }

            // Attribute operations
            Instruction::GetAttribute(name) => {
                if let Some(obj) = self.stack.pop() {
                    match &obj {
                        PythonValue::Object(_, attrs) => {
                            if let Some(value) = attrs.get(name) {
                                self.stack.push(value.as_ref().clone());
                            }
                            else {
                                return Err(PythonError::AttributeError(format!(
                                    "'{}' object has no attribute '{}'",
                                    obj.to_string(),
                                    name
                                )));
                            }
                        }
                        _ => {
                            // TODO: 实现内置类型的属性访问
                            self.stack.push(PythonValue::None);
                        }
                    }
                }
            }
            Instruction::SetAttribute(name) => {
                if let (Some(value), Some(obj)) = (self.stack.pop(), self.stack.pop()) {
                    match obj {
                        PythonValue::Object(_, mut attrs) => {
                            attrs.insert(name.to_string(), Arc::new(value));
                            self.stack.push(PythonValue::Object("object".to_string(), attrs));
                        }
                        _ => {
                            // TODO: 实现内置类型的属性设置
                            self.stack.push(obj);
                        }
                    }
                }
            }

            // Subscript operations
            Instruction::GetItem => {
                if let (Some(index), Some(obj)) = (self.stack.pop(), self.stack.pop()) {
                    match obj.get_item(&index) {
                        Ok(result) => self.stack.push(result.as_ref().clone()),
                        Err(_) => self.stack.push(PythonValue::None),
                    }
                }
            }
            Instruction::SetItem => {
                if let (Some(value), Some(index), Some(mut obj)) = (self.stack.pop(), self.stack.pop(), self.stack.pop()) {
                    obj.set_item(&index, std::sync::Arc::new(value))?;
                    self.stack.push(obj);
                }
            }

            // Object operations
            Instruction::BuildTuple(size) => {
                self.build_collection(*size, PythonValue::Tuple)?;
            }
            Instruction::BuildList(size) => {
                self.build_collection(*size, PythonValue::List)?;
            }
            Instruction::BuildDict(size) => {
                self.build_dict(*size)?;
            }
            Instruction::BuildSet(size) => {
                // TODO: Implement build set
            }

            // Exception handling
            Instruction::SetupExcept(target) => {
                // TODO: Implement exception handling
            }
            Instruction::SetupFinally(target) => {
                // TODO: Implement finally handling
            }
            Instruction::EndFinally => {
                // TODO: Implement end finally
            }
            Instruction::Raise => {
                // TODO: Implement raise
            }

            // Loop operations
            Instruction::SetupLoop(target) => {
                // 保存循环开始位置
                // TODO: 实现循环栈管理
            }
            Instruction::Break => {
                // 跳转到循环结束位置
                // TODO: 实现循环栈管理和跳转
            }
            Instruction::Continue => {
                // 跳转到循环开始位置
                // TODO: 实现循环栈管理和跳转
            }

            // Context management
            Instruction::SetupWith(target) => {
                // TODO: Implement with statement
            }
        }
        Ok(())
    }

    /// Execute a binary operation
    fn execute_binary_op<F>(&mut self, op: F) -> PythonResult<()>
    where
        F: Fn(PythonValue, PythonValue) -> PythonResult<PythonValue>,
    {
        if self.stack.len() >= 2 {
            let b = self.stack.pop().unwrap();
            let a = self.stack.pop().unwrap();
            let result = op(a, b)?;
            self.stack.push(result);
        }
        Ok(())
    }

    /// Build a collection (tuple, list, set)
    fn build_collection(
        &mut self,
        size: usize,
        constructor: fn(Vec<std::sync::Arc<PythonValue>>) -> PythonValue,
    ) -> PythonResult<()> {
        if self.stack.len() >= size {
            let mut items = Vec::with_capacity(size);
            // 从栈中弹出元素，注意顺序
            for _ in 0..size {
                items.push(std::sync::Arc::new(self.stack.pop().unwrap()));
            }
            // 反转顺序以保持正确的顺序
            items.reverse();
            self.stack.push(constructor(items));
        }
        Ok(())
    }

    /// Build a dictionary
    fn build_dict(&mut self, size: usize) -> PythonResult<()> {
        if self.stack.len() >= size * 2 {
            let mut items = std::collections::HashMap::with_capacity(size);
            for _ in 0..size {
                let value = std::sync::Arc::new(self.stack.pop().unwrap());
                let key = self.stack.pop().unwrap();
                // 使用 key 的字符串表示作为字典键
                items.insert(key.to_string(), value);
            }
            self.stack.push(PythonValue::Dict(items));
        }
        Ok(())
    }

    /// Set a global variable
    pub fn set_global(&mut self, name: String, value: PythonValue) {
        self.globals.insert(name, value);
    }

    /// Get a global variable
    pub fn get_global(&self, name: &str) -> Option<&PythonValue> {
        self.globals.get(name)
    }

    /// Set a local variable
    pub fn set_local(&mut self, name: String, value: PythonValue) {
        if let Some(frame) = self.frames.last_mut() {
            frame.locals.insert(name, value);
        }
    }

    /// Get a local variable
    pub fn get_local(&self, name: &str) -> Option<&PythonValue> {
        if let Some(frame) = self.frames.last() { frame.locals.get(name) } else { None }
    }

    /// Check and compile hot functions
    pub fn check_and_compile_hot_functions(&mut self, _ir: &python_ir::Module) {
        // TODO: Implement JIT compilation for hot functions
    }

    /// Define a method for a class
    pub fn define_method_in_context(
        &mut self,
        class: &str,
        name: &str,
        func: Box<dyn Fn(&mut Context, Vec<Arc<PythonValue>>) -> PythonResult<Arc<PythonValue>>>,
    ) -> bool {
        let mut context = self.context().lock().unwrap();
        if let Some(class_methods) = (*context).class_methods.get_mut(class) {
            class_methods.insert(name.to_string(), func);
            true
        }
        else {
            false
        }
    }

    /// Get global variable from context
    pub fn get_global_from_context(&self, name: &str) -> Option<Arc<PythonValue>> {
        let context = self.context().lock().unwrap();
        context.globals.get(name).cloned()
    }

    /// Set global variable in context
    pub fn set_global_in_context(&mut self, name: &str, value: Arc<PythonValue>) {
        let mut context = self.context().lock().unwrap();
        context.globals.insert(name.to_string(), value);
    }
}

/// 虚拟机类型别名
pub type VM = VirtualMachine;
