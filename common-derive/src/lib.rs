use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, GenericArgument, Ident, PathArguments, Type,
};

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
            #[inline(always)]
            fn created_date_time(&self) -> Option<TimeType> {
                self.created_date_time
            }

            #[inline(always)]
            fn end_device_lfdi(&self) -> &HexBinary160 {
                &self.end_device_lfdi
            }

            #[inline(always)]
            fn status(&self) -> Option<ResponseStatus> {
                self.status
            }

            #[inline(always)]
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

            #[inline(always)]
            fn mrid(&self) -> &MRIDType {
                &self.mrid
            }

            #[inline(always)]
            fn description(&self) -> Option<&String32> {
                self.description.as_ref()
            }

            #[inline(always)]
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
            #[inline(always)]
            fn reply_to(&self) -> Option<&str> {
                self.reply_to.as_ref().map(|s| s.as_str())
            }

            #[inline(always)]
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

            #[inline(always)]
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

            #[inline(always)]
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

            #[inline(always)]
            fn creation_time(&self) -> TimeType {
                self.creation_time
            }

            #[inline(always)]
            fn event_status(&self) -> &EventStatus {
                &self.event_status
            }


            #[inline(always)]
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

            #[inline(always)]
            fn randomize_duration(&self) -> Option<OneHourRangeType> {
                self.randomize_duration
            }

            #[inline(always)]
            fn randomize_start(&self) -> Option<OneHourRangeType> {
                self.randomize_start
            }
        }
    }
    .into()
}

fn extract_list(data: &Data) -> Option<(Ident, Type)> {
    if let Data::Struct(data) = data {
        if let Fields::Named(fields) = &data.fields {
            if let Some(field) = &fields.named.first() {
                let ident = field.ident.clone()?;
                if let Type::Path(vec_typepath) = &field.ty {
                    if let Some(segment) = vec_typepath.path.segments.first() {
                        if let PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(GenericArgument::Type(out_type)) = args.args.first() {
                                return Some((ident, out_type.clone()));
                            }
                        }
                    }
                }
            }
        }
    }
    None
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

                #[inline(always)]
                fn all(&self) -> Uint32 {
                    self.all
                }

                #[inline(always)]
                fn all_mut(&mut self) -> &mut Uint32 {
                    &mut self.all
                }

                #[inline(always)]
                fn results(&self) -> Uint32 {
                    self.results
                }

                #[inline(always)]
                fn results_mut(&mut self) -> &mut Uint32 {
                    &mut self.results
                }

                #[inline(always)]
                fn list_as_slice(&self) -> &[Self::Inner] {
                    self.#field.as_slice()
                }

                #[inline(always)]
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

            #[inline(always)]
            fn customer_account_list_link(&self) -> Option<&CustomerAccountListLink> {
                self.customer_account_list_link.as_ref()
            }

            #[inline(always)]
            fn demand_response_program_list_link(&self) -> Option<&DemandResponseProgramListLink> {
                self.demand_response_program_list_link.as_ref()
            }

            #[inline(always)]
            fn der_program_list_link(&self) -> Option<&DERProgramListLink> {
                self.der_program_list_link.as_ref()
            }

            #[inline(always)]
            fn file_list_link(&self) -> Option<&FileListLink> {
                self.file_list_link.as_ref()
            }

            #[inline(always)]
            fn messaging_program_list_link(&self) -> Option<&MessagingProgramListLink> {
                self.messaging_program_list_link.as_ref()
            }

            #[inline(always)]
            fn prepayment_list_link(&self) -> Option<&PrepaymentListLink> {
                self.prepayment_list_link.as_ref()
            }

            #[inline(always)]
            fn response_set_list_link(&self) -> Option<&ResponseSetListLink> {
                self.response_set_list_link.as_ref()
            }

            #[inline(always)]
            fn tariff_profile_list_link(&self) -> Option<&TariffProfileListLink> {
                self.tariff_profile_list_link.as_ref()
            }

            #[inline(always)]
            fn time_link(&self) -> Option<&TimeLink> {
                self.time_link.as_ref()
            }

            #[inline(always)]
            fn usage_point_list_link(&self) -> Option<&UsagePointListLink> {
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

            #[inline(always)]
            fn configuration_link(&self) -> Option<&ConfigurationLink> {
                self.configuration_link.as_ref()
            }

            #[inline(always)]
            fn der_list_link(&self) -> Option<&DERListLink> {
                self.der_list_link.as_ref()
            }

            #[inline(always)]
            fn device_category(&self) -> Option<DeviceCategoryType> {
                self.device_category
            }

            #[inline(always)]
            fn device_information_link(&self) -> Option<&DeviceInformationLink> {
                self.device_information_link.as_ref()
            }

            #[inline(always)]
            fn device_status_link(&self) -> Option<&DeviceStatusLink> {
                self.device_status_link.as_ref()
            }

            #[inline(always)]
            fn file_status_link(&self) -> Option<&FileStatusLink> {
                self.file_status_link.as_ref()
            }

            #[inline(always)]
            fn ip_interface_list_link(&self) -> Option<&IPInterfaceListLink> {
                self.ip_interface_list_link.as_ref()
            }

            #[inline(always)]
            fn lfdi(&self) -> Option<&HexBinary160> {
                self.lfdi.as_ref()
            }

            #[inline(always)]
            fn load_shed_availability_list_link(&self) -> Option<&LoadShedAvailabilityListLink> {
                self.load_shed_availability_list_link.as_ref()
            }

            #[inline(always)]
            fn log_event_list_link(&self) -> Option<&LogEventListLink> {
                self.log_event_list_link.as_ref()
            }

            #[inline(always)]
            fn power_status_link(&self) -> Option<&PowerStatusLink> {
                self.power_status_link.as_ref()
            }

            #[inline(always)]
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

            #[inline(always)]
            fn consumption_block(&self) -> Option<ConsumptionBlockType> {
                self.consumption_block
            }

            #[inline(always)]
            fn quality_flags(&self) -> Option<HexBinary16> {
                self.quality_flags
            }

            #[inline(always)]
            fn time_period(&self) -> Option<&DateTimeInterval> {
                self.time_period.as_ref()
            }

            #[inline(always)]
            fn tou_tier(&self) -> Option<TOUType> {
                self.tou_tier
            }

            #[inline(always)]
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

            #[inline(always)]
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

            #[inline(always)]
            fn role_flags(&self) -> RoleFlagsType {
                self.role_flags
            }

            #[inline(always)]
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

            #[inline(always)]
            fn billing_reading_set_list_link(&self) -> Option<&BillingReadingSetListLink> {
                self.billing_reading_set_list_link.as_ref()
            }

            #[inline(always)]
            fn reading_type_link(&self) -> Option<&ReadingTypeLink> {
                self.reading_type_link.as_ref()
            }
        }
    }
    .into()
}
