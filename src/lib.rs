use serde::{Deserialize, Serialize};

//just a lot of struct to deserialize (decerealize? un-cornflæke?) everything
pub type BusRoutes = Vec<BusRoute>; //top level

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusRoute {
    #[serde(rename = "RouteUID")]
    route_uid: String,
    #[serde(rename = "RouteID")]
    route_id: String,
    has_sub_routes: bool,
    operators: Vec<RouteOperator>,
    #[serde(rename = "AuthorityID")]
    authority_id: String,
    #[serde(rename = "ProviderID")]
    provider_id: String,
    sub_routes: Vec<SubRoute>,
    bus_route_type: i64,
    route_name: Name,
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
pub struct RouteOperator {
    #[serde(rename = "OperatorID")]
    operator_id: String,
    operator_name: Name,
    operator_code: Option<String>,
    operator_no: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubRoute {
    #[serde(rename = "SubRouteUID")]
    sub_route_uid: String,
    #[serde(rename = "SubRouteID")]
    sub_route_id: String,
    operator_i_ds: Vec<String>,
    sub_route_name: Name,
    headsign: String,
    headsign_en: String,
    direction: i64,
    first_bus_time: String,
    last_bus_time: String,
    holiday_first_bus_time: String,
    holiday_last_bus_time: String,
}

pub type BusStops = Vec<BusStop>; //top level

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusStop {
    #[serde(rename = "StopUID")]
    stop_uid: String,
    #[serde(rename = "StopID")]
    stop_id: String,
    #[serde(rename = "AuthorityID")]
    authority_id: String,
    stop_name: Name,
    stop_position: Position,
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
pub struct Position {
    position_lon: f64,
    position_lat: f64,
    geo_hash: Option<String>,
}

pub type Operators = Vec<Operator>; //top level

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Operator {
    #[serde(rename = "ProviderID")]
    provider_id: String,
    #[serde(rename = "OperatorID")]
    operator_id: String,
    operator_name: Name,
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

pub type BusSchedules = Vec<BusSchedule>; //top level

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusSchedule {
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
    timetables: Vec<BusTimetable>,
    frequencys: Vec<BusFrequency>,
    update_time: String,
    #[serde(rename = "VersionID")]
    version_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusFrequency {
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
    service_tag: Option<String>,
    sunday: Option<i64>,
    monday: i64,
    tuesday: i64,
    wednesday: i64,
    thursday: i64,
    friday: i64,
    saturday: i64,
    national_holidays: Option<i64>,
    day_before_holiday: Option<i64>,
    day_after_holiday: Option<i64>,
    typhoon_day: Option<i64>,
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
    zh_tw: Option<String>,
    en: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusTimetable {
    #[serde(rename = "TripID")]
    trip_id: String,
    is_low_floor: bool,
    service_day: ServiceDay,
    special_days: Vec<SpecialDay>,
    stop_times: Vec<BusStopTime>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusStopTime {
    stop_sequence: i64,
    #[serde(rename = "StopUID")]
    stop_uid: String,
    #[serde(rename = "StopID")]
    stop_id: String,
    stop_name: Name,
    arrival_time: String,
    departure_time: String,
}

pub type FirstLastTripInfo = Vec<FirstLastTripInfoElement>; //top level

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FirstLastTripInfoElement {
    #[serde(rename = "RouteUID")]
    route_uid: String,
    #[serde(rename = "RouteID")]
    route_id: String,
    route_name: Name,
    #[serde(rename = "OperatorID")]
    operator_id: String,
    operator_no: String,
    #[serde(rename = "SubRouteUID")]
    sub_route_uid: String,
    #[serde(rename = "SubRouteID")]
    sub_route_id: String,
    sub_route_name: Name,
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

pub type Shapes = Vec<Shape>; //top level

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Shape {
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
    geometry: String,
    encoded_polyline: String,
    update_time: String,
    #[serde(rename = "VersionID")]
    version_id: i64,
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
    stage_fares: Option<Vec<OdFare>>,
    #[serde(rename = "ODFares")]
    od_fares: Option<Vec<OdFare>>,
    update_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OdFare {
    direction: i64,
    origin_stop: DestinationStop,
    destination_stop: DestinationStop,
    fares: Vec<Fare>,
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
pub struct Fare {
    fare_name: Option<String>,
    ticket_type: i64,
    fare_class: i64,
    cabin_class: Option<i64>,
    price: i64,
    discount_periods: Option<Vec<DiscountPeriod>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SectionFare {
    buffer_zones: Vec<BufferZone>,
    fares: Vec<Fare>,
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
    stop_id: String,
    stop_name: String,
    sequence: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DiscountPeriod {
    service_day: ServiceDay,
    start_time: String,
    end_time: String,
}

pub type RailStations = Vec<RailStation>; //top level

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RailStation {
    #[serde(rename = "StationUID")]
    station_uid: String,
    #[serde(rename = "StationID")]
    station_id: String,
    station_code: Option<String>,
    station_name: Name,
    station_address: String,
    bike_allow_on_holiday: bool,
    src_update_time: String,
    update_time: String,
    #[serde(rename = "VersionID")]
    version_id: i64,
    station_position: Position,
    location_city: String,
    location_city_code: String,
    location_town: String,
    location_town_code: String,
}

pub type RailRoutes = Vec<RailRoute>; //top level

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RailRoute {
    #[serde(rename = "RouteID")]
    route_id: String,
    operator_code: String,
    route_name: Name,
    rail_route_type: i64,
    line_no: String,
    #[serde(rename = "LineID")]
    line_id: String,
    direction: i64,
    #[serde(rename = "StartStationID")]
    start_station_id: String,
    start_station_name: Name,
    #[serde(rename = "EndStationID")]
    end_station_id: String,
    end_station_name: Name,
    travel_time: i64,
    route_length: i64,
    src_update_time: String,
    update_time: String,
    #[serde(rename = "VersionID")]
    version_id: i64,
}

pub type FirstLastTimetables = Vec<FirstLastTimetable>; //top level

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FirstLastTimetable {
    line_no: String,
    #[serde(rename = "LineID")]
    line_id: String,
    #[serde(rename = "StationID")]
    station_id: String,
    station_name: Name,
    trip_head_sign: String,
    #[serde(rename = "DestinationStaionID")]
    destination_staion_id: String,
    destination_station_name: Name,
    train_type: i64,
    first_train_time: String,
    last_train_time: String,
    service_day: ServiceDay,
    src_update_time: String,
    update_time: String,
    #[serde(rename = "VersionID")]
    version_id: i64,
}

pub type RailFrequencies = Vec<RailFrequency>; //top level

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RailFrequency {
    line_no: String,
    #[serde(rename = "LineID")]
    line_id: String,
    #[serde(rename = "RouteID")]
    route_id: String,
    train_type: i64,
    service_day: ServiceDay,
    operation_time: OperationTime,
    headways: Vec<Headway>,
    src_update_time: String,
    update_time: String,
    #[serde(rename = "VersionID")]
    version_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Headway {
    peak_flag: String,
    start_time: String,
    end_time: String,
    min_headway_mins: i64,
    max_headway_mins: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OperationTime {
    start_time: String,
    end_time: String,
}

pub type RailShapes = Vec<RailShape>; //top level

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RailShape {
    line_no: String,
    #[serde(rename = "LineID")]
    line_id: String,
    line_name: Name,
    update_time: String,
    geometry: String,
    encoded_polyline: Option<String>,
}

pub type MetroFares = Vec<MetroFare>; //top level

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetroFare {
    #[serde(rename = "OriginStationID")]
    origin_station_id: String,
    origin_station_name: Name,
    #[serde(rename = "DestinationStationID")]
    destination_station_id: String,
    destination_station_name: Name,
    train_type: i64,
    fares: Vec<MetFare>,
    travel_time: i64,
    travel_distance: i64,
    src_update_time: String,
    update_time: String,
    #[serde(rename = "VersionID")]
    version_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetFare {
    ticket_type: i64,
    fare_class: i64,
    sale_type: String,
    citizen_code: String,
    price: i64,
}

pub type ThsrGeneralTimetables = Vec<ThsrGeneralTimetable>; //top level

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ThsrGeneralTimetable {
    update_time: String,
    effective_date: String,
    expiring_date: String,
    #[serde(rename = "VersionID")]
    version_id: i64,
    general_timetable: GeneralTimetable,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GeneralTimetable {
    general_train_info: GeneralTrainInfo,
    stop_times: Vec<StopTime>,
    service_day: ServiceDay,
    src_update_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GeneralTrainInfo {
    train_no: String,
    direction: i64,
    #[serde(rename = "StartingStationID")]
    starting_station_id: String,
    starting_station_name: Name,
    #[serde(rename = "EndingStationID")]
    ending_station_id: String,
    ending_station_name: Name,
    note: Name,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StopTime {
    stop_sequence: i64,
    #[serde(rename = "StationID")]
    station_id: String,
    station_name: Name,
    arrival_time: Option<String>,
    departure_time: Option<String>,
}

pub type ThsrFares = Vec<RailFare>; //top level

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RailFare {
    #[serde(rename = "OriginStationID")]
    origin_station_id: String,
    origin_station_name: Name,
    #[serde(rename = "DestinationStationID")]
    destination_station_id: String,
    destination_station_name: Name,
    direction: i64,
    fares: Vec<Fare>,
    train_type: Option<i64>,
    travel_distance: Option<i64>,
    src_update_time: Option<String>,
    update_time: Option<String>,
    #[serde(rename = "VersionID")]
    version_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3Operators {
    //top level
    update_time: String,
    update_interval: i64,
    src_update_time: String,
    src_update_interval: i64,
    authority_code: String,
    operators: Vec<V3Operator>,
    count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3Operator {
    operator_code: String,
    operator_name: Name,
    operator_phone: String,
    operator_email: String,
    #[serde(rename = "OperatorURL")]
    operator_url: String,
    #[serde(rename = "FareURL")]
    fare_url: String,
    #[serde(rename = "ReservationURL")]
    reservation_url: String,
    reservation_phone: String,
    #[serde(rename = "OperatorLogoURL")]
    operator_logo_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3Stations {  //also works for AFR stations
    //top level
    update_time: String,
    update_interval: i64,
    src_update_time: String,
    src_update_interval: i64,
    authority_code: String,
    stations: Vec<V3Station>,
    count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3Station { 
    #[serde(rename = "StationUID")]
    station_uid: String,
    #[serde(rename = "StationID")]
    station_id: String,
    reservation_code: String,
    station_name: Name,
    station_position: Position,
    station_address: String,
    station_phone: String,
    station_class: String,
    #[serde(rename = "StationURL")]
    station_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3GeneralTrainTimetables {
    //top level
    update_time: String,
    update_interval: i64,
    src_update_time: String,
    src_update_interval: i64,
    authority_code: String,
    effective_date: String,
    expire_date: String,
    src_version: String,
    timetable_name: String,
    validity_desciption: String,
    train_timetables: Vec<V3TrainTimetable>,
    count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3TrainTimetable {
    train_info: V3TrainInfo,
    stop_times: Vec<StopTime>,
    service_day: ServiceDay
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3TrainInfo {
    train_no: String,
    #[serde(rename = "RouteID")]
    route_id: String,
    direction: i64,
    #[serde(rename = "TrainTypeID")]
    train_type_id: String,
    train_type_code: String,
    train_type_name: Name,
    trip_head_sign: String,
    #[serde(rename = "StartingStationID")]
    starting_station_id: String,
    starting_station_name: Name,
    #[serde(rename = "EndingStationID")]
    ending_station_id: String,
    ending_station_name: Name,
    #[serde(rename = "OverNightStationID")]
    over_night_station_id: String,
    trip_line: i64,
    wheel_chair_flag: i64,
    package_service_flag: i64,
    dining_flag: i64,
    breast_feed_flag: i64,
    bike_flag: i64,
    car_flag: i64,
    daily_flag: i64,
    extra_train_flag: i64,
    note: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3Shapes {
    //top level
    update_time: String,
    update_interval: i64,
    src_update_time: String,
    src_update_interval: i64,
    authority_code: String,
    shapes: Vec<Shape>,
    count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3OdFares {
    //top level
    update_time: String,
    update_interval: i64,
    src_update_time: String,
    src_update_interval: i64,
    authority_code: String,
    effective_date: String,
    expire_date: String,
    src_version: String,
    #[serde(rename = "ODFares")]
    od_fares: Vec<RailFare>,
    count: i64,
}

pub type BusRtFrequency = Vec<BusRtFrequencyElement>; //top level

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusRtFrequencyElement {
    plate_numb: String,
    #[serde(rename = "OperatorID")]
    operator_id: String,
    operator_no: String,
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
    bus_position: Position,
    speed: i64,
    azimuth: i64,
    duty_status: i64,
    bus_status: i64,
    message_type: i64,
    #[serde(rename = "GPSTime")]
    gps_time: String,
    trans_time: String,
    src_rec_time: String,
    src_trans_time: String,
    src_update_time: String,
    update_time: String,
}

pub type BusRtStops = Vec<BusRtStop>; //top level

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusRtStop {
    plate_numb: String,
    #[serde(rename = "OperatorID")]
    operator_id: String,
    operator_no: String,
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
    #[serde(rename = "StopUID")]
    stop_uid: String,
    #[serde(rename = "StopID")]
    stop_id: String,
    stop_name: Name,
    stop_sequence: i64,
    message_type: i64,
    duty_status: i64,
    bus_status: i64,
    a2_event_type: i64,
    #[serde(rename = "GPSTime")]
    gps_time: String,
    trip_start_time_type: i64,
    trip_start_time: String,
    trans_time: String,
    src_rec_time: String,
    src_trans_time: String,
    src_update_time: String,
    update_time: String,
}

pub type BusEta = Vec<BusEtaElement>; //top level

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusEtaElement {
    plate_numb: String,
    #[serde(rename = "StopUID")]
    stop_uid: String,
    #[serde(rename = "StopID")]
    stop_id: String,
    stop_name: Name,
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
    estimate_time: i64,
    stop_count_down: i64,
    current_stop: String,
    destination_stop: String,
    stop_sequence: i64,
    stop_status: i64,
    message_type: i64,
    next_bus_time: String,
    is_last_bus: bool,
    estimates: Vec<Estimate>,
    data_time: String,
    trans_time: String,
    src_rec_time: String,
    src_trans_time: String,
    src_update_time: String,
    update_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Estimate {
    plate_numb: String,
    estimate_time: i64,
    is_last_bus: bool,
    vehicle_stop_status: i64,
}

pub type BusAlerts = Vec<BusAlert>; //top level
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusAlert {
    #[serde(rename = "AlertID")]
    alert_id: String,
    title: String,
    description: String,
    department: String,
    status: i64,
    cause: i64,
    effect: i64,
    scope: Scope,
    #[serde(rename = "AlertURL")]
    alert_url: String,
    publish_time: String,
    start_time: String,
    end_time: String,
    src_update_time: String,
    update_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Scope {
    operators: Vec<RouteOperator>,
    stops: Vec<Stop>,
    stations: Vec<Station>,
    routes: Vec<Route>,
    sub_routes: Vec<SubRouteAlert>,
    trips: Vec<Trip>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Route {
    #[serde(rename = "RouteID")]
    route_id: String,
    route_name: Name,
    direction: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Station {
    #[serde(rename = "StationID")]
    station_id: String,
    station_name: Name,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Stop {
    #[serde(rename = "StopID")]
    stop_id: String,
    stop_name: Name,
    #[serde(rename = "StationID")]
    station_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubRouteAlert {
    #[serde(rename = "SubRouteID")]
    sub_route_id: String,
    sub_route_name: Name,
    direction: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Trip {
    #[serde(rename = "TripID")]
    trip_id: String,
    #[serde(rename = "RouteID")]
    route_id: String,
    #[serde(rename = "SubRouteID")]
    sub_route_id: String,
    direction: i64,
    trip_dep_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TrainLiveBoard {
    //top level
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
    train_type_name: Name,
    #[serde(rename = "StationID")]
    station_id: String,
    station_name: Name,
    train_station_status: i64,
    delay_time: i64,
    update_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StationLiveBoard {
    //top level
    update_time: String,
    update_interval: i64,
    src_update_time: String,
    src_update_interval: i64,
    authority_code: String,
    station_live_boards: Vec<StationLiveBoardElement>,
    count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StationLiveBoardElement {
    #[serde(rename = "StationID")]
    station_id: String,
    station_name: Name,
    train_no: String,
    direction: i64,
    #[serde(rename = "TrainTypeID")]
    train_type_id: String,
    train_type_code: String,
    train_type_name: Name,
    #[serde(rename = "EndingStationID")]
    ending_station_id: String,
    ending_station_name: Name,
    trip_line: i64,
    platform: String,
    schedule_arrival_time: String,
    schedule_departure_time: String,
    delay_time: i64,
    running_status: i64,
    update_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TraAlerts {
    //top level
    update_time: String,
    update_interval: i64,
    src_update_time: String,
    src_update_interval: i64,
    authority_code: String,
    alerts: Vec<TraAlert>,
    count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TraAlert {
    #[serde(rename = "AlertID")]
    alert_id: String,
    title: String,
    description: String,
    status: i64,
    scope: TraScope,
    direction: i64,
    level: i64,
    effect: String,
    reason: String,
    #[serde(rename = "AlertURL")]
    alert_url: String,
    start_time: String,
    end_time: String,
    publish_time: String,
    update_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TraScope {
    network_list: NetworkList,
    stations: Vec<ScopeStation>,
    lines: Vec<Line>,
    routes: Vec<ScopeRoute>,
    trains: Vec<Train>,
    line_sections: Vec<LineSection>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LineSection {
    #[serde(rename = "LineID")]
    line_id: String,
    #[serde(rename = "StartingStationID")]
    starting_station_id: String,
    starting_station_name: String,
    #[serde(rename = "EndingStationID")]
    ending_station_id: String,
    ending_station_name: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Line {
    #[serde(rename = "LineID")]
    line_id: String,
    line_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkList {
    #[serde(rename = "NetworkID")]
    network_id: String,
    network_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScopeRoute {
    #[serde(rename = "RouteID")]
    route_id: String,
    route_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScopeStation {
    #[serde(rename = "StationID")]
    station_id: String,
    station_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Train {
    train_no: String,
}

pub type MetroLiveBoard = Vec<MetroLiveBoardElement>; //top level

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetroLiveBoardElement {
    #[serde(rename = "LineNO")]
    line_no: String,
    #[serde(rename = "LineID")]
    line_id: String,
    line_name: Name,
    #[serde(rename = "StationID")]
    station_id: String,
    station_name: Name,
    trip_head_sign: String,
    #[serde(rename = "DestinationStaionID")]
    destination_staion_id: String,
    #[serde(rename = "DestinationStationID")]
    destination_station_id: String,
    destination_station_name: Name,
    service_status: i64,
    estimate_time: i64,
    src_update_time: String,
    update_time: String,
}

pub type MetroStationTimeTable = Vec<MetroStationTimeTableElement>; //top level

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetroStationTimeTableElement {
    #[serde(rename = "RouteID")]
    route_id: String,
    #[serde(rename = "LineID")]
    line_id: String,
    #[serde(rename = "StationID")]
    station_id: String,
    station_name: StationName,
    direction: i64,
    #[serde(rename = "DestinationStaionID")]
    destination_staion_id: String,
    destination_station_name: StationName,
    timetables: Vec<Timetable>,
    service_day: MetroServiceDay,
    special_days: Vec<MetroSpecialDay>,
    src_update_time: String,
    update_time: String,
    #[serde(rename = "VersionID")]
    version_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StationName {
    #[serde(rename = "Zh_tw")]
    zh_tw: String,
    en: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetroServiceDay {
    service_tag: String,
    monday: bool,
    tuesday: bool,
    wednesday: bool,
    thursday: bool,
    friday: bool,
    saturday: bool,
    sunday: bool,
    national_holidays: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetroSpecialDay {
    sater_date: String,
    end_date: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Timetable {
    sequence: i64,
    train_no: String,
    arrival_time: String,
    departure_time: String,
    train_type: i64,
    #[serde(rename = "StoppingPatternID")]
    stopping_pattern_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetroAlerts {
    //top level
    update_time: String,
    update_interval: i64,
    src_update_time: String,
    src_update_interval: i64,
    authority_code: String,
    alerts: Vec<Alert>,
    count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Alert {
    #[serde(rename = "AlertID")]
    alert_id: String,
    title: String,
    description: String,
    status: i64,
    scope: MetroScope,
    direction: i64,
    level: i64,
    effect: String,
    reason: String,
    #[serde(rename = "AlertURL")]
    alert_url: String,
    start_time: String,
    end_time: String,
    publish_time: String,
    update_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetroScope {
    network: Network,
    stations: Vec<ScopeStation>,
    lines: Vec<Line>,
    routes: Vec<ScopeRoute>,
    trains: Vec<Train>,
    line_sections: Vec<LineSection>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Network {
    #[serde(rename = "NetworkID")]
    network_id: String,
}

pub type ThsrAlertInfo = Vec<ThsrAlertInfoElement>; //top level

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ThsrAlertInfoElement {
    #[serde(rename = "AlertID")]
    alert_id: String,
    title: String,
    description: String,
    status: String,
    scope: ThsrScope,
    direction: i64,
    level: i64,
    effect: String,
    reason: String,
    #[serde(rename = "AlertURL")]
    alert_url: String,
    occured_time: String,
    start_time: String,
    end_time: String,
    publish_time: String,
    src_update_time: String,
    update_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ThsrScope {
    line_sections: Vec<LineSection>,
}
