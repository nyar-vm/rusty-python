//! Tests for Python IR

use oak_core::{TextEdit, builder::Builder, parser::ParseSession};
use oak_python::{PythonBuilder, PythonLanguage};
use python_ir::{Function, Instruction, Module, ast_to_ir, optimize_ir};
use python_types::PythonValue;

#[test]
fn test_ast_to_ir() {
    // Create a simple Python script
    let script = "x = 1 + 2\nprint(x)";

    // Build AST
    let language = PythonLanguage::new();
    let builder = PythonBuilder::new(&language);
    let source = script;
    let edits: &[oak_core::TextEdit] = &[];
    let mut cache = ParseSession::<PythonLanguage>::default();
    let build_result = builder.build(source, edits, &mut cache);

    assert!(build_result.result.is_ok());
    let python_ast = build_result.result.unwrap();

    // Convert to IR
    let ir = ast_to_ir(&python_ast).unwrap();

    // Check module structure
    assert_eq!(ir.name, "__main__");
    assert!(!ir.functions.is_empty());

    // Check main function
    let main_function = &ir.functions[0];
    assert_eq!(main_function.name, "__main__");
    assert!(!main_function.instructions.is_empty());
}

#[test]
fn test_optimize_ir() {
    // Create a simple IR module
    let module = Module {
        name: "test".to_string(),
        functions: vec![Function {
            name: "test".to_string(),
            params: vec![],
            instructions: vec![
                Instruction::LoadConst(PythonValue::Integer(1)),
                Instruction::LoadConst(PythonValue::Integer(2)),
                Instruction::Add,
                Instruction::Return,
            ],
            locals: vec![],
        }],
        globals: vec![],
    };

    // Optimize IR
    let optimized_module = optimize_ir(&module).unwrap();

    // Check that optimization doesn't break the module
    assert_eq!(optimized_module.name, module.name);
    assert_eq!(optimized_module.functions.len(), module.functions.len());
}
