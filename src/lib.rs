//use std::collections::HashMap;
use iso8601::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct TRATrainLiveBoardList {
    UpdateTime: String, //"2024-05-12T05:18:17.454Z" might need parser? idk
    UpdateInterval: i32,
    SrcUpdateTime: String,
    SrcUpdateInterval: i32,
    AuthorityCode: String,
    TrainLiveBoards: Vec<TrainLiveBoards>,
    Count: i64,
}

#[derive(Deserialize, Debug)]
pub struct TrainLiveBoards {
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

#[derive(Deserialize, Debug)]
pub struct NameType {
    Zh_tw: String,
    En: String,
}
