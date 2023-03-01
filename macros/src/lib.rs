use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Fields::*};

/**
 * This macro searches for the first instance of a field that equals any
 * of the literals in the search list, and uses it's implementation of
 * the trait for the struct. If this first instance does not implement
 *  the trait being derived, the code will not compile.
 *
 * Search list: [
 *     "super_class",
 *     "superclass",
 *     "parent",
 *     "parent_class"
 * ]
 *
 * Tuple structs and Unit structs win ll be given the traits default implementaiton
 * A future implementation will implement the derive macro for Tuple structs
 * that contain the super/parent class behing "inherited".
 * A future implementation might also check if the fields implement the required
 * trait first. If that happens, the search list can be expanded so that
 * these names could refer to other fields within the struct
 */
#[proc_macro_derive(Resource)]
pub fn resource_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_resource_macro(&ast)
}

fn impl_resource_macro(ast: &syn::DeriveInput) -> TokenStream {
    let field_search_list = [
        "super_class",
        "superclass",
        "parent",
        "parent_class",
        "inner",
    ];
    let name = &ast.ident;
    let field = &ast.data;
    if let syn::Data::Struct(struct_data) = field {
        match struct_data.fields {
            Named(ref named_field) => {
                for field in &named_field.named {
                    let field_ident = &field.ident;
                    // check if field name == "super_class"
                    let field_ident_string = field_ident.as_ref().unwrap().to_string();
                    if let None = field_search_list
                        .iter()
                        .find(|f| f == &&field_ident_string.as_str())
                    {
                        continue;
                    }

                    // Code the macro generates if "super_class" field exists
                    let gen = quote! {
                        impl Resource for #name {
                            fn href(&self) -> Option<String>{
                                self.#field_ident.href()
                            }
                        }
                    };
                    return gen.into();
                }
                panic!("Could not find \"super_class\" field that implements the Resource trait in struct {}", name.to_string());
            }
            // resort to default implementation if a Unit struct or Tuple struct
            _ => {
                let gen = quote! {
                    impl Resource for #name {}
                };
                return gen.into();
            }
        }
    } else {
        // support for enums goes here
        unimplemented!(
            "Need to add support for Tuple structs here. 
        Currently, only Structs with named fields can be derived with this macro"
        );
    }
}

#[proc_macro_derive(Link)]
pub fn link_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_link_macro(&ast)
}

fn impl_link_macro(ast: &syn::DeriveInput) -> TokenStream {
    let field_search_list = [
        "super_class",
        "superclass",
        "parent",
        "parent_class",
        "inner",
    ];
    let name = &ast.ident;
    let field = &ast.data;
    if let syn::Data::Struct(struct_data) = field {
        match struct_data.fields {
            Named(ref named_field) => {
                for field in &named_field.named {
                    let field_ident = &field.ident;
                    // check if field name == "super_class"
                    let field_ident_string = field_ident.as_ref().unwrap().to_string();
                    if let None = field_search_list
                        .iter()
                        .find(|f| f == &&field_ident_string.as_str())
                    {
                        continue;
                    }

                    // Code the macro generates if "super_class" field exists
                    let gen = quote! {
                        impl Link for #name {
                            fn href(&self) -> String {
                                self.#field_ident.href()
                            }
                        }
                    };
                    return gen.into();
                }
                panic!("Could not find \"super_class\" field that implements the Resource trait in struct {}", name.to_string());
            }
            // resort to default implementation if a Unit struct or Tuple struct
            _ => {
                panic!("No default implementation for Link Trait");
            }
        }
    } else {
        // support for enums goes here
        unimplemented!(
            "Need to add support for Tuple structs here. 
        Currently, only Structs with named fields can be derived with this macro"
        );
    }
}
