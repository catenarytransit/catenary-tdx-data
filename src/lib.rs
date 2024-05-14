use serde::{Deserialize, Serialize};

//just a lot of struct to deserialize (decerealize? un-cornflæke?) everything
/*
done list: TrainLiveBoard, all static data (hopefully)

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
    operators: Vec<RouteOperator>,
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
pub struct RouteOperator {
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

pub type BusStops = Vec<BusStop>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusStop {
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

pub type Operators = Vec<Operator>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Operator {
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

pub type BusSchedules = Vec<BusSchedule>;

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
    service_day: BusServiceDay,
    first_trip_dep_time: String,
    last_trip_dep_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusServiceDay {
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

pub type Shapes = Vec<Shape>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Shape {
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

pub type RouteFares = Vec<RouteFare>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RouteFare {
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

pub type RailStations = Vec<RailStation>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RailStation {
    #[serde(rename = "StationUID")]
    station_uid: String,
    #[serde(rename = "StationID")]
    station_id: String,
    station_code: Option<String>,
    station_name: StationName,
    station_address: String,
    bike_allow_on_holiday: bool,
    src_update_time: String,
    update_time: String,
    #[serde(rename = "VersionID")]
    version_id: i64,
    station_position: StationPosition,
    location_city: String,
    location_city_code: String,
    location_town: String,
    location_town_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StationPosition {
    position_lon: i64,
    position_lat: i64,
    geo_hash: String,
}


pub type RailRoutes = Vec<RailRoute>;

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Name {
    #[serde(rename = "Zh_tw")]
    zh_tw: String,
    en: String,
}

pub type FirstLastTimetables = Vec<FirstLastTimetable>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FirstLastTimetable {
    line_no: String,
    #[serde(rename = "LineID")]
    line_id: String,
    #[serde(rename = "StationID")]
    station_id: String,
    station_name: StationName,
    trip_head_sign: String,
    #[serde(rename = "DestinationStaionID")]
    destination_staion_id: String,
    destination_station_name: StationName,
    train_type: i64,
    first_train_time: String,
    last_train_time: String,
    service_day: MetroServiceDay,
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


pub type RailFrequencies = Vec<RailFrequency>;

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceDay {
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

pub type RailShapes = Vec<RailShape>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RailShape {
    line_no: Option<String>,
    #[serde(rename = "LineID")]
    line_id: String,
    line_name: LineName,
    update_time: String,
    geometry: String,
    encoded_polyline: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LineName {
    #[serde(rename = "Zh_tw")]
    zh_tw: String,
    en: String,
}

pub type MetroFares = Vec<MetroFare>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetroFare {
    #[serde(rename = "OriginStationID")]
    origin_station_id: String,
    origin_station_name: NStationName,
    #[serde(rename = "DestinationStationID")]
    destination_station_id: String,
    destination_station_name: NStationName,
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

pub type ThsrGeneralTimetables = Vec<ThsrGeneralTimetable>;

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
    service_day: ThsrServiceDay,
    src_update_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GeneralTrainInfo {
    train_no: String,
    direction: i64,
    #[serde(rename = "StartingStationID")]
    starting_station_id: String,
    starting_station_name: EndingStationName,
    #[serde(rename = "EndingStationID")]
    ending_station_id: String,
    ending_station_name: EndingStationName,
    note: EndingStationName,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EndingStationName {
    #[serde(rename = "Zh_tw")]
    zh_tw: String,
    en: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ThsrServiceDay {
    monday: i64,
    tuesday: i64,
    wednesday: i64,
    thursday: i64,
    friday: i64,
    saturday: i64,
    sunday: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StopTime {
    stop_sequence: i64,
    #[serde(rename = "StationID")]
    station_id: String,
    station_name: EndingStationName,
    arrival_time: String,
    departure_time: String,
}

pub type ThsrFares = Vec<ThsrFare>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ThsrFare {
    #[serde(rename = "OriginStationID")]
    origin_station_id: String,
    origin_station_name: NStationName,
    #[serde(rename = "DestinationStationID")]
    destination_station_id: String,
    destination_station_name: NStationName,
    direction: i64,
    fares: Vec<Fare>,
    src_update_time: String,
    update_time: String,
    #[serde(rename = "VersionID")]
    version_id: i64,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Fare {
    ticket_type: i64,
    fare_class: i64,
    cabin_class: i64,
    price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3Operators {
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
    operator_name: OperatorName,
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
pub struct V3TraStations {
    update_time: String,
    update_interval: i64,
    src_update_time: String,
    src_update_interval: i64,
    authority_code: String,
    stations: Vec<V3TraStation>,
    count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3TraStation {
    #[serde(rename = "StationUID")]
    station_uid: String,
    #[serde(rename = "StationID")]
    station_id: String,
    reservation_code: String,
    station_name: StationName,
    station_position: StationPosition,
    station_address: String,
    station_phone: String,
    station_class: String,
    #[serde(rename = "StationURL")]
    station_url: String,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StationPosition {
    position_lon: f64,
    position_lat: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3GeneralTrainTimetables {
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
    stop_times: Vec<V3StopTime>,
    service_day: V3ServiceDay,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3ServiceDay {
    service_tag: String,
    monday: i64,
    tuesday: i64,
    wednesday: i64,
    thursday: i64,
    friday: i64,
    saturday: i64,
    sunday: i64,
    national_holidays: i64,
    day_before_holiday: i64,
    day_after_holiday: i64,
    typhoon_day: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3StopTime {
    stop_sequence: i64,
    #[serde(rename = "StationID")]
    station_id: String,
    station_name: Name,
    arrival_time: String,
    departure_time: String,
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
    update_time: String,
    update_interval: i64,
    src_update_time: String,
    src_update_interval: i64,
    authority_code: String,
    shapes: Vec<V3Shape>,
    count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3Shape {
    line_no: String,
    #[serde(rename = "LineID")]
    line_id: String,
    line_name: LineName,
    geometry: String,
    update_time: String,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3OdFares {
    update_time: String,
    update_interval: i64,
    src_update_time: String,
    src_update_interval: i64,
    authority_code: String,
    effective_date: String,
    expire_date: String,
    src_version: String,
    #[serde(rename = "ODFares")]
    od_fares: Vec<V3OdFare>,
    count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3OdFare {
    #[serde(rename = "OriginStationID")]
    origin_station_id: String,
    origin_station_name: NStationName,
    #[serde(rename = "DestinationStationID")]
    destination_station_id: String,
    destination_station_name: NStationName,
    direction: i64,
    train_type: i64,
    fares: Vec<Fare>,
    travel_distance: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NStationName {
    #[serde(rename = "Zh_tw")]
    zh_tw: String,
    en: String,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3AfrStations {
    update_time: String,
    update_interval: i64,
    src_update_time: String,
    src_update_interval: i64,
    authority_code: String,
    stations: Vec<V3AfrStation>,
    count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct V3AfrStation {
    station_class: String,
    #[serde(rename = "StationUID")]
    station_uid: String,
    #[serde(rename = "StationID")]
    station_id: String,
    reservation_code: String,
    station_name: StationName,
    station_position: StationPosition,
    station_address: String,
    station_phone: String,
    #[serde(rename = "StationURL")]
    station_url: String,
}
