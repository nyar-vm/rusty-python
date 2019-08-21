//! Python types for Rusty Python
//!
//! This crate provides core Python value and error types for the Rusty Python ecosystem.

#![warn(missing_docs)]

use std::sync::Arc;

/// Python value type
#[derive(Debug, Clone, PartialEq)]
pub enum PythonValue {
    /// None value
    None,
    /// Boolean value
    Boolean(bool),
    /// Integer value
    Integer(i64),
    /// Float value
    Float(f64),
    /// String value
    String(String),
    /// List value
    List(Vec<Arc<PythonValue>>),
    /// Tuple value
    Tuple(Vec<Arc<PythonValue>>),
    /// Dict value
    Dict(std::collections::HashMap<String, Arc<PythonValue>>),
    /// Object value
    Object(String, std::collections::HashMap<String, Arc<PythonValue>>),
    /// Function value
    Function(String),
}

impl PythonValue {
    /// Check if value is None
    pub fn is_none(&self) -> bool {
        matches!(self, PythonValue::None)
    }

    /// Convert value to i64
    pub fn to_i64(&self) -> i64 {
        match self {
            PythonValue::Integer(i) => *i,
            PythonValue::Float(f) => *f as i64,
            PythonValue::Boolean(b) => {
                if *b {
                    1
                }
                else {
                    0
                }
            }
            _ => 0,
        }
    }

    /// Convert value to f64
    pub fn to_f64(&self) -> f64 {
        match self {
            PythonValue::Float(f) => *f,
            PythonValue::Integer(i) => *i as f64,
            PythonValue::Boolean(b) => {
                if *b {
                    1.0
                }
                else {
                    0.0
                }
            }
            _ => 0.0,
        }
    }

    /// Convert value to bool
    pub fn to_bool(&self) -> bool {
        match self {
            PythonValue::None => false,
            PythonValue::Boolean(b) => *b,
            PythonValue::Integer(i) => *i != 0,
            PythonValue::Float(f) => *f != 0.0,
            PythonValue::String(s) => !s.is_empty(),
            PythonValue::List(l) => !l.is_empty(),
            PythonValue::Tuple(t) => !t.is_empty(),
            PythonValue::Dict(d) => !d.is_empty(),
            _ => true,
        }
    }

    /// Convert value to string
    pub fn to_string(&self) -> String {
        match self {
            PythonValue::String(s) => s.clone(),
            PythonValue::Integer(i) => i.to_string(),
            PythonValue::Float(f) => f.to_string(),
            PythonValue::Boolean(b) => b.to_string(),
            PythonValue::None => "None".to_string(),
            PythonValue::List(l) => format!("{:?}", l),
            PythonValue::Tuple(t) => format!("{:?}", t),
            PythonValue::Dict(d) => format!("{:?}", d),
            PythonValue::Object(name, _) => format!("<{} object>", name),
            PythonValue::Function(name) => format!("<function {}>", name),
        }
    }

    /// Get length of collection
    pub fn len(&self) -> usize {
        match self {
            PythonValue::List(l) => l.len(),
            PythonValue::Tuple(t) => t.len(),
            PythonValue::Dict(d) => d.len(),
            PythonValue::String(s) => s.len(),
            _ => 0,
        }
    }

    /// Get item from collection
    pub fn get_item(&self, index: &PythonValue) -> PythonResult<Arc<PythonValue>> {
        match (self, index) {
            (PythonValue::List(l), PythonValue::Integer(i)) => {
                let idx = *i as usize;
                if idx < l.len() {
                    Ok(l[idx].clone())
                }
                else {
                    Err(PythonError::IndexError(format!("list index out of range: {}", i)))
                }
            }
            (PythonValue::Tuple(t), PythonValue::Integer(i)) => {
                let idx = *i as usize;
                if idx < t.len() {
                    Ok(t[idx].clone())
                }
                else {
                    Err(PythonError::IndexError(format!("tuple index out of range: {}", i)))
                }
            }
            (PythonValue::Dict(d), PythonValue::String(key)) => {
                if let Some(value) = d.get(key) {
                    Ok(value.clone())
                }
                else {
                    Err(PythonError::KeyError(key.clone()))
                }
            }
            (PythonValue::String(s), PythonValue::Integer(i)) => {
                let idx = *i as usize;
                if idx < s.len() {
                    Ok(Arc::new(PythonValue::String(s.chars().nth(idx).unwrap().to_string())))
                }
                else {
                    Err(PythonError::IndexError(format!("string index out of range: {}", i)))
                }
            }
            _ => Err(PythonError::TypeError("unsupported operand type(s) for []".to_string())),
        }
    }

    /// Set item in collection
    pub fn set_item(&mut self, index: &PythonValue, value: Arc<PythonValue>) -> PythonResult<()> {
        match (self, index) {
            (PythonValue::List(l), PythonValue::Integer(i)) => {
                let idx = *i as usize;
                if idx < l.len() {
                    l[idx] = value;
                    Ok(())
                }
                else {
                    Err(PythonError::IndexError(format!("list assignment index out of range: {}", i)))
                }
            }
            (PythonValue::Dict(d), PythonValue::String(key)) => {
                d.insert(key.clone(), value);
                Ok(())
            }
            _ => Err(PythonError::TypeError("unsupported assignment to type".to_string())),
        }
    }

