//! Test parser example

use python_types::{PythonValue, PythonError, PythonResult};
use python_ir::{Module, ast_to_ir, optimize_ir};
use oak_python::{PythonLanguage, PythonBuilder};
use oak_core::{parser::ParseSession, TextEdit};

fn main() -> PythonResult<()> {
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
