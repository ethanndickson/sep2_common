#![allow(non_snake_case)]
use common::config::YASERDE_CFG;
// TODO Ethan: Temporary import all
use common::identification::*;
use common::objects::*;
use common::xsd::*;
use yaserde::de::from_str;
use yaserde::ser::to_string_with_config;

#[test]
fn default_DeviceCapability() {
    let orig = DeviceCapability::default();
    let new: DeviceCapability = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_AbstractDevice() {
    let orig = AbstractDevice::default();
    let new: AbstractDevice = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DeviceStatus() {
    let orig = DeviceStatus::default();
    let new: DeviceStatus = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_EndDevice() {
    let orig = EndDevice::default();
    let new: EndDevice = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_EndDeviceList() {
    let orig = EndDeviceList::default();
    let new: EndDeviceList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Registration() {
    let orig = Registration::default();
    let new: Registration = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SelfDevice() {
    let orig = SelfDevice::default();
    let new: SelfDevice = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Temperature() {
    let orig = Temperature::default();
    let new: Temperature = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FunctionSetAssignmentsBase() {
    let orig = FunctionSetAssignmentsBase::default();
    let new: FunctionSetAssignmentsBase = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FunctionSetAssignments() {
    let orig = FunctionSetAssignments::default();
    let new: FunctionSetAssignments = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FunctionSetAssignmentsList() {
    let orig = FunctionSetAssignmentsList::default();
    let new: FunctionSetAssignmentsList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Condition() {
    let orig = Condition::default();
    let new: Condition = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SubscriptionBase() {
    let orig = SubscriptionBase::default();
    let new: SubscriptionBase = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Subscription() {
    let orig = Subscription::default();
    let new: Subscription = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SubscriptionList() {
    let orig = SubscriptionList::default();
    let new: SubscriptionList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Notification() {
    let orig = Notification::default();
    let new: Notification = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_NotificationList() {
    let orig = NotificationList::default();
    let new: NotificationList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERControlResponse() {
    let orig = DercontrolResponse::default();
    let new: DercontrolResponse = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FlowReservationResponseResponse() {
    let orig = FlowReservationResponseResponse::default();
    let new: FlowReservationResponseResponse = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_AppliedTargetReduction() {
    let orig = AppliedTargetReduction::default();
    let new: AppliedTargetReduction = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DrResponse() {
    let orig = DrResponse::default();
    let new: DrResponse = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PriceResponse() {
    let orig = PriceResponse::default();
    let new: PriceResponse = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}


#[test]
fn default_ResponseList() {
    let orig = ResponseList::default();
    let new: ResponseList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ResponseSet() {
    let orig = ResponseSet::default();
    let new: ResponseSet = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ResponseSetList() {
    let orig = ResponseSetList::default();
    let new: ResponseSetList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TextResponse() {
    let orig = TextResponse::default();
    let new: TextResponse = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Time() {
    let orig = Time::default();
    let new: Time = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DeviceInformation() {
    let orig = DeviceInformation::default();
    let new: DeviceInformation = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DRLCCapabilities() {
    let orig = Drlccapabilities::default();
    let new: Drlccapabilities = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SupportedLocale() {
    let orig = SupportedLocale::default();
    let new: SupportedLocale = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SupportedLocaleList() {
    let orig = SupportedLocaleList::default();
    let new: SupportedLocaleList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PowerStatus() {
    let orig = PowerStatus::default();
    let new: PowerStatus = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PowerSourceType() {
    let orig = PowerSourceType::default();
    let new: PowerSourceType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PEVInfo() {
    let orig = Pevinfo::default();
    let new: Pevinfo = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}


#[test]
fn default_IPAddr() {
    let orig = Ipaddr::default();
    let new: Ipaddr = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_IPAddrList() {
    let orig = IpaddrList::default();
    let new: IpaddrList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_IPInterface() {
    let orig = Ipinterface::default();
    let new: Ipinterface = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_IPInterfaceList() {
    let orig = IpinterfaceList::default();
    let new: IpinterfaceList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_LLInterface() {
    let orig = Llinterface::default();
    let new: Llinterface = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_LLInterfaceList() {
    let orig = LlinterfaceList::default();
    let new: LlinterfaceList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_loWPAN() {
    let orig = LoWPAN::default();
    let new: LoWPAN = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Neighbor() {
    let orig = Neighbor::default();
    let new: Neighbor = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_NeighborList() {
    let orig = NeighborList::default();
    let new: NeighborList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RPLInstance() {
    let orig = Rplinstance::default();
    let new: Rplinstance = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RPLInstanceList() {
    let orig = RplinstanceList::default();
    let new: RplinstanceList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RPLSourceRoutes() {
    let orig = RplsourceRoutes::default();
    let new: RplsourceRoutes = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RPLSourceRoutesList() {
    let orig = RplsourceRoutesList::default();
    let new: RplsourceRoutesList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_LogEvent() {
    let orig = LogEvent::default();
    let new: LogEvent = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_LogEventList() {
    let orig = LogEventList::default();
    let new: LogEventList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Configuration() {
    let orig = Configuration::default();
    let new: Configuration = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PowerConfiguration() {
    let orig = PowerConfiguration::default();
    let new: PowerConfiguration = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PriceResponseCfg() {
    let orig = PriceResponseCfg::default();
    let new: PriceResponseCfg = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PriceResponseCfgList() {
    let orig = PriceResponseCfgList::default();
    let new: PriceResponseCfgList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TimeConfiguration() {
    let orig = TimeConfiguration::default();
    let new: TimeConfiguration = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_File() {
    let orig = File::default();
    let new: File = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FileList() {
    let orig = FileList::default();
    let new: FileList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FileStatus() {
    let orig = FileStatus::default();
    let new: FileStatus = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_LoadShedAvailabilityList() {
    let orig = LoadShedAvailabilityList::default();
    let new: LoadShedAvailabilityList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ApplianceLoadReduction() {
    let orig = ApplianceLoadReduction::default();
    let new: ApplianceLoadReduction = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DemandResponseProgramList() {
    let orig = DemandResponseProgramList::default();
    let new: DemandResponseProgramList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DutyCycle() {
    let orig = DutyCycle::default();
    let new: DutyCycle = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_EndDeviceControlList() {
    let orig = EndDeviceControlList::default();
    let new: EndDeviceControlList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_LoadShedAvailability() {
    let orig = LoadShedAvailability::default();
    let new: LoadShedAvailability = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Offset() {
    let orig = Offset::default();
    let new: Offset = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SetPoint() {
    let orig = SetPoint::default();
    let new: SetPoint = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TargetReduction() {
    let orig = TargetReduction::default();
    let new: TargetReduction = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_MeterReading() {
    let orig = MeterReading::default();
    let new: MeterReading = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_MeterReadingList() {
    let orig = MeterReadingList::default();
    let new: MeterReadingList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Reading() {
    let orig = Reading::default();
    let new: Reading = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ReadingList() {
    let orig = ReadingList::default();
    let new: ReadingList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ReadingSet() {
    let orig = ReadingSet::default();
    let new: ReadingSet = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ReadingSetList() {
    let orig = ReadingSetList::default();
    let new: ReadingSetList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ReadingType() {
    let orig = ReadingType::default();
    let new: ReadingType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_UsagePoint() {
    let orig = UsagePoint::default();
    let new: UsagePoint = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_UsagePointList() {
    let orig = UsagePointList::default();
    let new: UsagePointList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ConsumptionTariffInterval() {
    let orig = ConsumptionTariffInterval::default();
    let new: ConsumptionTariffInterval = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ConsumptionTariffIntervalList() {
    let orig = ConsumptionTariffIntervalList::default();
    let new: ConsumptionTariffIntervalList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CostKindType() {
    let orig = CostKindType::default();
    let new: CostKindType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_EnvironmentalCost() {
    let orig = EnvironmentalCost::default();
    let new: EnvironmentalCost = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RateComponent() {
    let orig = RateComponent::default();
    let new: RateComponent = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RateComponentList() {
    let orig = RateComponentList::default();
    let new: RateComponentList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TariffProfileList() {
    let orig = TariffProfileList::default();
    let new: TariffProfileList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TimeTariffIntervalList() {
    let orig = TimeTariffIntervalList::default();
    let new: TimeTariffIntervalList  = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_MessagingProgramList() {
    let orig = MessagingProgramList::default();
    let new: MessagingProgramList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PriorityType() {
    let orig = PriorityType::default();
    let new: PriorityType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TextMessageList() {
    let orig = TextMessageList::default();
    let new: TextMessageList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_BillingPeriod() {
    let orig = BillingPeriod::default();
    let new: BillingPeriod = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_BillingPeriodList() {
    let orig = BillingPeriodList::default();
    let new: BillingPeriodList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_BillingMeterReadingBase() {
    let orig = BillingMeterReadingBase::default();
    let new: BillingMeterReadingBase = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_BillingReading() {
    let orig = BillingReading::default();
    let new: BillingReading = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_BillingReadingList() {
    let orig = BillingReadingList::default();
    let new: BillingReadingList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_BillingReadingSet() {
    let orig = BillingReadingSet::default();
    let new: BillingReadingSet = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_BillingReadingSetList() {
    let orig = BillingReadingSetList::default();
    let new: BillingReadingSetList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Charge() {
    let orig = Charge::default();
    let new: Charge = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ChargeKind() {
    let orig = ChargeKind::default();
    let new: ChargeKind = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CustomerAccount() {
    let orig = CustomerAccount::default();
    let new: CustomerAccount = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CustomerAccountList() {
    let orig = CustomerAccountList::default();
    let new: CustomerAccountList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CustomerAgreement() {
    let orig = CustomerAgreement::default();
    let new: CustomerAgreement = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CustomerAgreementList() {
    let orig = CustomerAgreementList::default();
    let new: CustomerAgreementList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_HistoricalReading() {
    let orig = HistoricalReading::default();
    let new: HistoricalReading = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_HistoricalReadingList() {
    let orig = HistoricalReadingList::default();
    let new: HistoricalReadingList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ProjectionReading() {
    let orig = ProjectionReading::default();
    let new: ProjectionReading = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ProjectionReadingList() {
    let orig = ProjectionReadingList::default();
    let new: ProjectionReadingList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TargetReading() {
    let orig = TargetReading::default();
    let new: TargetReading = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TargetReadingList() {
    let orig = TargetReadingList::default();
    let new: TargetReadingList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ServiceSupplier() {
    let orig = ServiceSupplier::default();
    let new: ServiceSupplier = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ServiceSupplierList() {
    let orig = ServiceSupplierList::default();
    let new: ServiceSupplierList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_AccountBalance() {
    let orig = AccountBalance::default();
    let new: AccountBalance = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_AccountingUnit() {
    let orig = AccountingUnit::default();
    let new: AccountingUnit = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CreditRegister() {
    let orig = CreditRegister::default();
    let new: CreditRegister = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CreditRegisterList() {
    let orig = CreditRegisterList::default();
    let new: CreditRegisterList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Prepayment() {
    let orig = Prepayment::default();
    let new: Prepayment = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PrepaymentList() {
    let orig = PrepaymentList::default();
    let new: PrepaymentList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PrepayModeType() {
    let orig = PrepayModeType::default();
    let new: PrepayModeType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PrepayOperationStatus() {
    let orig = PrepayOperationStatus::default();
    let new: PrepayOperationStatus = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ServiceChange() {
    let orig = ServiceChange::default();
    let new: ServiceChange = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SupplyInterruptionOverride() {
    let orig = SupplyInterruptionOverride::default();
    let new: SupplyInterruptionOverride = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SupplyInterruptionOverrideList() {
    let orig = SupplyInterruptionOverrideList::default();
    let new: SupplyInterruptionOverrideList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CreditStatusType() {
    let orig = CreditStatusType::default();
    let new: CreditStatusType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CreditTypeType() {
    let orig = CreditTypeType::default();
    let new: CreditTypeType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CreditTypeChange() {
    let orig = CreditTypeChange::default();
    let new: CreditTypeChange = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ServiceStatusType() {
    let orig = ServiceStatusType::default();
    let new: ServiceStatusType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RequestStatus() {
    let orig = RequestStatus::default();
    let new: RequestStatus = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FlowReservationRequest() {
    let orig = FlowReservationRequest::default();
    let new: FlowReservationRequest = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FlowReservationRequestList() {
    let orig = FlowReservationRequestList::default();
    let new: FlowReservationRequestList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FlowReservationResponse() {
    let orig = FlowReservationResponse::default();
    let new: FlowReservationResponse = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FlowReservationResponseList() {
    let orig = FlowReservationResponseList::default();
    let new: FlowReservationResponseList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DefaultDERControl() {
    let orig = DefaultDERControl::default();
    let new: DefaultDERControl = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FreqDroopType() {
    let orig = FreqDroopType::default();
    let new: FreqDroopType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DER() {
    let orig = Der::default();
    let new: Der = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERList() {
    let orig = Derlist::default();
    let new: Derlist = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERSettings() {
    let orig = Dersettings::default();
    let new: Dersettings = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERType() {
    let orig = Dertype::default();
    let new: Dertype = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERAvailability() {
    let orig = Deravailability::default();
    let new: Deravailability = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERCapability() {
    let orig = Dercapability::default();
    let new: Dercapability = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERControlBase() {
    let orig = DercontrolBase::default();
    let new: DercontrolBase = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERControlList() {
    let orig = DercontrolList::default();
    let new: DercontrolList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERControlType() {
    let orig = DercontrolType::default();
    let new: DercontrolType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERCurve() {
    let orig = Dercurve::default();
    let new: Dercurve = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CurrentDERProgramLink() {
    let orig = CurrentDERProgramLink::default();
    let new: CurrentDERProgramLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERCurveList() {
    let orig = DercurveList::default();
    let new: DercurveList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CurveData() {
    let orig = CurveData::default();
    let new: CurveData = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERCurveType() {
    let orig = DercurveType::default();
    let new: DercurveType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERProgram() {
    let orig = Derprogram::default();
    let new: Derprogram = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERProgramList() {
    let orig = DerprogramList::default();
    let new: DerprogramList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERStatus() {
    let orig = Derstatus::default();
    let new: Derstatus = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERUnitRefType() {
    let orig = DerunitRefType::default();
    let new: DerunitRefType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CurrentRMS() {
    let orig = CurrentRMS::default();
    let new: CurrentRMS = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FixedPointType() {
    let orig = FixedPointType::default();
    let new: FixedPointType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_UnsignedFixedPointType() {
    let orig = UnsignedFixedPointType::default();
    let new: UnsignedFixedPointType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ActivePower() {
    let orig = ActivePower::default();
    let new: ActivePower = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_AmpereHour() {
    let orig = AmpereHour::default();
    let new: AmpereHour = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ApparentPower() {
    let orig = ApparentPower::default();
    let new: ApparentPower = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ReactivePower() {
    let orig = ReactivePower::default();
    let new: ReactivePower = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ReactiveSusceptance() {
    let orig = ReactiveSusceptance::default();
    let new: ReactiveSusceptance = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PowerFactor() {
    let orig = PowerFactor::default();
    let new: PowerFactor = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PowerFactorWithExcitation() {
    let orig = PowerFactorWithExcitation::default();
    let new: PowerFactorWithExcitation = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FixedVar() {
    let orig = FixedVar::default();
    let new: FixedVar = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_WattHour() {
    let orig = WattHour::default();
    let new: WattHour = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_VoltageRMS() {
    let orig = VoltageRMS::default();
    let new: VoltageRMS = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ConnectStatusType() {
    let orig = ConnectStatusType::default();
    let new: ConnectStatusType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_InverterStatusType() {
    let orig = InverterStatusType::default();
    let new: InverterStatusType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_LocalControlModeStatusType() {
    let orig = LocalControlModeStatusType::default();
    let new: LocalControlModeStatusType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ManufacturerStatusType() {
    let orig = ManufacturerStatusType::default();
    let new: ManufacturerStatusType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_OperationalModeStatusType() {
    let orig = OperationalModeStatusType::default();
    let new: OperationalModeStatusType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_StateOfChargeStatusType() {
    let orig = StateOfChargeStatusType::default();
    let new: StateOfChargeStatusType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_StorageModeStatusType() {
    let orig = StorageModeStatusType::default();
    let new: StorageModeStatusType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_AccountBalanceLink() {
    let orig = AccountBalanceLink::default();
    let new: AccountBalanceLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ActiveBillingPeriodListLink() {
    let orig = ActiveBillingPeriodListLink::default();
    let new: ActiveBillingPeriodListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ActiveCreditRegisterListLink() {
    let orig = ActiveCreditRegisterListLink::default();
    let new: ActiveCreditRegisterListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ActiveDERControlListLink() {
    let orig = ActiveDERControlListLink::default();
    let new: ActiveDERControlListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ActiveEndDeviceControlListLink() {
    let orig = ActiveEndDeviceControlListLink::default();
    let new: ActiveEndDeviceControlListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ActiveFlowReservationListLink() {
    let orig = ActiveFlowReservationListLink::default();
    let new: ActiveFlowReservationListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ActiveProjectionReadingListLink() {
    let orig = ActiveProjectionReadingListLink::default();
    let new: ActiveProjectionReadingListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ActiveSupplyInterruptionOverrideListLink() {
    let orig = ActiveSupplyInterruptionOverrideListLink::default();
    let new: ActiveSupplyInterruptionOverrideListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ActiveTargetReadingListLink() {
    let orig = ActiveTargetReadingListLink::default();
    let new: ActiveTargetReadingListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ActiveTextMessageListLink() {
    let orig = ActiveTextMessageListLink::default();
    let new: ActiveTextMessageListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ActiveTimeTariffIntervalListLink() {
    let orig = ActiveTimeTariffIntervalListLink::default();
    let new: ActiveTimeTariffIntervalListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_AssociatedDERProgramListLink() {
    let orig = AssociatedDERProgramListLink::default();
    let new: AssociatedDERProgramListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_AssociatedUsagePointLink() {
    let orig = AssociatedUsagePointLink::default();
    let new: AssociatedUsagePointLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_BillingPeriodListLink() {
    let orig = BillingPeriodListLink::default();
    let new: BillingPeriodListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_BillingReadingListLink() {
    let orig = BillingReadingListLink::default();
    let new: BillingReadingListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_BillingReadingSetListLink() {
    let orig = BillingReadingSetListLink::default();
    let new: BillingReadingSetListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ConfigurationLink() {
    let orig = ConfigurationLink::default();
    let new: ConfigurationLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ConsumptionTariffIntervalListLink() {
    let orig = ConsumptionTariffIntervalListLink::default();
    let new: ConsumptionTariffIntervalListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CreditRegisterListLink() {
    let orig = CreditRegisterListLink::default();
    let new: CreditRegisterListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CustomerAccountLink() {
    let orig = CustomerAccountLink::default();
    let new: CustomerAccountLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CustomerAccountListLink() {
    let orig = CustomerAccountListLink::default();
    let new: CustomerAccountListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CustomerAgreementListLink() {
    let orig = CustomerAgreementListLink::default();
    let new: CustomerAgreementListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DemandResponseProgramLink() {
    let orig = DemandResponseProgramLink::default();
    let new: DemandResponseProgramLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DemandResponseProgramListLink() {
    let orig = DemandResponseProgramListLink::default();
    let new: DemandResponseProgramListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERAvailabilityLink() {
    let orig = DeravailabilityLink::default();
    let new: DeravailabilityLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DefaultDERControlLink() {
    let orig = DefaultDERControlLink::default();
    let new: DefaultDERControlLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERCapabilityLink() {
    let orig = DercapabilityLink::default();
    let new: DercapabilityLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERControlListLink() {
    let orig = DercontrolListLink::default();
    let new: DercontrolListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERCurveLink() {
    let orig = DercurveLink::default();
    let new: DercurveLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERCurveListLink() {
    let orig = DercurveListLink::default();
    let new: DercurveListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERLink() {
    let orig = Derlink::default();
    let new: Derlink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERListLink() {
    let orig = DerlistLink::default();
    let new: DerlistLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERProgramLink() {
    let orig = DerprogramLink::default();
    let new: DerprogramLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERProgramListLink() {
    let orig = DerprogramListLink::default();
    let new: DerprogramListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERSettingsLink() {
    let orig = DersettingsLink::default();
    let new: DersettingsLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DERStatusLink() {
    let orig = DerstatusLink::default();
    let new: DerstatusLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DeviceCapabilityLink() {
    let orig = DeviceCapabilityLink::default();
    let new: DeviceCapabilityLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DeviceInformationLink() {
    let orig = DeviceInformationLink::default();
    let new: DeviceInformationLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DeviceStatusLink() {
    let orig = DeviceStatusLink::default();
    let new: DeviceStatusLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_EndDeviceControlListLink() {
    let orig = EndDeviceControlListLink::default();
    let new: EndDeviceControlListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_EndDeviceLink() {
    let orig = EndDeviceLink::default();
    let new: EndDeviceLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_EndDeviceListLink() {
    let orig = EndDeviceListLink::default();
    let new: EndDeviceListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FileLink() {
    let orig = FileLink::default();
    let new: FileLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FileListLink() {
    let orig = FileListLink::default();
    let new: FileListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FileStatusLink() {
    let orig = FileStatusLink::default();
    let new: FileStatusLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FlowReservationRequestListLink() {
    let orig = FlowReservationRequestListLink::default();
    let new: FlowReservationRequestListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FlowReservationResponseListLink() {
    let orig = FlowReservationResponseListLink::default();
    let new: FlowReservationResponseListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FunctionSetAssignmentsListLink() {
    let orig = FunctionSetAssignmentsListLink::default();
    let new: FunctionSetAssignmentsListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_HistoricalReadingListLink() {
    let orig = HistoricalReadingListLink::default();
    let new: HistoricalReadingListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_IPAddrListLink() {
    let orig = IpaddrListLink::default();
    let new: IpaddrListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_IPInterfaceListLink() {
    let orig = IpinterfaceListLink::default();
    let new: IpinterfaceListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_LLInterfaceListLink() {
    let orig = LlinterfaceListLink::default();
    let new: LlinterfaceListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_LoadShedAvailabilityListLink() {
    let orig = LoadShedAvailabilityListLink::default();
    let new: LoadShedAvailabilityListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_LogEventListLink() {
    let orig = LogEventListLink::default();
    let new: LogEventListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_MessagingProgramListLink() {
    let orig = MessagingProgramListLink::default();
    let new: MessagingProgramListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_MeterReadingLink() {
    let orig = MeterReadingLink::default();
    let new: MeterReadingLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_MeterReadingListLink() {
    let orig = MeterReadingListLink::default();
    let new: MeterReadingListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_MirrorUsagePointListLink() {
    let orig = MirrorUsagePointListLink::default();
    let new: MirrorUsagePointListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_NeighborListLink() {
    let orig = NeighborListLink::default();
    let new: NeighborListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_NotificationListLink() {
    let orig = NotificationListLink::default();
    let new: NotificationListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PowerStatusLink() {
    let orig = PowerStatusLink::default();
    let new: PowerStatusLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PrepaymentLink() {
    let orig = PrepaymentLink::default();
    let new: PrepaymentLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PrepaymentListLink() {
    let orig = PrepaymentListLink::default();
    let new: PrepaymentListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PrepayOperationStatusLink() {
    let orig = PrepayOperationStatusLink::default();
    let new: PrepayOperationStatusLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PriceResponseCfgListLink() {
    let orig = PriceResponseCfgListLink::default();
    let new: PriceResponseCfgListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ProjectionReadingListLink() {
    let orig = ProjectionReadingListLink::default();
    let new: ProjectionReadingListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RateComponentLink() {
    let orig = RateComponentLink::default();
    let new: RateComponentLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RateComponentListLink() {
    let orig = RateComponentListLink::default();
    let new: RateComponentListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ReadingLink() {
    let orig = ReadingLink::default();
    let new: ReadingLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ReadingListLink() {
    let orig = ReadingListLink::default();
    let new: ReadingListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ReadingSetListLink() {
    let orig = ReadingSetListLink::default();
    let new: ReadingSetListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ReadingTypeLink() {
    let orig = ReadingTypeLink::default();
    let new: ReadingTypeLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RegistrationLink() {
    let orig = RegistrationLink::default();
    let new: RegistrationLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ResponseListLink() {
    let orig = ResponseListLink::default();
    let new: ResponseListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ResponseSetListLink() {
    let orig = ResponseSetListLink::default();
    let new: ResponseSetListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RPLInstanceListLink() {
    let orig = RplinstanceListLink::default();
    let new: RplinstanceListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RPLSourceRoutesListLink() {
    let orig = RplsourceRoutesListLink::default();
    let new: RplsourceRoutesListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SelfDeviceLink() {
    let orig = SelfDeviceLink::default();
    let new: SelfDeviceLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ServiceSupplierLink() {
    let orig = ServiceSupplierLink::default();
    let new: ServiceSupplierLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SubscriptionListLink() {
    let orig = SubscriptionListLink::default();
    let new: SubscriptionListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SupplyInterruptionOverrideListLink() {
    let orig = SupplyInterruptionOverrideListLink::default();
    let new: SupplyInterruptionOverrideListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SupportedLocaleListLink() {
    let orig = SupportedLocaleListLink::default();
    let new: SupportedLocaleListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TargetReadingListLink() {
    let orig = TargetReadingListLink::default();
    let new: TargetReadingListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TariffProfileLink() {
    let orig = TariffProfileLink::default();
    let new: TariffProfileLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TariffProfileListLink() {
    let orig = TariffProfileListLink::default();
    let new: TariffProfileListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TextMessageListLink() {
    let orig = TextMessageListLink::default();
    let new: TextMessageListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TimeLink() {
    let orig = TimeLink::default();
    let new: TimeLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TimeTariffIntervalListLink() {
    let orig = TimeTariffIntervalListLink::default();
    let new: TimeTariffIntervalListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_UsagePointLink() {
    let orig = UsagePointLink::default();
    let new: UsagePointLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_UsagePointListLink() {
    let orig = UsagePointListLink::default();
    let new: UsagePointListLink = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_Link() {
    let orig = Link::default();
    let new: Link = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_AccumulationBehaviourType() {
    let orig = AccumulationBehaviourType::default();
    let new: AccumulationBehaviourType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ApplianceLoadReductionType() {
    let orig = ApplianceLoadReductionType::default();
    let new: ApplianceLoadReductionType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CommodityType() {
    let orig = CommodityType::default();
    let new: CommodityType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ConsumptionBlockType() {
    let orig = ConsumptionBlockType::default();
    let new: ConsumptionBlockType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_CurrencyCode() {
    let orig = CurrencyCode::default();
    let new: CurrencyCode = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DataQualifierType() {
    let orig = DataQualifierType::default();
    let new: DataQualifierType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DateTimeInterval() {
    let orig = DateTimeInterval::default();
    let new: DateTimeInterval = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DeviceCategoryType() {
    let orig = DeviceCategoryType::default();
    let new: DeviceCategoryType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_DstRuleType() {
    let orig = DstRuleType::default();
    let new: DstRuleType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_FlowDirectionType() {
    let orig = FlowDirectionType::default();
    let new: FlowDirectionType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_GPSLocationType() {
    let orig = GpslocationType::default();
    let new: GpslocationType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_KindType() {
    let orig = KindType::default();
    let new: KindType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_LocaleType() {
    let orig = LocaleType::default();
    let new: LocaleType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_mRIDType() {
    let orig = Mridtype::default();
    let new: Mridtype = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_OneHourRangeType() {
    let orig = OneHourRangeType::default();
    let new: OneHourRangeType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PENType() {
    let orig = Pentype::default();
    let new: Pentype = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PerCent() {
    let orig = PerCent::default();
    let new: PerCent = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PhaseCode() {
    let orig = PhaseCode::default();
    let new: PhaseCode = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PINType() {
    let orig = Pintype::default();
    let new: Pintype = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_PowerOfTenMultiplierType() {
    let orig = PowerOfTenMultiplierType::default();
    let new: PowerOfTenMultiplierType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RealEnergy() {
    let orig = RealEnergy::default();
    let new: RealEnergy = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_RoleFlagsType() {
    let orig = RoleFlagsType::default();
    let new: RoleFlagsType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ServiceKind() {
    let orig = ServiceKind::default();
    let new: ServiceKind = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SFDIType() {
    let orig = Sfditype::default();
    let new: Sfditype = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SignedPerCent() {
    let orig = SignedPerCent::default();
    let new: SignedPerCent = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_SignedRealEnergy() {
    let orig = SignedRealEnergy::default();
    let new: SignedRealEnergy = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TimeOffsetType() {
    let orig = TimeOffsetType::default();
    let new: TimeOffsetType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TimeType() {
    let orig = TimeType::default();
    let new: TimeType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_TOUType() {
    let orig = Toutype::default();
    let new: Toutype = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_UnitType() {
    let orig = UnitType::default();
    let new: UnitType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_UnitValueType() {
    let orig = UnitValueType::default();
    let new: UnitValueType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_UomType() {
    let orig = UomType::default();
    let new: UomType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_VersionType() {
    let orig = VersionType::default();
    let new: VersionType = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_MirrorMeterReading() {
    let orig = MirrorMeterReading::default();
    let new: MirrorMeterReading = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_MirrorMeterReadingList() {
    let orig = MirrorMeterReadingList::default();
    let new: MirrorMeterReadingList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_MeterReadingBase() {
    let orig = MeterReadingBase::default();
    let new: MeterReadingBase = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_MirrorReadingSet() {
    let orig = MirrorReadingSet::default();
    let new: MirrorReadingSet = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_MirrorUsagePoint() {
    let orig = MirrorUsagePoint::default();
    let new: MirrorUsagePoint = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_MirrorUsagePointList() {
    let orig = MirrorUsagePointList::default();
    let new: MirrorUsagePointList = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ReadingBase() {
    let orig = ReadingBase::default();
    let new: ReadingBase = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_ReadingSetBase() {
    let orig = ReadingSetBase::default();
    let new: ReadingSetBase = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}

#[test]
fn default_UsagePointBase() {
    let orig = UsagePointBase::default();
    let new: UsagePointBase = from_str(&to_string_with_config(&orig, &YASERDE_CFG).unwrap()).unwrap();
    assert_eq!(orig, new);
}
