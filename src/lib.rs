use serde::{Deserialize, Serialize};

//just a lot of struct to deserialize (decerealize? un-cornflæke?) everything
pub type Operators = Vec<Operator>; //top level works for v2 bus and rail

pub type BusRoutes = Vec<BusRoute>; //top level
pub type BusStops = Vec<BusStop>; //top level
pub type BusSchedules = Vec<BusSchedule>; //top level
pub type FirstLastTripInfo = Vec<FirstLastTripInfoElement>; //top level
pub type BusShapes = Vec<Shape>; //top level
pub type BusRouteFare = Vec<BusRouteFareElement>; //top level

pub type RailStations = Vec<RailStation>; //top level
pub type RailRoutes = Vec<RailRoute>; //top level
pub type FirstLastTimetables = Vec<FirstLastTimetable>; //top level
pub type RailFrequencies = Vec<RailFrequency>; //top level
pub type RailShapes = Vec<RailShape>; //top level
pub type MetroFares = Vec<MetroFare>; //top level

pub type BusRtFrequency = Vec<BusRtFrequencyElement>; //top level
pub type BusRtStops = Vec<BusRtStop>; //top level
pub type BusEta = Vec<BusEtaElement>; //top level
pub type BusAlerts = Vec<BusAlert>; //top level

pub type MetroLiveBoard = Vec<MetroLiveBoardElement>; //top level
pub type MetroStationTimeTable = Vec<MetroStationTimeTableElement>; //top level
pub type ThsrAlertInfo = Vec<ThsrAlertInfoElement>; //top level

