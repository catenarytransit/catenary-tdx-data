use serde::{Deserialize, Serialize};

//just a lot of struct to deserialize (decerealize? un-cornflæke?) everything
/*
done list: TrainLiveBoard, Routes (bus),
Operator, Schedule (bus), FirstLastTripInfo (bus), Shape (bus),
RouteFare (bus), Stop (bus)

*/

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TrainLiveBoard {
    update_time: String,
    update_interval: i64,
    src_update_time: String,
    src_update_interval: i64,
    authority_code: String,
    train_live_boards: Vec<TrainLiveBoardElement>,
    count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TrainLiveBoardElement {
    train_no: String,
    #[serde(rename = "TrainTypeID")]
    train_type_id: String,
    train_type_code: String,
    train_type_name: NameType,
    #[serde(rename = "StationID")]
    station_id: String,
    station_name: NameType,
    train_station_status: i64,
    delay_time: i64,
    update_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NameType {
    #[serde(rename = "Zh_tw")]
    zh_tw: String,
    en: String,
}

pub type BusRoutes = Vec<BusRoute>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusRoute {
    #[serde(rename = "RouteUID")]
    route_uid: String,
    #[serde(rename = "RouteID")]
    route_id: String,
    has_sub_routes: bool,
    operators: Vec<Operator>,
    #[serde(rename = "AuthorityID")]
    authority_id: String,
    #[serde(rename = "ProviderID")]
    provider_id: String,
    sub_routes: Vec<SubRoute>,
    bus_route_type: i64,
    route_name: NameType,
    departure_stop_name_zh: String,
    departure_stop_name_en: String,
    destination_stop_name_zh: String,
    destination_stop_name_en: String,
    ticket_price_description_zh: String,
    ticket_price_description_en: String,
    fare_buffer_zone_description_zh: String,
    fare_buffer_zone_description_en: String,
    route_map_image_url: String,
    city: String,
    city_code: String,
    update_time: String,
    #[serde(rename = "VersionID")]
    version_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Operator {
    #[serde(rename = "OperatorID")]
    operator_id: String,
    operator_name: NameType,
    operator_code: String,
    operator_no: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubRoute {
    #[serde(rename = "SubRouteUID")]
    sub_route_uid: String,
    #[serde(rename = "SubRouteID")]
    sub_route_id: String,
    operator_i_ds: Vec<String>,
    sub_route_name: NameType,
    headsign: String,
    headsign_en: String,
    direction: i64,
    first_bus_time: String,
    last_bus_time: String,
    holiday_first_bus_time: String,
    holiday_last_bus_time: String,
}

pub type Stop = Vec<StopElement>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StopElement {
    #[serde(rename = "StopUID")]
    stop_uid: String,
    #[serde(rename = "StopID")]
    stop_id: String,
    #[serde(rename = "AuthorityID")]
    authority_id: String,
    stop_name: StopName,
    stop_position: StopPosition,
    stop_address: String,
    bearing: String,
    #[serde(rename = "StationID")]
    station_id: String,
    #[serde(rename = "StationGroupID")]
    station_group_id: String,
    stop_description: String,
    city: String,
    city_code: String,
    location_city_code: String,
    update_time: String,
    #[serde(rename = "VersionID")]
    version_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StopName {
    #[serde(rename = "Zh_tw")]
    zh_tw: String,
    en: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StopPosition {
    position_lon: i64,
    position_lat: i64,
    geo_hash: String,
}

pub type Operator = Vec<OperatorElement>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OperatorElement {
    #[serde(rename = "ProviderID")]
    provider_id: String,
    #[serde(rename = "OperatorID")]
    operator_id: String,
    operator_name: OperatorName,
    operator_phone: String,
    operator_email: String,
    operator_url: String,
    reservation_url: String,
    reservation_phone: String,
    operator_code: String,
    authority_code: String,
    sub_authority_code: String,
    operator_no: String,
    update_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OperatorName {
    #[serde(rename = "Zh_tw")]
    zh_tw: String,
    en: String,
}

pub type Schedule = Vec<ScheduleElement>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScheduleElement {
    #[serde(rename = "RouteUID")]
    route_uid: String,
    #[serde(rename = "RouteID")]
    route_id: String,
    route_name: Name,
    #[serde(rename = "SubRouteUID")]
    sub_route_uid: String,
    #[serde(rename = "SubRouteID")]
    sub_route_id: String,
    sub_route_name: Name,
    direction: i64,
    #[serde(rename = "OperatorID")]
    operator_id: String,
    operator_code: String,
    operator_no: String,
    timetables: Vec<Timetable>,
    frequencys: Vec<Frequency>,
    update_time: String,
    #[serde(rename = "VersionID")]
    version_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Frequency {
    start_time: String,
    end_time: String,
    min_headway_mins: i64,
    max_headway_mins: i64,
    service_day: ServiceDay,
    special_days: Vec<SpecialDay>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceDay {
    service_tag: String,
    sunday: i64,
    monday: i64,
    tuesday: i64,
    wednesday: i64,
    thursday: i64,
    friday: i64,
    saturday: i64,
    national_holidays: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SpecialDay {
    dates: Vec<String>,
    date_period: DatePeriod,
    service_status: i64,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DatePeriod {
    start_date: String,
    end_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Name {
    #[serde(rename = "Zh_tw")]
    zh_tw: String,
    en: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Timetable {
    #[serde(rename = "TripID")]
    trip_id: String,
    is_low_floor: bool,
    service_day: ServiceDay,
    special_days: Vec<SpecialDay>,
    stop_times: Vec<StopTime>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StopTime {
    stop_sequence: i64,
    #[serde(rename = "StopUID")]
    stop_uid: String,
    #[serde(rename = "StopID")]
    stop_id: String,
    stop_name: Name,
    arrival_time: String,
    departure_time: String,
}


pub type FirstLastTripInfo = Vec<FirstLastTripInfoElement>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FirstLastTripInfoElement {
    #[serde(rename = "RouteUID")]
    route_uid: String,
    #[serde(rename = "RouteID")]
    route_id: String,
    route_name: RouteName,
    #[serde(rename = "OperatorID")]
    operator_id: String,
    operator_no: String,
    #[serde(rename = "SubRouteUID")]
    sub_route_uid: String,
    #[serde(rename = "SubRouteID")]
    sub_route_id: String,
    sub_route_name: RouteName,
    direction: i64,
    first_last_trips: Vec<FirstLastTrip>,
    update_time: String,
    #[serde(rename = "VersionID")]
    version_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FirstLastTrip {
    service_day: ServiceDay,
    first_trip_dep_time: String,
    last_trip_dep_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceDay {
    day_before_national_holiday: i64,
    day_after_national_holiday: i64,
    typhoon_day: i64,
    service_tag: String,
    sunday: i64,
    monday: i64,
    tuesday: i64,
    wednesday: i64,
    thursday: i64,
    friday: i64,
    saturday: i64,
    national_holidays: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RouteName {
    #[serde(rename = "Zh_tw")]
    zh_tw: String,
    en: String,
}

pub type Shape = Vec<ShapeElement>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShapeElement {
    #[serde(rename = "RouteUID")]
    route_uid: String,
    #[serde(rename = "RouteID")]
    route_id: String,
    route_name: RouteName,
    #[serde(rename = "SubRouteUID")]
    sub_route_uid: String,
    #[serde(rename = "SubRouteID")]
    sub_route_id: String,
    sub_route_name: RouteName,
    direction: i64,
    geometry: String,
    encoded_polyline: String,
    update_time: String,
    #[serde(rename = "VersionID")]
    version_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RouteName {
    #[serde(rename = "Zh_tw")]
    zh_tw: String,
    en: String,
}

pub type RouteFare = Vec<RouteFareElement>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RouteFareElement {
    #[serde(rename = "RouteID")]
    route_id: String,
    route_name: String,
    #[serde(rename = "OperatorID")]
    operator_id: String,
    operator_no: String,
    #[serde(rename = "SubRouteID")]
    sub_route_id: String,
    sub_route_name: String,
    fare_pricing_type: i64,
    is_free_bus: i64,
    is_for_all_sub_routes: i64,
    section_fares: Vec<SectionFare>,
    stage_fares: Vec<StageFare>,
    #[serde(rename = "ODFares")]
    od_fares: Vec<OdFare>,
    update_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OdFare {
    direction: i64,
    origin_stop: DestinationStop,
    destination_stop: DestinationStop,
    fares: Vec<OdFareFare>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DestinationStop {
    #[serde(rename = "StopID")]
    stop_id: String,
    stop_name: String,
    sequence: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OdFareFare {
    ticket_type: i64,
    fare_class: i64,
    price: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SectionFare {
    buffer_zones: Vec<BufferZone>,
    fares: Vec<OdFareFare>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BufferZone {
    #[serde(rename = "ZoneID")]
    zone_id: String,
    section_sequence: i64,
    direction: i64,
    fare_buffer_zone_origin: DestinationStop,
    fare_buffer_zone_destination: DestinationStop,
    buffer_zone_description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StageFare {
    direction: i64,
    origin_stage: DestinationStop,
    destination_stage: DestinationStop,
    fares: Vec<StageFareFare>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StageFareFare {
    fare_name: String,
    ticket_type: i64,
    fare_class: i64,
    price: i64,
    discount_periods: Vec<DiscountPeriod>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DiscountPeriod {
    service_day: ServiceDay,
    start_time: String,
    end_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceDay {
    service_tag: String,
    sunday: i64,
    monday: i64,
    tuesday: i64,
    wednesday: i64,
    thursday: i64,
    friday: i64,
    saturday: i64,
    national_holidays: i64,
}
