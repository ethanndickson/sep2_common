use list::extract_list;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod list;

#[proc_macro_derive(SEResource)]
pub fn derive_resource(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, generics, ..
    } = parse_macro_input!(input);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    quote! {
        impl #impl_generics SEResource for #ident #ty_generics #where_clause {
            fn href(&self) -> Option<&str> {
                self.href.as_ref().map(|s| s.as_str())
            }
        }
    }
    .into()
}

#[proc_macro_derive(SEResponse)]
pub fn derive_response(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    quote! {
        impl SEResponse for #ident {
            fn created_date_time(&self) -> Option<TimeType> {
                self.created_date_time
            }
            fn end_device_lfdi(&self) -> &HexBinary160 {
                &self.end_device_lfdi
            }
            fn status(&self) -> Option<ResponseStatus> {
                self.status
            }
            fn subject(&self) -> &MRIDType {
                &self.subject
            }
        }
    }
    .into()
}

#[proc_macro_derive(SEIdentifiedObject)]
pub fn derive_identified_object(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    quote! {
        impl SEIdentifiedObject for #ident {
            fn mrid(&self) -> &MRIDType {
                &self.mrid
            }
            fn description(&self) -> Option<&String32> {
                self.description.as_ref()
            }
            fn version(&self) -> Option<VersionType> {
                self.version
            }
        }
    }
    .into()
}

#[proc_macro_derive(SERespondableResource)]
pub fn derive_respondable_resource(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    quote! {
        impl SERespondableResource for #ident {
            fn reply_to(&self) -> Option<&str> {
                self.reply_to.as_ref().map(|s| s.as_str())
            }

            fn response_required(&self) -> Option<ResponseRequired> {
                self.response_required
            }
        }
    }
    .into()
}

#[proc_macro_derive(SESubscriptionBase)]
pub fn derive_subscription_base(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, generics, ..
    } = parse_macro_input!(input);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    quote! {
        impl #impl_generics SESubscriptionBase for #ident #ty_generics #where_clause {
            fn subscribed_resource(&self) -> &str {
                self.subscribed_resource.as_ref()
            }
        }
    }
    .into()
}

#[proc_macro_derive(SESubscribableResource)]
pub fn derive_subscribable_resource(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    quote! {
        impl SESubscribableResource for #ident {
            fn subscribable(&self) -> Option<SubscribableType> {
                self.subscribable
            }
        }
    }
    .into()
}

#[proc_macro_derive(SERespondableIdentifiedObject)]
pub fn derive_respondable_identified_object(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    quote! {
        impl SERespondableIdentifiedObject for #ident {}
    }
    .into()
}

#[proc_macro_derive(SERespondableSubscribableIdentifiedObject)]
pub fn derive_respondable_subscribable_identified_object(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    quote! {
        impl SERespondableSubscribableIdentifiedObject for #ident {}
    }
    .into()
}

#[proc_macro_derive(SESubscribableIdentifiedObject)]
pub fn derive_subscribable_identified_object(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    quote! {
        impl SESubscribableIdentifiedObject for #ident {}
    }
    .into()
}

#[proc_macro_derive(SEEvent)]
pub fn derive_event(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    quote! {
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
    }
    .into()
}

#[proc_macro_derive(SERandomizableEvent)]
pub fn derive_randomizable_event(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    quote! {
        impl SERandomizableEvent for #ident {
            fn randomize_duration(&self) -> Option<OneHourRangeType> {
                self.randomize_duration
            }
            fn randomize_start(&self) -> Option<OneHourRangeType> {
                self.randomize_start
            }
        }
    }
    .into()
}

#[proc_macro_derive(SEList)]
pub fn derive_list(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = parse_macro_input!(input);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let inner = extract_list(&data);
    if let Some((field, inner)) = inner {
        quote! {
            impl #impl_generics SEList for #ident #ty_generics #where_clause {
                type Inner = #inner;

                fn all(&self) -> Uint32 {
                    self.all
                }

                fn all_mut(&mut self) -> &mut Uint32 {
                    &mut self.all
                }

                fn results(&self) -> Uint32 {
                    self.results
                }

                fn results_mut(&mut self) -> &mut Uint32 {
                    &mut self.results
                }

                fn list_as_slice(&self) -> &[Self::Inner] {
                    self.#field.as_slice()
                }

                fn list_mut(&mut self) -> &mut Vec<Self::Inner> {
                    &mut self.#field
                }

            }
        }
        .into()
    } else {
        quote! { compile_error!("List resources require their first field to be a Vec" )}.into()
    }
}

#[proc_macro_derive(SESubscribableList)]
pub fn derive_subscribable_list(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    quote! {
        impl SESubscribableList for #ident {}
    }
    .into()
}

