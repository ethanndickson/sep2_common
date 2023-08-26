use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(SEResource)]
pub fn derive_resource(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, generics, ..
    } = parse_macro_input!(input);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let output = quote! {
        impl #impl_generics SEResource for #ident #ty_generics #where_clause {}
    };
    output.into()
}

#[proc_macro_derive(SELink)]
pub fn derive_link(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SELink for #ident {}
    };
    output.into()
}

#[proc_macro_derive(SEResponse)]
pub fn derive_response(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SEResponse for #ident {}
    };
    output.into()
}

#[proc_macro_derive(SEIdentifiedObject)]
pub fn derive_identified_object(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SEIdentifiedObject for #ident {}
    };
    output.into()
}

#[proc_macro_derive(SERespondableResource)]
pub fn derive_respondable_resource(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SERespondableResource for #ident {
            fn reply_to(&self) -> Option<&str> {
                self.reply_to.as_ref().map(|s| s.as_str())
            }

            fn response_required(&self) -> Option<HexBinary8> {
                self.response_required
            }
        }
    };
    output.into()
}

#[proc_macro_derive(SESubscriptionBase)]
pub fn derive_subscription_base(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, generics, ..
    } = parse_macro_input!(input);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let output = quote! {
        impl #impl_generics SESubscriptionBase for #ident #ty_generics #where_clause {}
    };
    output.into()
}

#[proc_macro_derive(SESubscribableResource)]
pub fn derive_subscribable_resource(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SESubscribableResource for #ident {}
    };
    output.into()
}

#[proc_macro_derive(SERespondableIdentifiedObject)]
pub fn derive_respondable_identified_object(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SERespondableIdentifiedObject for #ident {}
    };
    output.into()
}

#[proc_macro_derive(SERespondableSubscribableIdentifiedObject)]
pub fn derive_respondable_subscribable_identified_object(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SERespondableSubscribableIdentifiedObject for #ident {}
    };
    output.into()
}

#[proc_macro_derive(SESubscribableIdentifiedObject)]
pub fn derive_subscribable_identified_object(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SESubscribableIdentifiedObject for #ident {
            fn mrid(&self) -> &Mridtype {
                &self.mrid
            }
            fn description(&self) -> Option<&String32> {
                self.description.as_ref()
            }
            fn version(&self) -> Option<VersionType> {
                self.version
            }
            fn subscribable(&self) -> Option<SubscribableType> {
                self.subscribable
            }
        }
    };
    output.into()
}

#[proc_macro_derive(SEEvent)]
pub fn derive_event(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SEEvent for #ident {
            fn creation_time(&self) -> TimeType {
                self.creation_time
            }
            fn event_status(&self) -> &EventStatus {
                &self.event_status
            }
            fn interval(&self) -> &DateTimeInterval {
                &self.interval
            }
        }
    };
    output.into()
}

#[proc_macro_derive(SERandomizableEvent)]
pub fn derive_randomizable_event(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SERandomizableEvent for #ident {}
    };
    output.into()
}

#[proc_macro_derive(SEListLink)]
pub fn derive_list_link(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SEListLink for #ident {}
    };
    output.into()
}

#[proc_macro_derive(SEList)]
pub fn derive_list(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, generics, ..
    } = parse_macro_input!(input);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let output = quote! {
        impl #impl_generics SEList for #ident #ty_generics #where_clause {}
    };
    output.into()
}

#[proc_macro_derive(SESubscribableList)]
pub fn derive_subscribable_list(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SESubscribableList for #ident {}
    };
    output.into()
}

#[proc_macro_derive(SEFunctionSetAssignmentsBase)]
pub fn derive_fsa_base(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SEFunctionSetAssignmentsBase for #ident {}
    };
    output.into()
}

#[proc_macro_derive(SEAbstractDevice)]
pub fn derive_abstract_device(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SEAbstractDevice for #ident {}
    };
    output.into()
}

#[proc_macro_derive(SEMeterReadingBase)]
pub fn derive_meter_reading_base(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SEMeterReadingBase for #ident {}
    };
    output.into()
}

#[proc_macro_derive(SEReadingBase)]
pub fn derive_reading_base(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SEReadingBase for #ident {}
    };
    output.into()
}

#[proc_macro_derive(SEReadingSetBase)]
pub fn derive_reading_set_base(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SEReadingSetBase for #ident {}
    };
    output.into()
}

#[proc_macro_derive(SEUsagePointBase)]
pub fn derive_usage_point_base(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SEUsagePointBase for #ident {}
    };
    output.into()
}

#[proc_macro_derive(SEBillingMeterReadingBase)]
pub fn derive_billing_meter_reading_base(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SEBillingMeterReadingBase for #ident {}
    };
    output.into()
}