pub type ThsrGeneralTimetables = Vec<ThsrGeneralTimetable>; //top level
pub type ThsrFares = Vec<RailFare>; //top level

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3RailOperators {
    //top level
    update_time: Option<String>,
    update_interval: Option<i64>,
    src_update_time: Option<String>,
    src_update_interval: Option<i64>,
    authority_code: Option<String>,
    operators: Option<Vec<V3RailOperator>>,
    count: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3RailRoutes {
    //top level
    update_time: Option<String>,
    update_interval: Option<i64>,
    src_update_time: Option<String>,
    src_update_interval: Option<i64>,
    authority_code: Option<String>,
    routes: Option<Vec<V3RailRoute>>,
    count: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3RailStations {
    //top level //also works for AFR stations
    update_time: Option<String>,
    update_interval: Option<i64>,
    src_update_time: Option<String>,
    src_update_interval: Option<i64>,
    authority_code: Option<String>,
    stations: Option<Vec<V3RailStation>>,
    count: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3GeneralTrainTimetables {
    //top level
    update_time: Option<String>,
    update_interval: Option<i64>,
    src_update_time: Option<String>,
    src_update_interval: Option<i64>,
    authority_code: Option<String>,
    effective_date: Option<String>,
    expire_date: Option<String>,
    src_version: Option<String>,
    timetable_name: Option<String>,
    validity_desciption: Option<String>,
    train_timetables: Option<Vec<V3TrainTimetable>>,
    count: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3RailShapes {
    //top level
    update_time: Option<String>,
    update_interval: Option<i64>,
    src_update_time: Option<String>,
    src_update_interval: Option<i64>,
    authority_code: Option<String>,
    shapes: Option<Vec<V3RailShape>>,
    count: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3OdFares {
    //top level
    update_time: Option<String>,
    update_interval: Option<i64>,
    src_update_time: Option<String>,
    src_update_interval: Option<i64>,
    authority_code: Option<String>,
    effective_date: Option<String>,
    expire_date: Option<String>,
    src_version: Option<String>,
    #[serde(rename = "ODFares")]
    od_fares: Option<Vec<RailFare>>,
    count: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TrainLiveBoard {
    //top level
    update_time: Option<String>,
    update_interval: Option<i64>,
    src_update_time: Option<String>,
    src_update_interval: Option<i64>,
    authority_code: Option<String>,
    train_live_boards: Option<Vec<TrainLiveBoardElement>>,
    count: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StationLiveBoard {
    //top level
    update_time: Option<String>,
    update_interval: Option<i64>,
    src_update_time: Option<String>,
    src_update_interval: Option<i64>,
    authority_code: Option<String>,
    station_live_boards: Option<Vec<StationLiveBoardElement>>,
    count: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RailAlerts {
    //top level  //Metro alert and TRA alerts
    update_time: Option<String>,
    update_interval: Option<i64>,
    src_update_time: Option<String>,
    src_update_interval: Option<i64>,
    authority_code: Option<String>,
    alerts: Option<Vec<RailAlert>>,
    count: Option<i64>,
}

//sublevel structs

//BusRoutes
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusRoute {
    #[serde(rename = "RouteUID")]
    route_uid: Option<String>,
    #[serde(rename = "RouteID")]
    route_id: Option<String>,
    has_sub_routes: Option<bool>,
    operators: Option<Vec<RouteOperator>>,
    #[serde(rename = "AuthorityID")]
    authority_id: Option<String>,
    #[serde(rename = "ProviderID")]
    provider_id: Option<String>,
    sub_routes: Option<Vec<SubRoute>>,
    bus_route_type: Option<i64>,
    route_name: Option<Name>,
    departure_stop_name_zh: Option<String>,
    departure_stop_name_en: Option<String>,
    destination_stop_name_zh: Option<String>,
    destination_stop_name_en: Option<String>,
    ticket_price_description_zh: Option<String>,
    ticket_price_description_en: Option<String>,
    fare_buffer_zone_description_zh: Option<String>,
    fare_buffer_zone_description_en: Option<String>,
    route_map_image_url: Option<String>,
    city: Option<String>,
    city_code: Option<String>,
    update_time: Option<String>,
    #[serde(rename = "VersionID")]
    version_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RouteOperator {
    #[serde(rename = "OperatorID")]
    operator_id: Option<String>,
    operator_name: Option<Name>,
    operator_code: Option<String>,
    operator_no: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubRoute {
    #[serde(rename = "SubRouteUID")]
    sub_route_uid: Option<String>,
    #[serde(rename = "SubRouteID")]
    sub_route_id: Option<String>,
    operator_i_ds: Option<Vec<String>>,
    sub_route_name: Option<Name>,
    headsign: Option<String>,
    headsign_en: Option<String>,
    direction: Option<i64>,
    first_bus_time: Option<String>,
    last_bus_time: Option<String>,
    holiday_first_bus_time: Option<String>,
    holiday_last_bus_time: Option<String>,
}

//BusStops
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusStop {
    #[serde(rename = "StopUID")]
    stop_uid: Option<String>,
    #[serde(rename = "StopID")]
    stop_id: Option<String>,
    #[serde(rename = "AuthorityID")]
    authority_id: Option<String>,
    stop_name: Option<Name>,
    stop_position: Option<Position>,
    stop_address: Option<String>,
    bearing: Option<String>,
    #[serde(rename = "StationID")]
    station_id: Option<String>,
    #[serde(rename = "StationGroupID")]
    station_group_id: Option<String>,
    stop_description: Option<String>,
    city: Option<String>,
    city_code: Option<String>,
    location_city_code: Option<String>,
    update_time: Option<String>,
    #[serde(rename = "VersionID")]
    version_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Position {
    position_lon: Option<f64>,
    position_lat: Option<f64>,
    geo_hash: Option<String>,
}

//Operators
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Operator {
    #[serde(rename = "ProviderID")]
    provider_id: Option<String>,
    #[serde(rename = "OperatorID")]
    operator_id: Option<String>,
    operator_name: Option<Name>,
    operator_phone: Option<String>,
    operator_email: Option<String>,
    operator_url: Option<String>,
    reservation_url: Option<String>,
    reservation_phone: Option<String>,
    operator_code: Option<String>,
    authority_code: Option<String>,
    sub_authority_code: Option<String>,
    operator_no: Option<String>,
    update_time: Option<String>,
}

//BusSchedules
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusSchedule {
    #[serde(rename = "RouteUID")]
    route_uid: Option<String>,
    #[serde(rename = "RouteID")]
    route_id: Option<String>,
    route_name: Option<Name>,
    #[serde(rename = "SubRouteUID")]
    sub_route_uid: Option<String>,
    #[serde(rename = "SubRouteID")]
    sub_route_id: Option<String>,
    sub_route_name: Option<Name>,
    direction: Option<i64>,
    #[serde(rename = "OperatorID")]
    operator_id: Option<String>,
    operator_code: Option<String>,
    operator_no: Option<String>,
    timetables: Option<Vec<BusTimetable>>,
    frequencys: Option<Vec<BusFrequency>>,
    update_time: Option<String>,
    #[serde(rename = "VersionID")]
    version_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusFrequency {
    start_time: Option<String>,
    end_time: Option<String>,
    min_headway_mins: Option<i64>,
    max_headway_mins: Option<i64>,
    service_day: Option<ServiceDay>,
    special_days: Option<Vec<SpecialDay>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceDay {
    service_tag: Option<String>,
    sunday: Option<i64>,
    monday: Option<i64>,
    tuesday: Option<i64>,
    wednesday: Option<i64>,
    thursday: Option<i64>,
    friday: Option<i64>,
    saturday: Option<i64>,
    national_holidays: Option<i64>,
    day_before_holiday: Option<i64>,
    day_after_holiday: Option<i64>,
    typhoon_day: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SpecialDay {
    dates: Option<Vec<String>>,
    date_period: Option<DatePeriod>,
    service_status: Option<i64>,
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DatePeriod {
    start_date: Option<String>,
    end_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Name {
    #[serde(rename = "Zh_tw")]
    zh_tw: Option<String>,
    en: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusTimetable {
    #[serde(rename = "TripID")]
    trip_id: Option<String>,
    is_low_floor: Option<bool>,
    service_day: Option<ServiceDay>,
    special_days: Option<Vec<SpecialDay>>,
    stop_times: Option<Vec<BusStopTime>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusStopTime {
    stop_sequence: Option<i64>,
    #[serde(rename = "StopUID")]
    stop_uid: Option<String>,
    #[serde(rename = "StopID")]
    stop_id: Option<String>,
    stop_name: Option<Name>,
    arrival_time: Option<String>,
    departure_time: Option<String>,
}

//FirstLastTripInfo
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FirstLastTripInfoElement {
    #[serde(rename = "RouteUID")]
    route_uid: Option<String>,
    #[serde(rename = "RouteID")]
    route_id: Option<String>,
    route_name: Option<Name>,
    #[serde(rename = "OperatorID")]
    operator_id: Option<String>,
    operator_no: Option<String>,
    #[serde(rename = "SubRouteUID")]
    sub_route_uid: Option<String>,
    #[serde(rename = "SubRouteID")]
    sub_route_id: Option<String>,
    sub_route_name: Option<Name>,
    direction: Option<i64>,
    first_last_trips: Option<Vec<FirstLastTrip>>,
    update_time: Option<String>,
    #[serde(rename = "VersionID")]
    version_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FirstLastTrip {
    service_day: Option<ServiceDay>,
    first_trip_dep_time: Option<String>,
    last_trip_dep_time: Option<String>,
}

//BusShapes
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Shape {
    #[serde(rename = "RouteUID")]
    route_uid: Option<String>,
    #[serde(rename = "RouteID")]
    route_id: Option<String>,
    route_name: Option<Name>,
    #[serde(rename = "SubRouteUID")]
    sub_route_uid: Option<String>,
    #[serde(rename = "SubRouteID")]
    sub_route_id: Option<String>,
    sub_route_name: Option<Name>,
    direction: Option<i64>,
    geometry: Option<String>,
    encoded_polyline: Option<String>,
    update_time: Option<String>,
    #[serde(rename = "VersionID")]
    version_id: Option<i64>,
}

//BusRouteFare
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusRouteFareElement {
    #[serde(rename = "RouteID")]
    route_id: Option<String>,
    route_name: Option<String>,
    #[serde(rename = "OperatorID")]
    operator_id: Option<String>,
    operator_no: Option<String>,
    #[serde(rename = "SubRouteID")]
    sub_route_id: Option<String>,
    sub_route_name: Option<String>,
    fare_pricing_type: Option<i64>,
    is_free_bus: Option<i64>,
    is_for_all_sub_routes: Option<i64>,
    section_fares: Option<Vec<SectionFare>>,
    stage_fares: Option<Vec<OdFare>>,
    #[serde(rename = "ODFares")]
    od_fares: Option<Vec<OdFare>>,
    update_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OdFare {
    direction: Option<i64>,
    origin_stop: Option<DestinationStop>,
    destination_stop: Option<DestinationStop>,
    fares: Option<Vec<Fare>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DestinationStop {
    #[serde(rename = "StopID")]
    stop_id: Option<String>,
    stop_name: Option<String>,
    sequence: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Fare {
    fare_name: Option<String>,
    ticket_type: Option<i64>,
    fare_class: Option<i64>,
    cabin_class: Option<i64>,
    price: Option<i64>,
    discount_periods: Option<Vec<DiscountPeriod>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SectionFare {
    buffer_zones: Option<Vec<BufferZone>>,
    fares: Option<Vec<Fare>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BufferZone {
    #[serde(rename = "ZoneID")]
    zone_id: Option<String>,
    section_sequence: Option<i64>,
    direction: Option<i64>,
    fare_buffer_zone_origin: Option<FareBufferZone>,
    fare_buffer_zone_destination: Option<FareBufferZone>,
    buffer_zone_description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FareBufferZone {
    #[serde(rename = "StopID")]
    stop_id: Option<String>,
    stop_name: Option<String>,
    sequence: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DiscountPeriod {
    service_day: Option<ServiceDay>,
    start_time: Option<String>,
    end_time: Option<String>,
}

//RailStation
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RailStation {
    #[serde(rename = "StationUID")]
    station_uid: Option<String>,
    #[serde(rename = "StationID")]
    station_id: Option<String>,
    station_code: Option<String>,
    station_name: Option<Name>,
    station_address: Option<String>,
    bike_allow_on_holiday: Option<bool>,
    src_update_time: Option<String>,
    update_time: Option<String>,
    #[serde(rename = "VersionID")]
    version_id: Option<i64>,
    station_position: Option<Position>,
    location_city: Option<String>,
    location_city_code: Option<String>,
    location_town: Option<String>,
    location_town_code: Option<String>,
}

//RailRoutes
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RailRoute {
    #[serde(rename = "RouteID")]
    route_id: Option<String>,
    operator_code: Option<String>,
    route_name: Option<Name>,
    rail_route_type: Option<i64>,
    line_no: Option<String>,
    #[serde(rename = "LineID")]
    line_id: Option<String>,
    direction: Option<i64>,
    #[serde(rename = "StartStationID")]
    start_station_id: Option<String>,
    start_station_name: Option<Name>,
    #[serde(rename = "EndStationID")]
    end_station_id: Option<String>,
    end_station_name: Option<Name>,
    travel_time: Option<i64>,
    route_length: Option<i64>,
    src_update_time: Option<String>,
    update_time: Option<String>,
    #[serde(rename = "VersionID")]
    version_id: Option<i64>,
}

//FirstLastTimetable
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FirstLastTimetable {
    line_no: Option<String>,
    #[serde(rename = "LineID")]
    line_id: Option<String>,
    #[serde(rename = "StationID")]
    station_id: Option<String>,
    station_name: Option<Name>,
    trip_head_sign: Option<String>,
    #[serde(rename = "DestinationStaionID")]
    destination_staion_id: Option<String>,
    destination_station_name: Option<Name>,
    train_type: Option<i64>,
    first_train_time: Option<String>,
    last_train_time: Option<String>,
    service_day: Option<ServiceDay>,
    src_update_time: Option<String>,
    update_time: Option<String>,
    #[serde(rename = "VersionID")]
    version_id: Option<i64>,
}

//RailFrequencies
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RailFrequency {
    line_no: Option<String>,
    #[serde(rename = "LineID")]
    line_id: Option<String>,
    #[serde(rename = "RouteID")]
    route_id: Option<String>,
    train_type: Option<i64>,
    service_day: Option<ServiceDay>,
    operation_time: Option<OperationTime>,
    headways: Option<Vec<Headway>>,
    src_update_time: Option<String>,
    update_time: Option<String>,
    #[serde(rename = "VersionID")]
    version_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Headway {
    peak_flag: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    min_headway_mins: Option<i64>,
    max_headway_mins: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OperationTime {
    start_time: Option<String>,
    end_time: Option<String>,
}

//RailShapes
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RailShape {
    line_no: Option<String>,
    #[serde(rename = "LineID")]
    line_id: Option<String>,
    line_name: Option<Name>,
    update_time: Option<String>,
    geometry: Option<String>,
    encoded_polyline: Option<String>,
}

//MetroFares
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetroFare {
    #[serde(rename = "OriginStationID")]
    origin_station_id: Option<String>,
    origin_station_name: Option<Name>,
    #[serde(rename = "DestinationStationID")]
    destination_station_id: Option<String>,
    destination_station_name: Option<Name>,
    train_type: Option<i64>,
    fares: Option<Vec<MetFare>>,
    travel_time: Option<i64>,
    travel_distance: Option<i64>,
    src_update_time: Option<String>,
    update_time: Option<String>,
    #[serde(rename = "VersionID")]
    version_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetFare {
    ticket_type: Option<i64>,
    fare_class: Option<i64>,
    sale_type: Option<String>,
    citizen_code: Option<String>,
    price: Option<i64>,
}

//THSRGeneralTimetable
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ThsrGeneralTimetable {
    update_time: Option<String>,
    effective_date: Option<String>,
    expiring_date: Option<String>,
    #[serde(rename = "VersionID")]
    version_id: Option<i64>,
    general_timetable: Option<GeneralTimetable>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GeneralTimetable {
    general_train_info: Option<GeneralTrainInfo>,
    stop_times: Option<Vec<StopTime>>,
    service_day: Option<ServiceDay>,
    src_update_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GeneralTrainInfo {
    train_no: Option<String>,
    direction: Option<i64>,
    #[serde(rename = "StartingStationID")]
    starting_station_id: Option<String>,
    starting_station_name: Option<Name>,
    #[serde(rename = "EndingStationID")]
    ending_station_id: Option<String>,
    ending_station_name: Option<Name>,
    note: Option<Name>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StopTime {
    stop_sequence: Option<i64>,
    #[serde(rename = "StationID")]
    station_id: Option<String>,
    station_name: Option<Name>,
    arrival_time: Option<String>,
    departure_time: Option<String>,
}

//THSRFare
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RailFare {
    #[serde(rename = "OriginStationID")]
    origin_station_id: Option<String>,
    origin_station_name: Option<Name>,
    #[serde(rename = "DestinationStationID")]
    destination_station_id: Option<String>,
    destination_station_name: Option<Name>,
    direction: Option<i64>,
    fares: Option<Vec<Fare>>,
    train_type: Option<i64>,
    travel_distance: Option<i64>,
    src_update_time: Option<String>,
    update_time: Option<String>,
    #[serde(rename = "VersionID")]
    version_id: Option<i64>,
}

//V3RailOperators
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3RailOperator {
    operator_code: Option<String>,
    operator_name: Option<Name>,
    operator_phone: Option<String>,
    operator_email: Option<String>,
    #[serde(rename = "OperatorURL")]
    operator_url: Option<String>,
    #[serde(rename = "FareURL")]
    fare_url: Option<String>,
    #[serde(rename = "ReservationURL")]
    reservation_url: Option<String>,
    reservation_phone: Option<String>,
    #[serde(rename = "OperatorLogoURL")]
    operator_logo_url: Option<String>,
}

//V3RailStations
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3RailStation {
    #[serde(rename = "StationUID")]
    station_uid: Option<String>,
    #[serde(rename = "StationID")]
    station_id: Option<String>,
    reservation_code: Option<String>,
    station_name: Option<Name>,
    station_position: Option<Position>,
    station_address: Option<String>,
    station_phone: Option<String>,
    station_class: Option<String>,
    #[serde(rename = "StationURL")]
    station_url: Option<String>,
}

//V3RailRoutes
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3RailRoute {
    line_no: Option<String>,
    #[serde(rename = "LineID")]
    line_id: Option<String>,
    #[serde(rename = "RouteID")]
    route_id: Option<String>,
    operator_code: Option<String>,
    route_name: Option<Name>,
    route_type: Option<i64>,
    #[serde(rename = "StartStationID")]
    start_station_id: Option<String>,
    start_station_name: Option<Name>,
    #[serde(rename = "EndStationID")]
    end_station_id: Option<String>,
    end_station_name: Option<Name>,
    #[serde(rename = "RouteURL")]
    route_url: Option<String>,
    route_distance: Option<i64>,
}

//V3GeneralTrainTimetable
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3TrainTimetable {
    train_info: Option<V3TrainInfo>,
    stop_times: Option<Vec<StopTime>>,
    service_day: Option<ServiceDay>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3TrainInfo {
    train_no: Option<String>,
    #[serde(rename = "RouteID")]
    route_id: Option<String>,
    direction: Option<i64>,
    #[serde(rename = "TrainTypeID")]
    train_type_id: Option<String>,
    train_type_code: Option<String>,
    train_type_name: Option<Name>,
    trip_head_sign: Option<String>,
    #[serde(rename = "StartingStationID")]
    starting_station_id: Option<String>,
    starting_station_name: Option<Name>,
    #[serde(rename = "EndingStationID")]
    ending_station_id: Option<String>,
    ending_station_name: Option<Name>,
    #[serde(rename = "OverNightStationID")]
    over_night_station_id: Option<String>,
    trip_line: Option<i64>,
    wheel_chair_flag: Option<i64>,
    package_service_flag: Option<i64>,
    dining_flag: Option<i64>,
    breast_feed_flag: Option<i64>,
    bike_flag: Option<i64>,
    car_flag: Option<i64>,
    daily_flag: Option<i64>,
    extra_train_flag: Option<i64>,
    note: Option<String>,
}

//V3RailShapes
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3RailShape {
    line_no: Option<String>,
    #[serde(rename = "LineID")]
    line_id: Option<String>,
    line_name: Option<Name>,
    geometry: Option<String>,
    update_time: Option<String>,
}

//BusRtFrequency
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusRtFrequencyElement {
    plate_numb: Option<String>,
    #[serde(rename = "OperatorID")]
    operator_id: Option<String>,
    operator_no: Option<String>,
    #[serde(rename = "RouteUID")]
    route_uid: Option<String>,
    #[serde(rename = "RouteID")]
    route_id: Option<String>,
    route_name: Option<Name>,
    #[serde(rename = "SubRouteUID")]
    sub_route_uid: Option<String>,
    #[serde(rename = "SubRouteID")]
    sub_route_id: Option<String>,
    sub_route_name: Option<Name>,
    direction: Option<i64>,
    bus_position: Option<Position>,
    speed: Option<i64>,
    azimuth: Option<i64>,
    duty_status: Option<i64>,
    bus_status: Option<i64>,
    message_type: Option<i64>,
    #[serde(rename = "GPSTime")]
    gps_time: Option<String>,
    trans_time: Option<String>,
    src_rec_time: Option<String>,
    src_trans_time: Option<String>,
    src_update_time: Option<String>,
    update_time: Option<String>,
}

//BusRtStops
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusRtStop {
    plate_numb: Option<String>,
    #[serde(rename = "OperatorID")]
    operator_id: Option<String>,
    operator_no: Option<String>,
    #[serde(rename = "RouteUID")]
    route_uid: Option<String>,
    #[serde(rename = "RouteID")]
    route_id: Option<String>,
    route_name: Option<Name>,
    #[serde(rename = "SubRouteUID")]
    sub_route_uid: Option<String>,
    #[serde(rename = "SubRouteID")]
    sub_route_id: Option<String>,
    sub_route_name: Option<Name>,
    direction: Option<i64>,
    #[serde(rename = "StopUID")]
    stop_uid: Option<String>,
    #[serde(rename = "StopID")]
    stop_id: Option<String>,
    stop_name: Option<Name>,
    stop_sequence: Option<i64>,
    message_type: Option<i64>,
    duty_status: Option<i64>,
    bus_status: Option<i64>,
    a2_event_type: Option<i64>,
    #[serde(rename = "GPSTime")]
    gps_time: Option<String>,
    trip_start_time_type: Option<i64>,
    trip_start_time: Option<String>,
    trans_time: Option<String>,
    src_rec_time: Option<String>,
    src_trans_time: Option<String>,
    src_update_time: Option<String>,
    update_time: Option<String>,
}

//BusEta
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusEtaElement {
    plate_numb: Option<String>,
    #[serde(rename = "StopUID")]
    stop_uid: Option<String>,
    #[serde(rename = "StopID")]
    stop_id: Option<String>,
    stop_name: Option<Name>,
    #[serde(rename = "RouteUID")]
    route_uid: Option<String>,
    #[serde(rename = "RouteID")]
    route_id: Option<String>,
    route_name: Option<Name>,
    #[serde(rename = "SubRouteUID")]
    sub_route_uid: Option<String>,
    #[serde(rename = "SubRouteID")]
    sub_route_id: Option<String>,
    sub_route_name: Option<Name>,
    direction: Option<i64>,
    estimate_time: Option<i64>,
    stop_count_down: Option<i64>,
    current_stop: Option<String>,
    destination_stop: Option<String>,
    stop_sequence: Option<i64>,
    stop_status: Option<i64>,
    message_type: Option<i64>,
    next_bus_time: Option<String>,
    is_last_bus: Option<bool>,
    estimates: Option<Vec<Estimate>>,
    data_time: Option<String>,
    trans_time: Option<String>,
    src_rec_time: Option<String>,
    src_trans_time: Option<String>,
    src_update_time: Option<String>,
    update_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Estimate {
    plate_numb: Option<String>,
    estimate_time: Option<i64>,
    is_last_bus: Option<bool>,
    vehicle_stop_status: Option<i64>,
}

//BusAlerts
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusAlert {
    #[serde(rename = "AlertID")]
    alert_id: Option<String>,
    title: Option<String>,
    description: Option<String>,
    department: Option<String>,
    status: Option<i64>,
    cause: Option<i64>,
    effect: Option<i64>,
    scope: Option<Scope>,
    #[serde(rename = "AlertURL")]
    alert_url: Option<String>,
    publish_time: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    src_update_time: Option<String>,
    update_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Scope {
    operators: Option<Vec<RouteOperator>>,
    stops: Option<Vec<Stop>>,
    stations: Option<Vec<Station>>,
    routes: Option<Vec<Route>>,
    sub_routes: Option<Vec<SubRouteAlert>>,
    trips: Option<Vec<Trip>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Route {
    #[serde(rename = "RouteID")]
    route_id: Option<String>,
    route_name: Option<Name>,
    direction: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Station {
    #[serde(rename = "StationID")]
    station_id: Option<String>,
    station_name: Option<Name>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Stop {
    #[serde(rename = "StopID")]
    stop_id: Option<String>,
    stop_name: Option<Name>,
    #[serde(rename = "StationID")]
    station_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubRouteAlert {
    #[serde(rename = "SubRouteID")]
    sub_route_id: Option<String>,
    sub_route_name: Option<Name>,
    direction: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Trip {
    #[serde(rename = "TripID")]
    trip_id: Option<String>,
    #[serde(rename = "RouteID")]
    route_id: Option<String>,
    #[serde(rename = "SubRouteID")]
    sub_route_id: Option<String>,
    direction: Option<i64>,
    trip_dep_time: Option<String>,
}

//TrailLiveBoard
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TrainLiveBoardElement {
    train_no: Option<String>,
    #[serde(rename = "TrainTypeID")]
    train_type_id: Option<String>,
    train_type_code: Option<String>,
    train_type_name: Option<Name>,
    #[serde(rename = "StationID")]
    station_id: Option<String>,
    station_name: Option<Name>,
    train_station_status: Option<i64>,
    delay_time: Option<i64>,
    update_time: Option<String>,
}

//StationLiveBoardElement
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StationLiveBoardElement {
    #[serde(rename = "StationID")]
    station_id: Option<String>,
    station_name: Option<Name>,
    train_no: Option<String>,
    direction: Option<i64>,
    #[serde(rename = "TrainTypeID")]
    train_type_id: Option<String>,
    train_type_code: Option<String>,
    train_type_name: Option<Name>,
    #[serde(rename = "EndingStationID")]
    ending_station_id: Option<String>,
    ending_station_name: Option<Name>,
    trip_line: Option<i64>,
    platform: Option<String>,
    schedule_arrival_time: Option<String>,
    schedule_departure_time: Option<String>,
    delay_time: Option<i64>,
    running_status: Option<i64>,
    update_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RailAlert {
    #[serde(rename = "AlertID")]
    alert_id: Option<String>,
    title: Option<String>,
    description: Option<String>,
    status: Option<i64>,
    scope: Option<RailScope>,
    direction: Option<i64>,
    level: Option<i64>,
    effect: Option<String>,
    reason: Option<String>,
    #[serde(rename = "AlertURL")]
    alert_url: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    publish_time: Option<String>,
    update_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RailScope {
    network_list: Option<NetworkList>,
    network: Option<NetworkList>,
    stations: Option<Vec<AlertStation>>,
    lines: Option<Vec<Line>>,
    routes: Option<Vec<AlertRoute>>,
    trains: Option<Vec<Train>>,
    line_sections: Option<Vec<LineSection>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LineSection {
    #[serde(rename = "LineID")]
    line_id: Option<String>,
    #[serde(rename = "StartingStationID")]
    starting_station_id: Option<String>,
    starting_station_name: Option<String>,
    #[serde(rename = "EndingStationID")]
    ending_station_id: Option<String>,
    ending_station_name: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Line {
    #[serde(rename = "LineID")]
    line_id: Option<String>,
    line_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkList {
    #[serde(rename = "NetworkID")]
    network_id: Option<String>,
    network_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AlertRoute {
    #[serde(rename = "RouteID")]
    route_id: Option<String>,
    route_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AlertStation {
    #[serde(rename = "StationID")]
    station_id: Option<String>,
    station_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Train {
    train_no: Option<String>,
}

//MetroLiveBoard
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetroLiveBoardElement {
    #[serde(rename = "LineNO")]
    line_no: Option<String>,
    #[serde(rename = "LineID")]
    line_id: Option<String>,
    line_name: Option<Name>,
    #[serde(rename = "StationID")]
    station_id: Option<String>,
    station_name: Option<Name>,
    trip_head_sign: Option<String>,
    #[serde(rename = "DestinationStaionID")]
    destination_staion_id: Option<String>,
    #[serde(rename = "DestinationStationID")]
    destination_station_id: Option<String>,
    destination_station_name: Option<Name>,
    service_status: Option<i64>,
    estimate_time: Option<i64>,
    src_update_time: Option<String>,
    update_time: Option<String>,
}

//MetroStationTimeTable
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetroStationTimeTableElement {
    #[serde(rename = "RouteID")]
    route_id: Option<String>,
    #[serde(rename = "LineID")]
    line_id: Option<String>,
    #[serde(rename = "StationID")]
    station_id: Option<String>,
    station_name: Option<Name>,
    direction: Option<i64>,
    #[serde(rename = "DestinationStaionID")]
    destination_staion_id: Option<String>,
    destination_station_name: Option<Name>,
    timetables: Option<Vec<Timetable>>,
    service_day: Option<MetroServiceDay>,
    special_days: Option<Vec<MetroSpecialDay>>,
    src_update_time: Option<String>,
    update_time: Option<String>,
    #[serde(rename = "VersionID")]
    version_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetroServiceDay {
    service_tag: Option<String>,
    monday: Option<bool>,
    tuesday: Option<bool>,
    wednesday: Option<bool>,
    thursday: Option<bool>,
    friday: Option<bool>,
    saturday: Option<bool>,
    sunday: Option<bool>,
    national_holidays: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetroSpecialDay {
    sater_date: Option<String>,
    end_date: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Timetable {
    sequence: Option<i64>,
    train_no: Option<String>,
    arrival_time: Option<String>,
    departure_time: Option<String>,
    train_type: Option<i64>,
    #[serde(rename = "StoppingPatternID")]
    stopping_pattern_id: Option<String>,
}

//THSRAlertInfo
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ThsrAlertInfoElement {
    #[serde(rename = "AlertID")]
    alert_id: Option<String>,
    title: Option<String>,
    description: Option<String>,
    status: Option<String>,
    scope: Option<ThsrScope>,
    direction: Option<i64>,
    level: Option<i64>,
    effect: Option<String>,
    reason: Option<String>,
    #[serde(rename = "AlertURL")]
    alert_url: Option<String>,
    occured_time: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    publish_time: Option<String>,
    src_update_time: Option<String>,
    update_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ThsrScope {
    line_sections: Option<Vec<LineSection>>,
}
