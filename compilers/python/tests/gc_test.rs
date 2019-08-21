//! Tests for Python garbage collector

use python::{Allocator, GC};
use python_types::PythonValue;
use std::sync::Arc;

#[test]
fn test_gc_basic() {
    // Create a new GC
    let gc = Arc::new(std::sync::Mutex::new(GC::new()));
    let allocator = Allocator::new(gc.clone());

    // Allocate some objects
    let obj1 = allocator.allocate(PythonValue::Integer(42));
    let obj2 = allocator.allocate(PythonValue::String("test".to_string()));

    // Allocate a root object that references other objects
    let list = allocator.allocate_root(PythonValue::List(vec![obj1, obj2]));

    // Trigger garbage collection
    allocator.collect();

    // The objects should still be alive because they're referenced by the root
    assert_eq!(gc.lock().unwrap().get_objects_count(), 3);
}

#[test]
fn test_gc_collect_unused() {
    // Create a new GC
    let gc = Arc::new(std::sync::Mutex::new(GC::new()));
    let allocator = Allocator::new(gc.clone());

    // Allocate some objects
    let obj1 = allocator.allocate(PythonValue::Integer(42));
    let obj2 = allocator.allocate(PythonValue::String("test".to_string()));

    // Don't reference these objects from any root
    drop(obj1);
    drop(obj2);

    // Trigger garbage collection
    allocator.collect();

    // The objects should be collected
    assert_eq!(gc.lock().unwrap().get_objects_count(), 0);
}

#[test]
fn test_gc_circular_reference() {
    // Create a new GC
    let gc = Arc::new(std::sync::Mutex::new(GC::new()));
    let allocator = Allocator::new(gc.clone());

    // Create circular reference
    let mut list1 = allocator.allocate(PythonValue::List(vec![]));
    let list2 = allocator.allocate(PythonValue::List(vec![list1.clone()]));

    // Make list1 reference list2, creating a cycle
    let list1_mut = Arc::make_mut(&mut list1);
    if let PythonValue::List(items) = list1_mut {
        items.push(list2.clone());
    }

    // Don't reference these objects from any root
    drop(list1);
    drop(list2);

    // Trigger garbage collection
    allocator.collect();

    // The circular reference should be collected
    assert_eq!(gc.lock().unwrap().get_objects_count(), 0);
}
