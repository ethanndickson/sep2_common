#![allow(non_snake_case)]
use sep2_common::{
    deserialize,
    packages::{
        billing::*, configuration::*, dcap::*, der::*, di::*, drlc::*, edev::*,
        flow_reservation::*, fsa::*, identification::*, log_events::*, messaging::*, metering::*,
        metering_mirror::*, network_status::*, objects::*, power_status::*, prepayment::*,
        pricing::*, pubsub::*, response::*, software_download::*, time::*, types::*,
    },
    serialize,
};

#[test]
fn default_Resource() {
    let orig = Resource::default();
    let new: Resource = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Response() {
    let orig = Response::default();
    let new: Response = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_List() {
    let orig = List::default();
    let new: List = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ListLink() {
    let orig: ListLink = ListLink::default();
    let new: ListLink = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_IdentifiedObject() {
    let orig = IdentifiedObject::default();
    let new: IdentifiedObject = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RespondableResource() {
    let orig = RespondableResource::default();
    let new: RespondableResource = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RespondableIdentifiedObject() {
    let orig = RespondableIdentifiedObject::default();
    let new: RespondableIdentifiedObject = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RespondableSubscribableIdentifiedObject() {
    let orig = RespondableSubscribableIdentifiedObject::default();
    let new: RespondableSubscribableIdentifiedObject =
        deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SubscribableResource() {
    let orig = SubscribableResource::default();
    let new: SubscribableResource = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SubscribableList() {
    let orig = SubscribableList::default();
    let new: SubscribableList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SubscribableIdentifiedObject() {
    let orig = SubscribableIdentifiedObject::default();
    let new: SubscribableIdentifiedObject = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_EventStatus() {
    let orig = EventStatus::default();
    let new: EventStatus = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Event() {
    let orig = Event::default();
    let new: Event = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Error() {
    let orig = Error::default();
    let new: Error = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RandomizableEvent() {
    let orig = RandomizableEvent::default();
    let new: RandomizableEvent = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERControl() {
    let orig = DERControl::default();
    let new: DERControl = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TextMessage() {
    let orig = TextMessage::default();
    let new: TextMessage = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TimeTariffInterval() {
    let orig = TimeTariffInterval::default();
    let new: TimeTariffInterval = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_EndDeviceControl() {
    let orig = EndDeviceControl::default();
    let new: EndDeviceControl = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DemandResponseProgram() {
    let orig = DemandResponseProgram::default();
    let new: DemandResponseProgram = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TariffProfile() {
    let orig = TariffProfile::default();
    let new: TariffProfile = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_MessagingProgram() {
    let orig = MessagingProgram::default();
    let new: MessagingProgram = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PrimacyType() {
    let orig = PrimacyType::default();
    let new: PrimacyType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DeviceCapability() {
    let orig = DeviceCapability::default();
    let new: DeviceCapability = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_AbstractDevice() {
    let orig = AbstractDevice::default();
    let new: AbstractDevice = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DeviceStatus() {
    let orig = DeviceStatus::default();
    let new: DeviceStatus = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_EndDevice() {
    let orig = EndDevice::default();
    let new: EndDevice = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_EndDeviceList() {
    let orig = EndDeviceList::default();
    let new: EndDeviceList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Registration() {
    let orig = Registration::default();
    let new: Registration = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SelfDevice() {
    let orig = SelfDevice::default();
    let new: SelfDevice = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Temperature() {
    let orig = Temperature::default();
    let new: Temperature = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FunctionSetAssignmentsBase() {
    let orig = FunctionSetAssignmentsBase::default();
    let new: FunctionSetAssignmentsBase = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FunctionSetAssignments() {
    let orig = FunctionSetAssignments::default();
    let new: FunctionSetAssignments = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FunctionSetAssignmentsList() {
    let orig = FunctionSetAssignmentsList::default();
    let new: FunctionSetAssignmentsList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Condition() {
    let orig = Condition::default();
    let new: Condition = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SubscriptionBase() {
    let orig = SubscriptionBase::default();
    let new: SubscriptionBase = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Subscription() {
    let orig = Subscription::default();
    let new: Subscription = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SubscriptionList() {
    let orig = SubscriptionList::default();
    let new: SubscriptionList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Notification() {
    let orig: Notification<EndDeviceControl> = Notification::default();
    let new: Notification<EndDeviceControl> = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_NotificationList() {
    let orig: NotificationList<EndDeviceControl> = NotificationList::default();
    let new: NotificationList<EndDeviceControl> = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERControlResponse() {
    let orig = DERControlResponse::default();
    let new: DERControlResponse = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FlowReservationResponseResponse() {
    let orig = FlowReservationResponseResponse::default();
    let new: FlowReservationResponseResponse = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_AppliedTargetReduction() {
    let orig = AppliedTargetReduction::default();
    let new: AppliedTargetReduction = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DrResponse() {
    let orig = DrResponse::default();
    let new: DrResponse = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PriceResponse() {
    let orig = PriceResponse::default();
    let new: PriceResponse = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ResponseSet() {
    let orig = ResponseSet::default();
    let new: ResponseSet = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ResponseSetList() {
    let orig = ResponseSetList::default();
    let new: ResponseSetList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TextResponse() {
    let orig = TextResponse::default();
    let new: TextResponse = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Time() {
    let orig = Time::default();
    let new: Time = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DeviceInformation() {
    let orig = DeviceInformation::default();
    let new: DeviceInformation = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DRLCCapabilities() {
    let orig = DRLCCapabilities::default();
    let new: DRLCCapabilities = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SupportedLocale() {
    let orig = SupportedLocale::default();
    let new: SupportedLocale = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SupportedLocaleList() {
    let orig = SupportedLocaleList::default();
    let new: SupportedLocaleList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PowerStatus() {
    let orig = PowerStatus::default();
    let new: PowerStatus = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PowerSourceType() {
    let orig = PowerSourceType::default();
    let new: PowerSourceType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PEVInfo() {
    let orig = Pevinfo::default();
    let new: Pevinfo = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_IPAddr() {
    let orig = IPAddr::default();
    let new: IPAddr = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_IPAddrList() {
    let orig = IPAddrList::default();
    let new: IPAddrList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_IPInterface() {
    let orig = IPInterface::default();
    let new: IPInterface = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_IPInterfaceList() {
    let orig = IPInterfaceList::default();
    let new: IPInterfaceList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_LLInterface() {
    let orig = LLInterface::default();
    let new: LLInterface = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_LLInterfaceList() {
    let orig = LlinterfaceList::default();
    let new: LlinterfaceList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_loWPAN() {
    let orig = LoWPAN::default();
    let new: LoWPAN = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Neighbor() {
    let orig = Neighbor::default();
    let new: Neighbor = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_NeighborList() {
    let orig = NeighborList::default();
    let new: NeighborList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RPLInstance() {
    let orig = RPLInstance::default();
    let new: RPLInstance = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RPLInstanceList() {
    let orig = RPLInstanceList::default();
    let new: RPLInstanceList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RPLSourceRoutes() {
    let orig = RPLSourceRoutes::default();
    let new: RPLSourceRoutes = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RPLSourceRoutesList() {
    let orig = RPLSourceRoutesList::default();
    let new: RPLSourceRoutesList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_LogEvent() {
    let orig = LogEvent::default();
    let new: LogEvent = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_LogEventList() {
    let orig = LogEventList::default();
    let new: LogEventList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Configuration() {
    let orig = Configuration::default();
    let new: Configuration = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PowerConfiguration() {
    let orig = PowerConfiguration::default();
    let new: PowerConfiguration = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PriceResponseCfg() {
    let orig = PriceResponseCfg::default();
    let new: PriceResponseCfg = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PriceResponseCfgList() {
    let orig = PriceResponseCfgList::default();
    let new: PriceResponseCfgList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TimeConfiguration() {
    let orig = TimeConfiguration::default();
    let new: TimeConfiguration = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_File() {
    let orig = File::default();
    let new: File = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FileList() {
    let orig = FileList::default();
    let new: FileList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FileStatus() {
    let orig = FileStatus::default();
    let new: FileStatus = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_LoadShedAvailabilityList() {
    let orig = LoadShedAvailabilityList::default();
    let new: LoadShedAvailabilityList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ApplianceLoadReduction() {
    let orig = ApplianceLoadReduction::default();
    let new: ApplianceLoadReduction = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DemandResponseProgramList() {
    let orig = DemandResponseProgramList::default();
    let new: DemandResponseProgramList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DutyCycle() {
    let orig = DutyCycle::default();
    let new: DutyCycle = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_EndDeviceControlList() {
    let orig = EndDeviceControlList::default();
    let new: EndDeviceControlList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_LoadShedAvailability() {
    let orig = LoadShedAvailability::default();
    let new: LoadShedAvailability = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Offset() {
    let orig = Offset::default();
    let new: Offset = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SetPoint() {
    let orig = SetPoint::default();
    let new: SetPoint = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TargetReduction() {
    let orig = TargetReduction::default();
    let new: TargetReduction = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_MeterReading() {
    let orig = MeterReading::default();
    let new: MeterReading = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_MeterReadingList() {
    let orig = MeterReadingList::default();
    let new: MeterReadingList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Reading() {
    let orig = Reading::default();
    let new: Reading = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ReadingList() {
    let orig = ReadingList::default();
    let new: ReadingList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ReadingSet() {
    let orig = ReadingSet::default();
    let new: ReadingSet = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ReadingSetList() {
    let orig = ReadingSetList::default();
    let new: ReadingSetList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ReadingType() {
    let orig = ReadingType::default();
    let new: ReadingType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_UsagePoint() {
    let orig = UsagePoint::default();
    let new: UsagePoint = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_UsagePointList() {
    let orig = UsagePointList::default();
    let new: UsagePointList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ConsumptionTariffInterval() {
    let orig = ConsumptionTariffInterval::default();
    let new: ConsumptionTariffInterval = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ConsumptionTariffIntervalList() {
    let orig = ConsumptionTariffIntervalList::default();
    let new: ConsumptionTariffIntervalList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CostKindType() {
    let orig = CostKindType::default();
    let new: CostKindType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_EnvironmentalCost() {
    let orig = EnvironmentalCost::default();
    let new: EnvironmentalCost = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RateComponent() {
    let orig = RateComponent::default();
    let new: RateComponent = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RateComponentList() {
    let orig = RateComponentList::default();
    let new: RateComponentList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TariffProfileList() {
    let orig = TariffProfileList::default();
    let new: TariffProfileList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TimeTariffIntervalList() {
    let orig = TimeTariffIntervalList::default();
    let new: TimeTariffIntervalList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_MessagingProgramList() {
    let orig = MessagingProgramList::default();
    let new: MessagingProgramList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PriorityType() {
    let orig = PriorityType::default();
    let new: PriorityType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TextMessageList() {
    let orig = TextMessageList::default();
    let new: TextMessageList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_BillingPeriod() {
    let orig = BillingPeriod::default();
    let new: BillingPeriod = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_BillingPeriodList() {
    let orig = BillingPeriodList::default();
    let new: BillingPeriodList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_BillingMeterReadingBase() {
    let orig = BillingMeterReadingBase::default();
    let new: BillingMeterReadingBase = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_BillingReading() {
    let orig = BillingReading::default();
    let new: BillingReading = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_BillingReadingList() {
    let orig = BillingReadingList::default();
    let new: BillingReadingList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_BillingReadingSet() {
    let orig = BillingReadingSet::default();
    let new: BillingReadingSet = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_BillingReadingSetList() {
    let orig = BillingReadingSetList::default();
    let new: BillingReadingSetList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Charge() {
    let orig = Charge::default();
    let new: Charge = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ChargeKind() {
    let orig = ChargeKind::default();
    let new: ChargeKind = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CustomerAccount() {
    let orig = CustomerAccount::default();
    let new: CustomerAccount = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CustomerAccountList() {
    let orig = CustomerAccountList::default();
    let new: CustomerAccountList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CustomerAgreement() {
    let orig = CustomerAgreement::default();
    let new: CustomerAgreement = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CustomerAgreementList() {
    let orig = CustomerAgreementList::default();
    let new: CustomerAgreementList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_HistoricalReading() {
    let orig = HistoricalReading::default();
    let new: HistoricalReading = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_HistoricalReadingList() {
    let orig = HistoricalReadingList::default();
    let new: HistoricalReadingList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ProjectionReading() {
    let orig = ProjectionReading::default();
    let new: ProjectionReading = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ProjectionReadingList() {
    let orig = ProjectionReadingList::default();
    let new: ProjectionReadingList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TargetReading() {
    let orig = TargetReading::default();
    let new: TargetReading = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TargetReadingList() {
    let orig = TargetReadingList::default();
    let new: TargetReadingList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ServiceSupplier() {
    let orig = ServiceSupplier::default();
    let new: ServiceSupplier = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ServiceSupplierList() {
    let orig = ServiceSupplierList::default();
    let new: ServiceSupplierList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_AccountBalance() {
    let orig = AccountBalance::default();
    let new: AccountBalance = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_AccountingUnit() {
    let orig = AccountingUnit::default();
    let new: AccountingUnit = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CreditRegister() {
    let orig = CreditRegister::default();
    let new: CreditRegister = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CreditRegisterList() {
    let orig = CreditRegisterList::default();
    let new: CreditRegisterList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Prepayment() {
    let orig = Prepayment::default();
    let new: Prepayment = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PrepaymentList() {
    let orig = PrepaymentList::default();
    let new: PrepaymentList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PrepayModeType() {
    let orig = PrepayModeType::default();
    let new: PrepayModeType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PrepayOperationStatus() {
    let orig = PrepayOperationStatus::default();
    let new: PrepayOperationStatus = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ServiceChange() {
    let orig = ServiceChange::default();
    let new: ServiceChange = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SupplyInterruptionOverride() {
    let orig = SupplyInterruptionOverride::default();
    let new: SupplyInterruptionOverride = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SupplyInterruptionOverrideList() {
    let orig = SupplyInterruptionOverrideList::default();
    let new: SupplyInterruptionOverrideList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CreditStatusType() {
    let orig = CreditStatusType::default();
    let new: CreditStatusType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CreditTypeType() {
    let orig = CreditTypeType::default();
    let new: CreditTypeType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CreditTypeChange() {
    let orig = CreditTypeChange::default();
    let new: CreditTypeChange = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ServiceStatusType() {
    let orig = ServiceStatusType::default();
    let new: ServiceStatusType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RequestStatus() {
    let orig = RequestStatus::default();
    let new: RequestStatus = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FlowReservationRequest() {
    let orig = FlowReservationRequest::default();
    let new: FlowReservationRequest = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FlowReservationRequestList() {
    let orig = FlowReservationRequestList::default();
    let new: FlowReservationRequestList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FlowReservationResponse() {
    let orig = FlowReservationResponse::default();
    let new: FlowReservationResponse = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FlowReservationResponseList() {
    let orig = FlowReservationResponseList::default();
    let new: FlowReservationResponseList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DefaultDERControl() {
    let orig = DefaultDERControl::default();
    let new: DefaultDERControl = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FreqDroopType() {
    let orig = FreqDroopType::default();
    let new: FreqDroopType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DER() {
    let orig = DER::default();
    let new: DER = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERList() {
    let orig = DERList::default();
    let new: DERList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERSettings() {
    let orig = DERSettings::default();
    let new: DERSettings = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERType() {
    let orig = DERType::default();
    let new: DERType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERAvailability() {
    let orig = DERAvailability::default();
    let new: DERAvailability = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERCapability() {
    let orig = DERCapability::default();
    let new: DERCapability = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERControlBase() {
    let orig = DERControlBase::default();
    let new: DERControlBase = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERControlList() {
    let orig = DERControlList::default();
    let new: DERControlList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERControlType() {
    let orig = DERControlType::default();
    let new: DERControlType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERCurve() {
    let orig = DERCurve::default();
    let new: DERCurve = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERCurveList() {
    let orig = DERCurveList::default();
    let new: DERCurveList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CurveData() {
    let orig = CurveData::default();
    let new: CurveData = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERCurveType() {
    let orig = DERCurveType::default();
    let new: DERCurveType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERProgram() {
    let orig = DERProgram::default();
    let new: DERProgram = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERProgramList() {
    let orig = DERProgramList::default();
    let new: DERProgramList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERStatus() {
    let orig = DERStatus::default();
    let new: DERStatus = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERUnitRefType() {
    let orig = DERUnitRefType::default();
    let new: DERUnitRefType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CurrentRMS() {
    let orig = CurrentRMS::default();
    let new: CurrentRMS = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FixedPointType() {
    let orig = FixedPointType::default();
    let new: FixedPointType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_UnsignedFixedPointType() {
    let orig = UnsignedFixedPointType::default();
    let new: UnsignedFixedPointType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ActivePower() {
    let orig = ActivePower::default();
    let new: ActivePower = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_AmpereHour() {
    let orig = AmpereHour::default();
    let new: AmpereHour = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ApparentPower() {
    let orig = ApparentPower::default();
    let new: ApparentPower = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ReactivePower() {
    let orig = ReactivePower::default();
    let new: ReactivePower = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ReactiveSusceptance() {
    let orig = ReactiveSusceptance::default();
    let new: ReactiveSusceptance = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PowerFactor() {
    let orig = PowerFactor::default();
    let new: PowerFactor = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PowerFactorWithExcitation() {
    let orig = PowerFactorWithExcitation::default();
    let new: PowerFactorWithExcitation = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FixedVar() {
    let orig = FixedVar::default();
    let new: FixedVar = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_WattHour() {
    let orig = WattHour::default();
    let new: WattHour = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_VoltageRMS() {
    let orig = VoltageRMS::default();
    let new: VoltageRMS = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ConnectStatusType() {
    let orig = ConnectStatusType::default();
    let new: ConnectStatusType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_InverterStatusType() {
    let orig = InverterStatusType::default();
    let new: InverterStatusType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_LocalControlModeStatusType() {
    let orig = LocalControlModeStatusType::default();
    let new: LocalControlModeStatusType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ManufacturerStatusType() {
    let orig = ManufacturerStatusType::default();
    let new: ManufacturerStatusType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_OperationalModeStatusType() {
    let orig = OperationalModeStatusType::default();
    let new: OperationalModeStatusType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_StateOfChargeStatusType() {
    let orig = StateOfChargeStatusType::default();
    let new: StateOfChargeStatusType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_StorageModeStatusType() {
    let orig = StorageModeStatusType::default();
    let new: StorageModeStatusType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Link() {
    let orig: Link = Link::default();
    let new: Link = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_AccumulationBehaviourType() {
    let orig = AccumulationBehaviourType::default();
    let new: AccumulationBehaviourType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ApplianceLoadReductionType() {
    let orig = ApplianceLoadReductionType::default();
    let new: ApplianceLoadReductionType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CommodityType() {
    let orig = CommodityType::default();
    let new: CommodityType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ConsumptionBlockType() {
    let orig = ConsumptionBlockType::default();
    let new: ConsumptionBlockType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CurrencyCode() {
    let orig = CurrencyCode::default();
    let new: CurrencyCode = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DataQualifierType() {
    let orig = DataQualifierType::default();
    let new: DataQualifierType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DateTimeInterval() {
    let orig = DateTimeInterval::default();
    let new: DateTimeInterval = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DeviceCategoryType() {
    let orig = DeviceCategoryType::default();
    let new: DeviceCategoryType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DstRuleType() {
    let orig = DstRuleType::default();
    let new: DstRuleType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FlowDirectionType() {
    let orig = FlowDirectionType::default();
    let new: FlowDirectionType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_GPSLocationType() {
    let orig = GPSLocationType::default();
    let new: GPSLocationType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_KindType() {
    let orig = KindType::default();
    let new: KindType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_LocaleType() {
    let orig = LocaleType::default();
    let new: LocaleType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_mRIDType() {
    let orig = MRIDType::default();
    let new: MRIDType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_OneHourRangeType() {
    let orig = OneHourRangeType::default();
    let new: OneHourRangeType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PENType() {
    let orig = PENType::default();
    let new: PENType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Percent() {
    let orig = Percent::default();
    let new: Percent = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PhaseCode() {
    let orig = PhaseCode::default();
    let new: PhaseCode = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PINType() {
    let orig = PINType::default();
    let new: PINType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PowerOfTenMultiplierType() {
    let orig = PowerOfTenMultiplierType::default();
    let new: PowerOfTenMultiplierType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RealEnergy() {
    let orig = RealEnergy::default();
    let new: RealEnergy = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RoleFlagsType() {
    let orig = RoleFlagsType::default();
    let new: RoleFlagsType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ServiceKind() {
    let orig = ServiceKind::default();
    let new: ServiceKind = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SFDIType() {
    let orig = SFDIType::default();
    let new: SFDIType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SignedPercent() {
    let orig = SignedPercent::default();
    let new: SignedPercent = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SignedRealEnergy() {
    let orig = SignedRealEnergy::default();
    let new: SignedRealEnergy = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TimeOffsetType() {
    let orig = TimeOffsetType::default();
    let new: TimeOffsetType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TimeType() {
    let orig = TimeType::default();
    let new: TimeType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TOUType() {
    let orig = TOUType::default();
    let new: TOUType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_UnitType() {
    let orig = UnitType::default();
    let new: UnitType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_UnitValueType() {
    let orig = UnitValueType::default();
    let new: UnitValueType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_UomType() {
    let orig = UomType::default();
    let new: UomType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_VersionType() {
    let orig = VersionType::default();
    let new: VersionType = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_MirrorMeterReading() {
    let orig = MirrorMeterReading::default();
    let new: MirrorMeterReading = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_MirrorMeterReadingList() {
    let orig = MirrorMeterReadingList::default();
    let new: MirrorMeterReadingList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_MeterReadingBase() {
    let orig = MeterReadingBase::default();
    let new: MeterReadingBase = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_MirrorReadingSet() {
    let orig = MirrorReadingSet::default();
    let new: MirrorReadingSet = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_MirrorUsagePoint() {
    let orig = MirrorUsagePoint::default();
    let new: MirrorUsagePoint = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_MirrorUsagePointList() {
    let orig = MirrorUsagePointList::default();
    let new: MirrorUsagePointList = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ReadingBase() {
    let orig = ReadingBase::default();
    let new: ReadingBase = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ReadingSetBase() {
    let orig = ReadingSetBase::default();
    let new: ReadingSetBase = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_UsagePointBase() {
    let orig = UsagePointBase::default();
    let new: UsagePointBase = deserialize(&serialize(&orig).unwrap()).unwrap();
    assert_eq!(orig, new);
}
