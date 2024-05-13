//use std::collections::HashMap;
use serde::{Deserialize, Serialize};

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
    pub TrainNo: String,
    pub TrainTypeID: String,
    pub TrainTypeCode: String,
    pub TrainTypeName: NameType,
    pub StationID: String,
    pub StationName: NameType,
    pub TrainStationStatus: i32,
    pub DelayTime: i32,
    pub UpdateTime: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct NameType {
    pub Zh_tw: String,
    pub En: String,
}
