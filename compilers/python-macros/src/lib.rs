//! Python macro support for Rusty Python
//!
//! This crate provides macro support for Python, allowing Rust code to interact
//! with Python code more seamlessly.

#![warn(missing_docs)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

/// Macro to define a Python function
#[proc_macro_attribute]
pub fn python_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let body = &input.block;
    let vis = &input.vis;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;

    let expanded = quote! {
        #vis fn #name(#inputs) #output {
            // Python function wrapper
            #body
        }
    };

    TokenStream::from(expanded)
}
