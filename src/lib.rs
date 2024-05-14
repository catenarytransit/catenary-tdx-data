use serde::{Deserialize, Serialize};

//just a lot of struct to deserialize (decerealize? un-cornflæke?) everything

//top level structs, lists of items
pub type BusRouteList = Vec<BusRouteItem>;
pub type RealTimeByFrequency = Vec<RealTimeByFrequencyItem>;

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct TRATrainLiveBoardList {
    UpdateTime: String, //"2024-05-12T05:18:17.454Z"
    UpdateInterval: i32,
    SrcUpdateTime: String,
    SrcUpdateInterval: i32,
    AuthorityCode: String,
    TrainLiveBoards: Vec<TrainLiveBoardItem>,
}

//second level structs, items
#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct BusRouteItem {
    RouteUID: String,
    RouteID: String,
    HasSubRoutes: bool,
    Operators: Vec<RouteOperator>,
    AuthorityID: String,
    ProviderID: String,
    SubRoutes: Vec<BusSubRoute>,
    BusRouteType: i32,
    RouteName: NameType,
    DepartureStopNameZh: String,
    DepartureStopNameEn: String,
    DestinationStopNameZh: String,
    DestinationStopNameEn: String,
    TicketPriceDescriptionZh: String,
    TicketPriceDescriptionEn: String,
    FareBufferZoneDescriptionZh: String,
    FareBufferZoneDescriptionEn: String,
    RouteMapImageUrl: String,
    City: String,
    CityCode: String,
    UpdateTime: String,
    VersionID: i32,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct RealTimeByFrequencyItem {
    PlateNumb: String,
    OperatorID: String,
    OperatorNo: String,
    RouteUID: String,
    RouteID: String,
    RouteName: NameType,
    SubRouteUID: String,
    SubRouteID: String,
    SubRouteName: NameType,
    Direction: i32,
    BusPosition: PointType,
    Speed: f64,
    Azimuth: f64,
    DutyStatus: i32,
    BusStatus: i32,
    MessageType: i32,
    GPSTime: String,
    TransTime: String,
    SrcRecTime: String,
    SrcTransTime: String,
    SrcUpdateTime: String,
    UpdateTime: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct TrainLiveBoardItem {
    TrainNo: String,
    TrainTypeID: String,
    TrainTypeCode: String,
    TrainTypeName: NameType,
    StationID: String,
    StationName: NameType,
    TrainStationStatus: i32,
    DelayTime: i32,
    UpdateTime: String,
}

//third level structs, stores other information
#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct BusSubRoute {
    SubRouteUID: String,
    SubRouteID: String,
    OperatorIDs: Vec<String>,
    SubRouteName: NameType,
    Headsign: String,
    HeadsignEn: String,
    Direction: i32,
    FirstBusTime: String,
    LastBusTime: String,
    HolidayFirstBusTime: String,
    HolidayLastBusTime: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct NameType {
    Zh_tw: String,
    En: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct PointType {
    PositionLon: f64,
    PositionLat: f64,
    GeoHash: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct RouteOperator {
    OperatorID: String,
    OperatorName: NameType,
    OperatorCode: String,
    OperatorNo: String,
}