#[proc_macro_derive(SEFunctionSetAssignmentsBase)]
pub fn derive_fsa_base(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    quote! {
        impl SEFunctionSetAssignmentsBase for #ident {
            fn customer_account_list_link(&self) -> Option<&ListLink> {
                self.customer_account_list_link.as_ref()
            }
            fn demand_response_program_list_link(&self) -> Option<&ListLink> {
                self.demand_response_program_list_link.as_ref()
            }
            fn der_program_list_link(&self) -> Option<&ListLink> {
                self.der_program_list_link.as_ref()
            }
            fn file_list_link(&self) -> Option<&ListLink> {
                self.file_list_link.as_ref()
            }
            fn messaging_program_list_link(&self) -> Option<&ListLink> {
                self.messaging_program_list_link.as_ref()
            }
            fn prepayment_list_link(&self) -> Option<&ListLink> {
                self.prepayment_list_link.as_ref()
            }
            fn response_set_list_link(&self) -> Option<&ListLink> {
                self.response_set_list_link.as_ref()
            }
            fn tariff_profile_list_link(&self) -> Option<&ListLink> {
                self.tariff_profile_list_link.as_ref()
            }
            fn time_link(&self) -> Option<&Link> {
                self.time_link.as_ref()
            }
            fn usage_point_list_link(&self) -> Option<&ListLink> {
                self.usage_point_list_link.as_ref()
            }
        }
    }
    .into()
}

#[proc_macro_derive(SEAbstractDevice)]
pub fn derive_abstract_device(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    quote! {
        impl SEAbstractDevice for #ident {
            fn configuration_link(&self) -> Option<&Link> {
                self.configuration_link.as_ref()
            }
            fn der_list_link(&self) -> Option<&ListLink> {
                self.der_list_link.as_ref()
            }
            fn device_category(&self) -> Option<DeviceCategoryType> {
                self.device_category
            }
            fn device_information_link(&self) -> Option<&Link> {
                self.device_information_link.as_ref()
            }
            fn device_status_link(&self) -> Option<&Link> {
                self.device_status_link.as_ref()
            }
            fn file_status_link(&self) -> Option<&Link> {
                self.file_status_link.as_ref()
            }
            fn ip_interface_list_link(&self) -> Option<&ListLink> {
                self.ip_interface_list_link.as_ref()
            }
            fn lfdi(&self) -> Option<&HexBinary160> {
                self.lfdi.as_ref()
            }
            fn load_shed_availability_list_link(&self) -> Option<&ListLink> {
                self.load_shed_availability_list_link.as_ref()
            }
            fn log_event_list_link(&self) -> Option<&ListLink> {
                self.log_event_list_link.as_ref()
            }
            fn power_status_link(&self) -> Option<&Link> {
                self.power_status_link.as_ref()
            }
            fn sfdi(&self) -> SFDIType {
                self.sfdi
            }
        }
    }
    .into()
}

#[proc_macro_derive(SEMeterReadingBase)]
pub fn derive_meter_reading_base(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    quote! {
        impl SEMeterReadingBase for #ident {}
    }
    .into()
}

#[proc_macro_derive(SEReadingBase)]
pub fn derive_reading_base(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    quote! {
        impl SEReadingBase for #ident {
            fn consumption_block(&self) -> Option<ConsumptionBlockType> {
                self.consumption_block
            }
            fn quality_flags(&self) -> Option<HexBinary16> {
                self.quality_flags
            }
            fn time_period(&self) -> Option<&DateTimeInterval> {
                self.time_period.as_ref()
            }
            fn tou_tier(&self) -> Option<Toutype> {
                self.tou_tier
            }
            fn value(&self) -> Option<Int48> {
                self.value
            }
        }
    }
    .into()
}

#[proc_macro_derive(SEReadingSetBase)]
pub fn derive_reading_set_base(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    quote! {
        impl SEReadingSetBase for #ident {
            fn time_period(&self) -> &DateTimeInterval {
                &self.time_period
            }
        }
    }
    .into()
}

#[proc_macro_derive(SEUsagePointBase)]
pub fn derive_usage_point_base(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    quote! {
        impl SEUsagePointBase for #ident {
            fn role_flags(&self) -> RoleFlagsType {
                self.role_flags
            }
            fn service_category_kind(&self) -> ServiceKind {
                self.service_category_kind
            }
        }
    }
    .into()
}

#[proc_macro_derive(SEBillingMeterReadingBase)]
pub fn derive_billing_meter_reading_base(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    quote! {
        impl SEBillingMeterReadingBase for #ident {
            fn billing_reading_set_list_link(&self) -> Option<&ListLink> {
                self.billing_reading_set_list_link.as_ref()
            }
            fn reading_type_link(&self) -> Option<&Link> {
                self.reading_type_link.as_ref()
            }
        }
    }
    .into()
}
