//! Tests for Python types

use python_types::{PythonError, PythonResult, PythonValue};
use std::sync::Arc;

#[test]
fn test_python_value_creation() {
    // Test None value
    let none = PythonValue::None;
    assert!(none.is_none());

    // Test boolean values
    let true_val = PythonValue::Boolean(true);
    let false_val = PythonValue::Boolean(false);
    assert_eq!(true_val.to_bool(), true);
    assert_eq!(false_val.to_bool(), false);

    // Test integer values
    let int_val = PythonValue::Integer(42);
    assert_eq!(int_val.to_i64(), 42);
    assert_eq!(int_val.to_f64(), 42.0);

    // Test float values
    let float_val = PythonValue::Float(3.14);
    assert_eq!(float_val.to_f64(), 3.14);
    assert_eq!(float_val.to_i64(), 3);

    // Test string values
    let string_val = PythonValue::String("hello".to_string());
    assert_eq!(string_val.to_string(), "hello");

    // Test list values
    let list_val = PythonValue::List(vec![
        Arc::new(PythonValue::Integer(1)),
        Arc::new(PythonValue::Integer(2)),
        Arc::new(PythonValue::Integer(3)),
    ]);
    assert!(list_val.to_bool()); // Non-empty list should be true
    let empty_list_val = PythonValue::List(vec![]);
    assert!(!empty_list_val.to_bool()); // Empty list should be false

    // Test dict values
    let mut dict = std::collections::HashMap::new();
    dict.insert("key".to_string(), Arc::new(PythonValue::String("value".to_string())));
    let dict_val = PythonValue::Dict(dict);
    assert!(dict_val.to_bool()); // Non-empty dict should be true
    let empty_dict_val = PythonValue::Dict(std::collections::HashMap::new());
    assert!(!empty_dict_val.to_bool()); // Empty dict should be false

    // Test object values
    let mut obj_attrs = std::collections::HashMap::new();
    obj_attrs.insert("name".to_string(), Arc::new(PythonValue::String("test".to_string())));
    let obj_val = PythonValue::Object("Test".to_string(), obj_attrs);
    assert_eq!(obj_val.to_string(), "<Test object>");

    // Test function values
    let func_val = PythonValue::Function("test_func".to_string());
    assert_eq!(func_val.to_string(), "<function test_func>");
}

#[test]
fn test_python_value_conversions() {
    // Test to_bool conversion
    assert_eq!(PythonValue::None.to_bool(), false);
    assert_eq!(PythonValue::Boolean(true).to_bool(), true);
    assert_eq!(PythonValue::Boolean(false).to_bool(), false);
    assert_eq!(PythonValue::Integer(0).to_bool(), false);
    assert_eq!(PythonValue::Integer(1).to_bool(), true);
    assert_eq!(PythonValue::Float(0.0).to_bool(), false);
    assert_eq!(PythonValue::Float(0.1).to_bool(), true);
    assert_eq!(PythonValue::String("".to_string()).to_bool(), false);
    assert_eq!(PythonValue::String("hello".to_string()).to_bool(), true);

    // Test to_string conversion
    assert_eq!(PythonValue::None.to_string(), "None");
    assert_eq!(PythonValue::Boolean(true).to_string(), "true");
    assert_eq!(PythonValue::Boolean(false).to_string(), "false");
    assert_eq!(PythonValue::Integer(42).to_string(), "42");
    assert_eq!(PythonValue::Float(3.14).to_string(), "3.14");
}

#[test]
fn test_python_error_display() {
    let errors = vec![
        PythonError::MethodNotFound("test_method".to_string()),
        PythonError::AttributeNotFound("test_attr".to_string()),
        PythonError::LexicalError("Invalid syntax".to_string()),
        PythonError::SyntaxError("Syntax error".to_string()),
        PythonError::RuntimeError("Runtime error".to_string()),
        PythonError::TypeError("Type error".to_string()),
        PythonError::ArgumentError("Argument error".to_string()),
        PythonError::NameError("Name error".to_string()),
        PythonError::IndexError("Index error".to_string()),
        PythonError::KeyError("Key error".to_string()),
        PythonError::ZeroDivisionError("Division by zero".to_string()),
    ];

    for error in errors {
        let error_str = error.to_string();
        assert!(!error_str.is_empty());
    }
}
