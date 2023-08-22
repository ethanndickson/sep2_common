use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(SEEvent)]
pub fn derive_event(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SEEvent for #ident {

        }
    };
    output.into()
}

#[proc_macro_derive(SESubscribableIdentifiedObject)]
pub fn derive_subident(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SESubscribableIdentifiedObject for #ident {

        }
    };
    output.into()
}

#[proc_macro_derive(SERespondableResource)]
pub fn derive_respres(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SESubscribableIdentifiedObject for #ident {

        }
    };
    output.into()
}