    /// Append item to list
    pub fn append(&mut self, value: Arc<PythonValue>) -> PythonResult<()> {
        match self {
            PythonValue::List(l) => {
                l.push(value);
                Ok(())
            }
            _ => Err(PythonError::TypeError("append() takes exactly one argument (0 given)".to_string())),
        }
    }

    /// Add two values
    pub fn add(&self, other: &PythonValue) -> PythonResult<PythonValue> {
        match (self, other) {
            (PythonValue::Integer(a), PythonValue::Integer(b)) => Ok(PythonValue::Integer(a + b)),
            (PythonValue::Integer(a), PythonValue::Float(b)) => Ok(PythonValue::Float((*a as f64) + *b)),
            (PythonValue::Float(a), PythonValue::Integer(b)) => Ok(PythonValue::Float(*a + (*b as f64))),
            (PythonValue::Float(a), PythonValue::Float(b)) => Ok(PythonValue::Float(*a + *b)),
            (PythonValue::String(a), PythonValue::String(b)) => Ok(PythonValue::String(format!("{}{}", a, b))),
            (PythonValue::List(a), PythonValue::List(b)) => {
                let mut result = a.clone();
                result.extend(b.clone());
                Ok(PythonValue::List(result))
            }
            (PythonValue::Tuple(a), PythonValue::Tuple(b)) => {
                let mut result = a.clone();
                result.extend(b.clone());
                Ok(PythonValue::Tuple(result))
            }
            _ => Err(PythonError::TypeError("unsupported operand type(s) for +".to_string())),
        }
    }

    /// Subtract two values
    pub fn sub(&self, other: &PythonValue) -> PythonResult<PythonValue> {
        match (self, other) {
            (PythonValue::Integer(a), PythonValue::Integer(b)) => Ok(PythonValue::Integer(a - b)),
            (PythonValue::Integer(a), PythonValue::Float(b)) => Ok(PythonValue::Float((*a as f64) - *b)),
            (PythonValue::Float(a), PythonValue::Integer(b)) => Ok(PythonValue::Float(*a - (*b as f64))),
            (PythonValue::Float(a), PythonValue::Float(b)) => Ok(PythonValue::Float(*a - *b)),
            _ => Err(PythonError::TypeError("unsupported operand type(s) for -".to_string())),
        }
    }

    /// Multiply two values
    pub fn mul(&self, other: &PythonValue) -> PythonResult<PythonValue> {
        match (self, other) {
            (PythonValue::Integer(a), PythonValue::Integer(b)) => Ok(PythonValue::Integer(a * b)),
            (PythonValue::Integer(a), PythonValue::Float(b)) => Ok(PythonValue::Float((*a as f64) * *b)),
            (PythonValue::Float(a), PythonValue::Integer(b)) => Ok(PythonValue::Float(*a * (*b as f64))),
            (PythonValue::Float(a), PythonValue::Float(b)) => Ok(PythonValue::Float(*a * *b)),
            (PythonValue::String(a), PythonValue::Integer(b)) => {
                if *b >= 0 {
                    Ok(PythonValue::String(a.repeat(*b as usize)))
                }
                else {
                    Ok(PythonValue::String("".to_string()))
                }
            }
            (PythonValue::List(a), PythonValue::Integer(b)) => {
                if *b >= 0 {
                    let mut result = Vec::new();
                    for _ in 0..*b {
                        result.extend(a.clone());
                    }
                    Ok(PythonValue::List(result))
                }
                else {
                    Ok(PythonValue::List(Vec::new()))
                }
            }
            (PythonValue::Tuple(a), PythonValue::Integer(b)) => {
                if *b >= 0 {
                    let mut result = Vec::new();
                    for _ in 0..*b {
                        result.extend(a.clone());
                    }
                    Ok(PythonValue::Tuple(result))
                }
                else {
                    Ok(PythonValue::Tuple(Vec::new()))
                }
            }
            _ => Err(PythonError::TypeError("unsupported operand type(s) for *".to_string())),
        }
    }

    /// Divide two values
    pub fn div(&self, other: &PythonValue) -> PythonResult<PythonValue> {
        match (self, other) {
            (_, PythonValue::Integer(0)) | (_, PythonValue::Float(0.0)) => {
                Err(PythonError::ZeroDivisionError("division by zero".to_string()))
            }
            (PythonValue::Integer(a), PythonValue::Integer(b)) => Ok(PythonValue::Float((*a as f64) / (*b as f64))),
            (PythonValue::Integer(a), PythonValue::Float(b)) => Ok(PythonValue::Float((*a as f64) / *b)),
            (PythonValue::Float(a), PythonValue::Integer(b)) => Ok(PythonValue::Float(*a / (*b as f64))),
            (PythonValue::Float(a), PythonValue::Float(b)) => Ok(PythonValue::Float(*a / *b)),
            _ => Err(PythonError::TypeError("unsupported operand type(s) for /".to_string())),
        }
    }

