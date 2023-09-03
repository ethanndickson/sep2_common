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
        impl #impl_generics SEResource for #ident #ty_generics #where_clause {
            fn href(&self) -> Option<&str> {
                self.href.as_ref().map(|s| s.as_str())
            }
        }
    };
    output.into()
}

#[proc_macro_derive(SELink)]
pub fn derive_link(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SELink for #ident {
            fn href(&self) -> &str {
                self.href.as_str()
            }
        }
    };
    output.into()
}

#[proc_macro_derive(SEResponse)]
pub fn derive_response(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
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
            fn subject(&self) -> &Mridtype {
                &self.subject
            }
        }
    };
    output.into()
}

#[proc_macro_derive(SEIdentifiedObject)]
pub fn derive_identified_object(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SEIdentifiedObject for #ident {
            fn mrid(&self) -> &Mridtype {
                &self.mrid
            }
            fn description(&self) -> Option<&String32> {
                self.description.as_ref()
            }
            fn version(&self) -> Option<VersionType> {
                self.version
            }
        }
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

            fn response_required(&self) -> Option<ResponseRequired> {
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
        impl #impl_generics SESubscriptionBase for #ident #ty_generics #where_clause {
            fn subscribed_resource(&self) -> &str {
                self.subscribed_resource.as_ref()
            }
        }
    };
    output.into()
}

#[proc_macro_derive(SESubscribableResource)]
pub fn derive_subscribable_resource(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SESubscribableResource for #ident {
            fn subscribable(&self) -> Option<SubscribableType> {
                self.subscribable
            }
        }
    };
    output.into()
}

#[proc_macro_derive(SERespondableIdentifiedObject)]
pub fn derive_respondable_identified_object(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SERespondableIdentifiedObject for #ident {
            fn mrid(&self) -> &Mridtype {
                &self.mrid
            }
            fn description(&self) -> Option<&String32> {
                self.description.as_ref()
            }
            fn version(&self) -> Option<VersionType> {
                self.version
            }
        }
    };
    output.into()
}

#[proc_macro_derive(SERespondableSubscribableIdentifiedObject)]
pub fn derive_respondable_subscribable_identified_object(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SERespondableSubscribableIdentifiedObject for #ident {
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
        impl SERandomizableEvent for #ident {
            fn randomize_duration(&self) -> Option<OneHourRangeType> {
                self.randomize_duration
            }
            fn randomize_start(&self) -> Option<OneHourRangeType> {
                self.randomize_start
            }
        }
    };
    output.into()
}

#[proc_macro_derive(SEListLink)]
pub fn derive_list_link(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SEListLink for #ident {
            fn all(&self) -> Option<Uint32> {
                self.all
            }
        }
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
        impl #impl_generics SEList for #ident #ty_generics #where_clause {
            fn all(&self) -> Uint32 {
                self.all
            }
            fn results(&self) -> Uint32 {
                self.results
            }
        }
    };
    output.into()
}

#[proc_macro_derive(SESubscribableList)]
pub fn derive_subscribable_list(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SESubscribableList for #ident {
            fn all(&self) -> Uint32 {
                self.all
            }
            fn results(&self) -> Uint32 {
                self.results
            }
        }
    };
    output.into()
}

#[proc_macro_derive(SEFunctionSetAssignmentsBase)]
pub fn derive_fsa_base(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SEFunctionSetAssignmentsBase for #ident {
            fn customer_account_list_link(&self) -> Option<&CustomerAccountListLink> {
                self.customer_account_list_link.as_ref()
            }
            fn demand_response_program_list_link(&self) -> Option<&DemandResponseProgramListLink> {
                self.demand_response_program_list_link.as_ref()
            }
            fn der_program_list_link(&self) -> Option<&DerprogramListLink> {
                self.der_program_list_link.as_ref()
            }
            fn file_list_link(&self) -> Option<&FileListLink> {
                self.file_list_link.as_ref()
            }
            fn messaging_program_list_link(&self) -> Option<&MessagingProgramListLink> {
                self.messaging_program_list_link.as_ref()
            }
            fn prepayment_list_link(&self) -> Option<&PrepaymentListLink> {
                self.prepayment_list_link.as_ref()
            }
            fn response_set_list_link(&self) -> Option<&ResponseSetListLink> {
                self.response_set_list_link.as_ref()
            }
            fn tariff_profile_list_link(&self) -> Option<&TariffProfileListLink> {
                self.tariff_profile_list_link.as_ref()
            }
            fn time_link(&self) -> Option<&TimeLink> {
                self.time_link.as_ref()
            }
            fn usage_point_list_link(&self) -> Option<&UsagePointListLink> {
                self.usage_point_list_link.as_ref()
            }
        }
    };
    output.into()
}

#[proc_macro_derive(SEAbstractDevice)]
pub fn derive_abstract_device(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SEAbstractDevice for #ident {
            fn configuration_link(&self) -> Option<&ConfigurationLink> {
                self.configuration_link.as_ref()
            }
            fn der_list_link(&self) -> Option<&DerlistLink> {
                self.der_list_link.as_ref()
            }
            fn device_category(&self) -> Option<DeviceCategoryType> {
                self.device_category
            }
            fn device_information_link(&self) -> Option<&DeviceInformationLink> {
                self.device_information_link.as_ref()
            }
            fn device_status_link(&self) -> Option<&DeviceStatusLink> {
                self.device_status_link.as_ref()
            }
            fn file_status_link(&self) -> Option<&FileStatusLink> {
                self.file_status_link.as_ref()
            }
            fn ip_interface_list_link(&self) -> Option<&IpinterfaceListLink> {
                self.ip_interface_list_link.as_ref()
            }
            fn lfdi(&self) -> Option<&HexBinary160> {
                self.lfdi.as_ref()
            }
            fn load_shed_availability_list_link(&self) -> Option<&LoadShedAvailabilityListLink> {
                self.load_shed_availability_list_link.as_ref()
            }
            fn log_event_list_link(&self) -> Option<&LogEventListLink> {
                self.log_event_list_link.as_ref()
            }
            fn power_status_link(&self) -> Option<&PowerStatusLink> {
                self.power_status_link.as_ref()
            }
            fn sfdi(&self) -> SFDIType {
                self.sfdi
            }
            fn subscribable(&self) -> Option<&SubscribableType> {
                self.subscribable.as_ref()
            }
        }
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
    };
    output.into()
}

#[proc_macro_derive(SEReadingSetBase)]
pub fn derive_reading_set_base(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SEReadingSetBase for #ident {
            fn time_period(&self) -> &DateTimeInterval {
                &self.time_period
            }
        }
    };
    output.into()
}

#[proc_macro_derive(SEUsagePointBase)]
pub fn derive_usage_point_base(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SEUsagePointBase for #ident {
            fn role_flags(&self) -> RoleFlagsType {
                self.role_flags
            }
            fn service_category_kind(&self) -> ServiceKind {
                self.service_category_kind
            }
        }
    };
    output.into()
}

#[proc_macro_derive(SEBillingMeterReadingBase)]
pub fn derive_billing_meter_reading_base(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SEBillingMeterReadingBase for #ident {
            fn billing_reading_set_list_link(&self) -> Option<&BillingReadingSetListLink> {
                self.billing_reading_set_list_link.as_ref()
            }
            fn reading_type_link(&self) -> Option<&ReadingTypeLink> {
                self.reading_type_link.as_ref()
            }
        }
    };
    output.into()
}
