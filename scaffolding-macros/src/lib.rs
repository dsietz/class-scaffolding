use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Parser, Result};
use syn::{parse, parse_macro_input, punctuated::Punctuated, ItemStruct, LitStr, Token};

#[proc_macro_attribute]
pub fn as_entity(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(args as parse::Nothing);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        // The unique identifier of the object
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { id: String })
                .unwrap(),
        );
        // The timestamp when the object was created
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { created_dtm: i64 })
                .unwrap(),
        );
        // The timestamp when the object was last modified
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { modified_dtm: i64 })
                .unwrap(),
        );
        // The timestamp when the object is no longer active
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { inactive_dtm: i64 })
                .unwrap(),
        );
        // The timestamp when the object is expired
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { expired_dtm: i64 })
                .unwrap(),
        );
    }

    return quote! {
        #item_struct
    }
    .into();
}

#[derive(Debug)]
struct Args {
    pub vars: Vec<LitStr>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let vars = Punctuated::<syn::LitStr, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            vars: vars.into_iter().collect::<Vec<LitStr>>(),
        })
    }
}

#[proc_macro_derive(Scaffolding)]
pub fn scaffolding_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_scaffolding(&ast)
}

fn impl_scaffolding(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Scaffolding for #name {
            // fn hello(&self) {
            //     println!("Hello, My name is {}!", stringify!(#name));
            //     println!("My id is {}", self.id);
            // }
        }
    };
    gen.into()
}
