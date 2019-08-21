//! Rusty Python 语言前端
//!
//! 这个库提供了 Rusty Python 语言的词法分析、语法分析和运行时功能。

#![warn(missing_docs)]

#[macro_use]
extern crate smallvec;

use oak_core::{Builder, TextEdit, source::Source};
use oak_python::{PythonBuilder, PythonLanguage, PythonRoot};
use python_ir::{ast_to_ir, optimize_ir};
use python_types::{PythonError, PythonResult, PythonValue};
use std::sync::Arc;

mod codegen;
mod ffi;
mod gc;
mod jit;
mod module;
mod vm;
use codegen::ir_to_vm_instructions;
use module::ModuleManager;

/// 导出虚拟机相关类型
pub use codegen::{BytecodeFunction, Instruction};
/// 导出垃圾回收相关类型
pub use gc::{Allocator, GC};
pub use vm::{Context, VM};

/// Python 运行时错误类型
type Result<T> = PythonResult<T>;

/// Trait for converting Rust types to Python values
pub trait ToPythonValue {
    /// Convert the Rust type to a Python Value
    fn to_python_value(&self) -> Arc<PythonValue>;
}

/// Python 运行时环境
pub struct Python {
    language: PythonLanguage,
    vm: VM,
    module_manager: ModuleManager,
}

impl Python {
    /// 创建新的 Python 运行时环境
    pub fn new() -> Result<Self> {
        Ok(Self { language: PythonLanguage::new(), vm: VM::new(), module_manager: ModuleManager::new() })
    }

    /// 执行 Python 脚本
    pub fn execute_script(&mut self, script: &str) -> Result<()> {
        // 构建 Python AST
        let builder = PythonBuilder::new(&self.language);
        let source = script;
        let edits: &[TextEdit] = &[];
        let mut cache = oak_core::parser::ParseSession::<PythonLanguage>::default();
        let build_result = builder.build(source, edits, &mut cache);

        match build_result.result {
            Ok(python_ast) => {
                println!("Building IR...");
                // 转换 AST 到 IR
                let ir = ast_to_ir(&python_ast)?;
                println!("IR built successfully");

                // 优化 IR
                let optimized_ir = optimize_ir(&ir)?;
                println!("IR optimized successfully");

                // 检查并编译热点函数
                self.vm.check_and_compile_hot_functions(&optimized_ir);

                // 转换 IR 到 VM 指令
                let vm_instructions = ir_to_vm_instructions(&optimized_ir)?;
                println!("VM instructions generated");

                // 执行 VM 指令
                self.vm.execute(&vm_instructions)?;
            }
            Err(error) => {
                return Err(PythonError::SyntaxError(format!("Parse error: {:?}", error)));
            }
        }

        Ok(())
    }

    /// 定义 Python 类
    pub fn define_class(&mut self, name: &str) -> Result<()> {
        // 获取 VM 上下文并定义类
        let mut context = self.vm.context().lock().unwrap();
        context.define_class(name);
        Ok(())
    }

    /// 定义 Python 方法
    pub fn define_method(&mut self, class: &str, name: &str, _func: Box<dyn Fn()>) -> Result<()> {
        if self.vm.define_method_in_context(class, name, Box::new(|_, _| Ok(Arc::new(PythonValue::None)))) {
            Ok(())
        }
        else {
            return Err(PythonError::NameError(format!("Class not found: {}", class)));
        }
    }

    /// 定义 Python 函数
    pub fn define_function(
        &mut self,
        name: &str,
        func: Box<dyn Fn(&mut vm::Context, Vec<Arc<PythonValue>>) -> PythonResult<Arc<PythonValue>>>,
    ) -> Result<()> {
        let mut context = self.vm.context().lock().unwrap();
        context.define_function(name, func);
        Ok(())
    }

    /// 获取全局变量
    pub fn get_global(&self, name: &str) -> Result<Arc<PythonValue>> {
        if let Some(value) = self.vm.get_global_from_context(name) {
            Ok(value.clone())
        }
        else {
            Ok(Arc::new(PythonValue::None))
        }
    }

    /// 设置全局变量
    pub fn set_global(&mut self, name: &str, value: Arc<PythonValue>) -> Result<()> {
        self.vm.set_global_in_context(name, value);
        Ok(())
    }

    /// 加载模块
    pub fn import_module(&mut self, name: &str) -> Result<Arc<PythonValue>> {
        self.module_manager.load_module(name, &mut self.vm)
    }

    /// 添加模块搜索路径
    pub fn add_module_search_path(&mut self, path: &str) {
        self.module_manager.add_search_path(path);
    }

    /// 获取已加载的模块
    pub fn get_module(&self, name: &str) -> Option<Arc<PythonValue>> {
        self.module_manager.get_module(name)
    }

    /// 列出所有已加载的模块
    pub fn list_modules(&self) -> Vec<String> {
        self.module_manager.list_modules()
    }
}

/// Implement ToPythonValue for PythonValue itself
impl ToPythonValue for PythonValue {
    fn to_python_value(&self) -> Arc<PythonValue> {
        Arc::new(self.clone())
    }
}

/// Implement ToPythonValue for common types
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

impl<T: ToPythonValue> ToPythonValue for Vec<T> {
    fn to_python_value(&self) -> Arc<PythonValue> {
        let values: Vec<_> = self.iter().map(|item| item.to_python_value()).collect();
        Arc::new(PythonValue::List(values))
    }
}

impl<K: ToString, V: ToPythonValue> ToPythonValue for std::collections::HashMap<K, V> {
    fn to_python_value(&self) -> Arc<PythonValue> {
        let mut dict = std::collections::HashMap::new();
        for (k, v) in self {
            dict.insert(k.to_string(), v.to_python_value());
        }
        Arc::new(PythonValue::Dict(dict))
    }
}
