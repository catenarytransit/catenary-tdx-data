use serde::{Deserialize, Serialize};

//just a lot of struct to deserialize (decerealize? un-cornflæke?) everything
/*
done list: TrainLiveBoard, Routes (bus)
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