    /// Check equality
    pub fn eq(&self, other: &PythonValue) -> bool {
        match (self, other) {
            (PythonValue::Integer(a), PythonValue::Float(b)) => *a as f64 == *b,
            (PythonValue::Float(a), PythonValue::Integer(b)) => *a == *b as f64,
            (PythonValue::Integer(a), PythonValue::Integer(b)) => a == b,
            (PythonValue::Float(a), PythonValue::Float(b)) => a == b,
            (PythonValue::Boolean(a), PythonValue::Boolean(b)) => a == b,
            (PythonValue::String(a), PythonValue::String(b)) => a == b,
            (PythonValue::List(a), PythonValue::List(b)) => a == b,
            (PythonValue::Tuple(a), PythonValue::Tuple(b)) => a == b,
            (PythonValue::Dict(a), PythonValue::Dict(b)) => a == b,
            (PythonValue::None, PythonValue::None) => true,
            (PythonValue::Function(a), PythonValue::Function(b)) => a == b,
            (PythonValue::Object(a_name, a_attrs), PythonValue::Object(b_name, b_attrs)) => {
                a_name == b_name && a_attrs == b_attrs
            }
            _ => false,
        }
    }

    /// Check less than
    pub fn lt(&self, other: &PythonValue) -> PythonResult<bool> {
        match (self, other) {
            (PythonValue::Integer(a), PythonValue::Integer(b)) => Ok(a < b),
            (PythonValue::Integer(a), PythonValue::Float(b)) => Ok((*a as f64) < *b),
            (PythonValue::Float(a), PythonValue::Integer(b)) => Ok(*a < (*b as f64)),
            (PythonValue::Float(a), PythonValue::Float(b)) => Ok(a < b),
            (PythonValue::String(a), PythonValue::String(b)) => Ok(a < b),
            _ => Err(PythonError::TypeError("unorderable types".to_string())),
        }
    }

    /// Check greater than
    pub fn gt(&self, other: &PythonValue) -> PythonResult<bool> {
        match (self, other) {
            (PythonValue::Integer(a), PythonValue::Integer(b)) => Ok(a > b),
            (PythonValue::Integer(a), PythonValue::Float(b)) => Ok((*a as f64) > *b),
            (PythonValue::Float(a), PythonValue::Integer(b)) => Ok(*a > (*b as f64)),
            (PythonValue::Float(a), PythonValue::Float(b)) => Ok(a > b),
            (PythonValue::String(a), PythonValue::String(b)) => Ok(a > b),
            _ => Err(PythonError::TypeError("unorderable types".to_string())),
        }
    }
}

/// Python error type
#[derive(Debug, Clone, PartialEq)]
pub enum PythonError {
    /// Method not found error
    MethodNotFound(String),
    /// Attribute not found error
    AttributeNotFound(String),
    /// Attribute error
    AttributeError(String),
    /// Lexical analysis error
    LexicalError(String),
    /// Syntax analysis error
    SyntaxError(String),
    /// Runtime error
    RuntimeError(String),
    /// Type error
    TypeError(String),
    /// Argument error
    ArgumentError(String),
    /// Name error
    NameError(String),
    /// Index error
    IndexError(String),
    /// Key error
    KeyError(String),
    /// Zero division error
    ZeroDivisionError(String),
    /// IO error
    IOError(String),
    /// Import error
    ImportError(String),
    /// Value error
    ValueError(String),
}

impl std::fmt::Display for PythonError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PythonError::MethodNotFound(name) => write!(f, "Method not found: {}", name),
            PythonError::AttributeNotFound(name) => write!(f, "Attribute not found: {}", name),
            PythonError::AttributeError(msg) => write!(f, "Attribute error: {}", msg),
            PythonError::LexicalError(msg) => write!(f, "Lexical error: {}", msg),
            PythonError::SyntaxError(msg) => write!(f, "Syntax error: {}", msg),
            PythonError::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
            PythonError::TypeError(msg) => write!(f, "Type error: {}", msg),
            PythonError::ArgumentError(msg) => write!(f, "Argument error: {}", msg),
            PythonError::NameError(msg) => write!(f, "Name error: {}", msg),
            PythonError::IndexError(msg) => write!(f, "Index error: {}", msg),
            PythonError::KeyError(msg) => write!(f, "Key error: {}", msg),
            PythonError::ZeroDivisionError(msg) => write!(f, "ZeroDivisionError: {}", msg),
            PythonError::IOError(msg) => write!(f, "IO error: {}", msg),
            PythonError::ImportError(msg) => write!(f, "Import error: {}", msg),
            PythonError::ValueError(msg) => write!(f, "Value error: {}", msg),
        }
    }
}

impl std::error::Error for PythonError {}

/// Python result type
pub type PythonResult<T> = std::result::Result<T, PythonError>;
