//! Python FFI 接口实现
//!
//! 实现 Python 与 Rust 之间的互操作，包括 Python 调用 Rust 函数和 Rust 调用 Python 函数。

use crate::vm::{Context, VM};
use python_types::{PythonError, PythonResult, PythonValue};
use std::{
    ffi::{CStr, CString},
    ptr,
    sync::{Arc, Mutex},
};

/// FFI 函数类型
pub type FnType = Box<dyn Fn(&mut Context, Vec<Arc<PythonValue>>) -> PythonResult<Arc<PythonValue>> + Send + Sync>;

/// C 函数指针类型
pub type CFunctionPtr = *const extern "C" fn() -> i32;

/// FFI 管理器
pub struct FFI {
    /// 注册的 Rust 函数
    functions: std::collections::HashMap<String, FnType>,
    /// 注册的 C 函数
    c_functions: std::collections::HashMap<String, CFunctionPtr>,
}

impl FFI {
    /// 创建新的 FFI 管理器
    pub fn new() -> Self {
        Self { functions: std::collections::HashMap::new(), c_functions: std::collections::HashMap::new() }
    }

    /// 注册 Rust 函数
    pub fn register_function(&mut self, name: &str, func: FnType) {
        self.functions.insert(name.to_string(), func);
    }

    /// 注册 C 函数
    pub fn register_c_function(&mut self, name: &str, func: CFunctionPtr) {
        self.c_functions.insert(name.to_string(), func);
    }

    /// 调用 Rust 函数
    pub fn call_function(
        &self,
        name: &str,
        context: &mut Context,
        args: Vec<Arc<PythonValue>>,
    ) -> PythonResult<Arc<PythonValue>> {
        if let Some(func) = self.functions.get(name) {
            func(context, args)
        }
        else {
            Err(PythonError::NameError(format!("Function not found: {}", name)))
        }
    }

    /// 调用 C 函数
    pub fn call_c_function(&self, name: &str, _args: Vec<Arc<PythonValue>>) -> PythonResult<Arc<PythonValue>> {
        if let Some(func_ptr) = self.c_functions.get(name) {
            // 由于 C 函数指针类型的复杂性，我们暂时返回一个默认值
            // 实际实现需要根据具体的 C 函数签名进行类型转换
            Ok(Arc::new(PythonValue::Integer(42)))
        }
        else {
            Err(PythonError::NameError(format!("C function not found: {}", name)))
        }
    }

    /// Get the FFI instance
    pub fn ffi(&self) -> &Self {
        self
    }
}

/// FFI 扩展
pub trait FFIExt {
    /// 注册 Rust 函数到 Python
    fn register_rust_function(&mut self, name: &str, func: FnType);

    /// 注册 C 函数到 Python
    fn register_c_function(&mut self, name: &str, func: CFunctionPtr);

    /// 从 Python 调用 Rust 函数
    fn call_rust_function(&mut self, name: &str, args: Vec<Arc<PythonValue>>) -> PythonResult<Arc<PythonValue>>;

    /// 从 Python 调用 C 函数
    fn call_c_function(&mut self, name: &str, args: Vec<Arc<PythonValue>>) -> PythonResult<Arc<PythonValue>>;
}

/// 为 VM 实现 FFI 扩展
impl FFIExt for VM {
    /// 注册 Rust 函数
    fn register_rust_function(&mut self, name: &str, func: FnType) {
        // 将函数添加到 VM 的 FFI 管理器中
        self.ffi_mut().register_function(name, func);
    }

    /// 注册 C 函数
    fn register_c_function(&mut self, name: &str, func: CFunctionPtr) {
        // 将 C 函数添加到 VM 的 FFI 管理器中
        self.ffi_mut().register_c_function(name, func);
    }

    /// 从 Python 调用 Rust 函数
    fn call_rust_function(&mut self, name: &str, args: Vec<Arc<PythonValue>>) -> PythonResult<Arc<PythonValue>> {
        // 从 VM 的 FFI 管理器中获取并调用函数
        // 先获取 context 锁
        let context = self.context();
        let mut context = context.lock().unwrap();

        // 然后获取 ffi 引用
        let ffi = self.ffi();
        ffi.call_function(name, &mut context, args)
    }

    /// 从 Python 调用 C 函数
    fn call_c_function(&mut self, name: &str, args: Vec<Arc<PythonValue>>) -> PythonResult<Arc<PythonValue>> {
        // 从 VM 的 FFI 管理器中获取并调用 C 函数
        let ffi = self.ffi();
        ffi.call_c_function(name, args)
    }
}

/// 将 Rust 值转换为 Python 值
pub trait ToPythonValue {
    /// 转换为 Python 值
    fn to_python_value(&self) -> Arc<PythonValue>;
}

/// 将 Python 值转换为 Rust 值
pub trait FromPythonValue<T> {
    /// 从 Python 值转换
    fn from_python_value(value: &PythonValue) -> PythonResult<T>;
}

/// 实现基本类型的转换
impl ToPythonValue for i64 {
    fn to_python_value(&self) -> Arc<PythonValue> {
        Arc::new(PythonValue::Integer(*self))
    }
}

impl ToPythonValue for f64 {
    fn to_python_value(&self) -> Arc<PythonValue> {
        Arc::new(PythonValue::Float(*self))
    }
}

impl ToPythonValue for bool {
    fn to_python_value(&self) -> Arc<PythonValue> {
        Arc::new(PythonValue::Boolean(*self))
    }
}

impl ToPythonValue for String {
    fn to_python_value(&self) -> Arc<PythonValue> {
        Arc::new(PythonValue::String(self.clone()))
    }
}

impl ToPythonValue for &str {
    fn to_python_value(&self) -> Arc<PythonValue> {
        Arc::new(PythonValue::String(self.to_string()))
    }
}

/// 实现从 Python 值到 Rust 基本类型的转换
impl FromPythonValue<i64> for PythonValue {
    fn from_python_value(value: &PythonValue) -> PythonResult<i64> {
        match value {
            PythonValue::Integer(i) => Ok(*i),
            PythonValue::Float(f) => Ok(*f as i64),
            PythonValue::Boolean(b) => Ok(if *b { 1 } else { 0 }),
            _ => Err(PythonError::TypeError("Cannot convert to i64".to_string())),
        }
    }
}

impl FromPythonValue<f64> for PythonValue {
    fn from_python_value(value: &PythonValue) -> PythonResult<f64> {
        match value {
            PythonValue::Float(f) => Ok(*f),
            PythonValue::Integer(i) => Ok(*i as f64),
            PythonValue::Boolean(b) => Ok(if *b { 1.0 } else { 0.0 }),
            _ => Err(PythonError::TypeError("Cannot convert to f64".to_string())),
        }
    }
}

impl FromPythonValue<bool> for PythonValue {
    fn from_python_value(value: &PythonValue) -> PythonResult<bool> {
        Ok(value.to_bool())
    }
}

impl FromPythonValue<String> for PythonValue {
    fn from_python_value(value: &PythonValue) -> PythonResult<String> {
        Ok(value.to_string())
    }
}
