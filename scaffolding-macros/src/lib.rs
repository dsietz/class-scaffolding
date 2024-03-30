use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Parser, Result};
use syn::Expr::Struct;
use syn::FieldValue;
use syn::Member;
use syn::{parse_macro_input, parse_quote, punctuated::Punctuated, ItemStruct, LitStr, Token};
// use serde::Serialize;

static METADATA: &str = "metadata";
static CORE_ATTRS: [&str; 6] = [
    "id",
    "created_dtm",
    "modified_dtm",
    "inactive_dtm",
    "expired_dtm",
    "activity",
];

///
/// Modifying a struct
///
#[proc_macro_attribute]
pub fn scaffolding_struct(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct: ItemStruct = parse_macro_input!(input as ItemStruct);
    let attrs = parse_macro_input!(args as Args)
        .vars
        .iter()
        .map(|a| a.value())
        .collect::<Vec<_>>();

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

        // The list of activity performed on the object
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { activity: Vec<ActivityItem> })
                .unwrap(),
        );

        // optional attributes
        match attrs.contains(&METADATA.to_string()) {
            true => {
                // The metadata handler
                fields.named.push(
                    syn::Field::parse_named
                        .parse2(quote! { metadata: BTreeMap<String, String> })
                        .unwrap(),
                );
            }
            false => {}
        }
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

///
/// Implementing the Traits
///
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
            fn get_activity(&self, name: String) -> Vec<ActivityItem>{
                self.activity.iter().filter(|a| a.action == name).cloned().collect()
            }

            fn log_activity(&mut self, name: String, descr: String) {
                self.activity.push(ActivityItem::new(name, descr));
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn scaffolding_fn(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item: syn::Item = syn::parse(input).unwrap();
    let fn_item = match &mut item {
        syn::Item::Fn(fn_item) => fn_item,
        _ => panic!("expected fn"),
    };
    let attrs = parse_macro_input!(args as Args)
        .vars
        .iter()
        .map(|a| a.value())
        .collect::<Vec<_>>();

    // get the name of the method
    let name = &fn_item.sig.ident.to_string();

    match name.as_ref() {
        "new" => {
            print!("Modifying function {} ...", name);
            // find the line that sets the id attribute
            for s in 0..fn_item.block.stmts.len() {
                match &mut fn_item.block.stmts[s] {
                    syn::Stmt::Expr(expr, None) => match expr {
                        Struct(expr_struct) => {
                            // println!("Found a Struct!");
                            let mut modify_attr_list = vec![
                                "id",
                                "created_dtm",
                                "modified_dtm",
                                "inactive_dtm",
                                "expired_dtm",
                                "activity",
                            ];

                            match attrs.contains(&METADATA.to_string()) {
                                true => {
                                    modify_attr_list.push(&METADATA);
                                }
                                _ => {}
                            }
                            // first determine if the attributes already exist
                            for f in 0..expr_struct.fields.len() {
                                match &expr_struct.fields[f].member {
                                    Member::Named(mbr) => {
                                        match CORE_ATTRS.contains(&mbr.to_string().as_str()) {
                                            true => {
                                                // core attribute already set, so don't need to add it
                                                // println!("Ignoring attribute {}", mbr.to_string());
                                                modify_attr_list
                                                    .retain_mut(|a| *a != mbr.to_string().as_str());
                                            }
                                            false => {}
                                        }
                                    }
                                    _ => {}
                                }
                            }

                            // then, add the missing attributes
                            for attr in modify_attr_list.iter() {
                                // println!("Adding attribute {}", attr);
                                match *attr {
                                    "id" => {
                                        let line: FieldValue = parse_quote! {id: defaults::id()};
                                        expr_struct.fields.insert(0, line);
                                    }
                                    "created_dtm" => {
                                        let line: FieldValue =
                                            parse_quote! {created_dtm: defaults::now()};
                                        expr_struct.fields.insert(0, line);
                                    }
                                    "modified_dtm" => {
                                        let line: FieldValue =
                                            parse_quote! {modified_dtm: defaults::now()};
                                        expr_struct.fields.insert(0, line);
                                    }
                                    "inactive_dtm" => {
                                        let line: FieldValue = parse_quote! {inactive_dtm: defaults::add_days(defaults::now(), 90)};
                                        expr_struct.fields.insert(0, line);
                                    }
                                    "expired_dtm" => {
                                        let line: FieldValue = parse_quote! {expired_dtm: defaults::add_years(defaults::now(), 3)};
                                        expr_struct.fields.insert(0, line);
                                    }
                                    "activity" => {
                                        let line: FieldValue = parse_quote! {activity: Vec::new()};
                                        expr_struct.fields.insert(0, line);
                                    }
                                    "metadata" => {
                                        let line: FieldValue =
                                            parse_quote! {metadata: BTreeMap::new()};
                                        expr_struct.fields.insert(0, line);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {
                            // println!("Not an Struct!");
                        }
                    },
                    _ => {
                        // println!("Not an Expr!");
                    }
                }
            }
        }
        _ => {
            print!(
                "Function {} is unsupported. Nothing to add to function ",
                name
            );
        }
    }

    item.into_token_stream().into()
}
