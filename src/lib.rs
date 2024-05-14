use serde::{Deserialize, Serialize};

//just a lot of struct to deserialize (decerealize? un-cornflæke?) everything

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct RealTimeByFrequency {
    FrequencyList: Vec<RealTimeByFrequencyItem>
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct TRATrainLiveBoardList {
    pub UpdateTime: String, //"2024-05-12T05:18:17.454Z"
    pub UpdateInterval: i32,
    pub SrcUpdateTime: String,
    pub SrcUpdateInterval: i32,
    pub AuthorityCode: String,
    pub TrainLiveBoards: Vec<TrainLiveBoards>,
    //Count: i64,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct TrainLiveBoards {
    TrainNo: String,
    TrainTypeID: String,
    TrainTypeCode: String,
    TrainTypeName: NameType,
    StationID: String,
    StationName: NameType,
    TrainStationStatus: i32,
    DelayTime: i32,
    pub UpdateTime: String,
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
    pub UpdateTime: String,
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
    pub PositionLon: f64,
    pub PositionLat: f64,
    pub GeoHash: String,
}
