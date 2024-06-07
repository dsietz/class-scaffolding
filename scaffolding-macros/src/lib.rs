use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Parser, Result};
use syn::Expr::Struct;
use syn::FieldValue;
use syn::Member;
use syn::{parse_macro_input, parse_quote, punctuated::Punctuated, ItemStruct, LitStr, Token};
// use serde::Serialize;

static ADDRESS: &str = "addresses";
static METADATA: &str = "metadata";
static NOTES: &str = "notes";
static TAGS: &str = "tags";
static CORE_ATTRS: [&str; 6] = [
    "id",
    "created_dtm",
    "modified_dtm",
    "inactive_dtm",
    "expired_dtm",
    "activity",
];

/// Modifying a struct
///
/// Dynammically adds the following public attributes to the struct
/// + id: String
/// + created_dtm: i64
/// + modified_dtm: i64
/// + inactive_dtm: i64
/// + expired_dtm: i64
/// + activity: Vec<ActivityItem>
///
/// Optionally
/// + addresses: Vec<Address>
/// + metadata: BTreeMap<String, String>
/// + notes: BTreeMap<String, Note>
/// + tags: Vec<String>
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
                .parse2(quote! { pub id: String })
                .unwrap(),
        );
        // The timestamp when the object was created
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub created_dtm: i64 })
                .unwrap(),
        );
        // The timestamp when the object was last modified
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub modified_dtm: i64 })
                .unwrap(),
        );
        // The timestamp when the object is no longer active
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub inactive_dtm: i64 })
                .unwrap(),
        );
        // The timestamp when the object is expired
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub expired_dtm: i64 })
                .unwrap(),
        );

        // The list of activity performed on the object
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub activity: Vec<ActivityItem> })
                .unwrap(),
        );

        // optional attributes
        match attrs.contains(&ADDRESS.to_string()) {
            true => {
                // The metadata handler
                fields.named.push(
                    syn::Field::parse_named
                        .parse2(quote! { pub addresses: Vec<Address> })
                        .unwrap(),
                );
            }
            false => {}
        }

        // optional attributes
        match attrs.contains(&METADATA.to_string()) {
            true => {
                // The metadata handler
                fields.named.push(
                    syn::Field::parse_named
                        .parse2(quote! { pub metadata: BTreeMap<String, String> })
                        .unwrap(),
                );
            }
            false => {}
        }

        // optional attributes
        match attrs.contains(&NOTES.to_string()) {
            true => {
                // The notes handler
                fields.named.push(
                    syn::Field::parse_named
                        .parse2(quote! { pub notes: BTreeMap<String, Note> })
                        .unwrap(),
                );
            }
            false => {}
        }

        // optional attributes
        match attrs.contains(&TAGS.to_string()) {
            true => {
                // The tags handler
                fields.named.push(
                    syn::Field::parse_named
                        .parse2(quote! { pub tags: Vec<String> })
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

// Addresses Trait
#[proc_macro_derive(ScaffoldingAddresses)]
pub fn scaffolding_addresses_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    impl_scaffolding_addresses(&ast)
}

fn impl_scaffolding_addresses(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl ScaffoldingAddresses for #name {
            fn add_address(
                &mut self,
                category: String,
                line_1: String,
                line_2: String,
                line_3: String,
                line_4: String,
                country_code: String,
            ) -> Address {
                let address = Address::new(category, line_1, line_2, line_3, line_4, country_code);
                self.addresses.push(address.clone());
                return address;
            }

            fn addresses_by_category(&self, category: String) -> Vec<&Address> {
                self.addresses
                    .iter()
                    .filter(|a| a.category == category)
                    .collect()
            }

            fn remove_address(&mut self, id: String) {
                match self.addresses.iter().position(|a| a.id == id) {
                    Some(idx) => {
                        self.addresses.remove(idx);
                    }
                    None => {
                        // do nothing
                    }
                }
            }
        }
    };
    gen.into()
}

// Notes Trait
#[proc_macro_derive(ScaffoldingNotes)]
pub fn scaffolding_notes_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    impl_scaffolding_notes(&ast)
}

fn impl_scaffolding_notes(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl ScaffoldingNotes for #name {
            fn get_note(&self, id: String) -> Option<&Note> {
                self.notes.get(&id)
            }

            fn insert_note(&mut self, auth: String, cont: Vec<u8>, acc: Option<String>) -> String {
                let note = Note::new(auth, cont, acc);
                let id = note.id.clone();
                self.notes.insert(id.clone(), note);
                id
            }

            fn modify_note(&mut self, id: String, auth: String, cont: Vec<u8>, acc: Option<String>) {
                self.notes
                    .entry(id)
                    .and_modify(|note|
                        note.update(auth, cont, acc)
                    );
            }

            fn search_notes(&mut self, search: String) -> Vec<Note> {
                let mut results: Vec<Note> = Vec::new();

                for (key, note) in self.notes.iter() {
                    let mut cont = String::from_utf8(note.content.clone())
                    .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
                    .unwrap();

                    match cont.contains(&search) {
                        true => {
                            results.push(note.clone())
                        },
                        false => {},
                    }
                }

                results
            }

            fn remove_note(&mut self, id: String) {
                self.notes.remove(&id);
            }
        }
    };
    gen.into()
}

// Tagging Trait
#[proc_macro_derive(ScaffoldingTags)]
pub fn scaffolding_tags_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    impl_scaffolding_tags(&ast)
}

fn impl_scaffolding_tags(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl ScaffoldingTags for #name {
            fn add_tag(&mut self, tag: String) {
                // don't add duplicates
                match self.has_tag(tag.clone()) {
                    false => {
                        self.tags.push(tag);
                    },
                    true => {
                        println!("Ignoring tag {}. Tag already exists!", tag);
                    },
                }
            }
            fn has_tag(&self, tag: String) -> bool {
                let results = self.tags.iter().filter(|t| **t == tag).cloned().collect::<String>();
                match results.len() {
                    0 => false,
                    _ => true,
                }
            }
            fn remove_tag(&mut self, tag: String) {
                let pos = self.tags.iter().position(|t| **t == tag).unwrap();
                self.tags.remove(pos);
            }
        }
    };
    gen.into()
}

///
/// Modifies the following functions
/// + new - Adds the core attributes to the new struct using the defined or default values
///
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

                            match attrs.contains(&ADDRESS.to_string()) {
                                true => {
                                    modify_attr_list.push(&ADDRESS);
                                }
                                _ => {}
                            }

                            match attrs.contains(&METADATA.to_string()) {
                                true => {
                                    modify_attr_list.push(&METADATA);
                                }
                                _ => {}
                            }

                            match attrs.contains(&NOTES.to_string()) {
                                true => {
                                    modify_attr_list.push(&NOTES);
                                }
                                _ => {}
                            }

                            match attrs.contains(&TAGS.to_string()) {
                                true => {
                                    modify_attr_list.push(&TAGS);
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
                                    "notes" => {
                                        let line: FieldValue =
                                            parse_quote! {notes: BTreeMap::new()};
                                        expr_struct.fields.insert(0, line);
                                    }
                                    "tags" => {
                                        let line: FieldValue = parse_quote! {tags: Vec::new()};
                                        expr_struct.fields.insert(0, line);
                                    }
                                    "addresses" => {
                                        let line: FieldValue = parse_quote! {addresses: Vec::new()};
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
