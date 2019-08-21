//! Tests for Python parser

use oak_core::{Builder, TextEdit, parser::ParseSession};
use oak_python::{PythonBuilder, PythonLanguage};
use python_ir::{ast_to_ir, optimize_ir};
use python_types::{PythonError, PythonResult};

#[test]
fn test_parser() -> PythonResult<()> {
    // Test Python code
    let python_code = r#"
    def hello():
        print("Hello, World!")
        return 42
    
    result = hello()
    print(result)
    "#;

    // Build AST
    let language = PythonLanguage::new();
    let builder = PythonBuilder::new(&language);
    let edits: &[TextEdit] = &[];
    let mut cache = ParseSession::<PythonLanguage>::default();
    let build_result = builder.build(python_code, edits, &mut cache);

    match build_result.result {
        Ok(python_ast) => {
            println!("AST built successfully");

            // Convert AST to IR
            let ir = ast_to_ir(&python_ast)?;
            println!("IR built successfully: {:?}", ir);

            // Optimize IR
            let optimized_ir = optimize_ir(&ir)?;
            println!("IR optimized successfully: {:?}", optimized_ir);
        }
        Err(error) => {
            println!("Parse error: {:?}", error);
            return Err(PythonError::SyntaxError(format!("Parse error: {:?}", error)));
        }
    }

    Ok(())
}
