#[macro_use]
extern crate serde;
extern crate serde_derive;

// extern crate log;
// extern crate derive;
// extern crate proc_macro

// use proc_macro::TokenStream;
// use syn::{parse_macro_input, DeriveInput, parse::Parser};
// use quote::quote;

// /** reference material
//  * - https://github.com/eonm-abes/proc-macro-issue-minimal-example/tree/solution)
//  * - https://users.rust-lang.org/t/solved-derive-and-proc-macro-add-field-to-an-existing-struct/52307/3
// */
// #[proc_macro_attribute]
// pub fn add_field(_args: TokenStream, input: TokenStream) -> TokenStream  {
//     let mut ast = parse_macro_input!(input as DeriveInput);
//     match &mut ast.data {
//         syn::Data::Struct(ref mut struct_data) => {
//             match &mut struct_data.fields {
//                 syn::Fields::Named(fields) => {
//                     fields
//                         .named
//                         .push(syn::Field::parse_named.parse2(quote! { pub a: String }).unwrap());
//                 }
//                 _ => {
//                     ()
//                 }
//             }

//             return quote! {
//                 #ast
//             }.into();
//         }
//         _ => panic!("`add_field` has to be used with structs "),
//     }
// }

pub mod reference;
// pub mod scaffolding;
