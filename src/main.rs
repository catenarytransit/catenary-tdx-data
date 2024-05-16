use catenary_tdx_data::*;
use core::result::Result;
//use iso8601::datetime;
use reqwest::{header::AUTHORIZATION, header::CONTENT_TYPE, *};
use serde_json::*;
use std::{collections::HashMap, env, error::Error, fs::File, path::Path};
//use std::io::Write;
use std::{thread, time::Duration};

static AUTH_URL: &str =
    "https://tdx.transportdata.tw/auth/realms/TDXConnect/protocol/openid-connect/token";
static URL_HEAD: &str = "https://tdx.transportdata.tw/api/basic";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    //let mut output = File::create("ilha_formosa.json")?;

    let bus: Vec<String> = vec![
        //static bus by city
        "/v2/Bus/Route/City/city".to_string(),
        "/v2/Bus/Stop/City/city".to_string(),
        "/v2/Bus/Operator/City/city".to_string(),
        "/v2/Bus/Schedule/City/city".to_string(), //includes calender,trips,stop times,frequency
        "/v2/Bus/FirstLastTripInfo/City/city".to_string(),
        "/v2/Bus/Shape/City/city".to_string(),
        "/v2/Bus/RouteFare/City/city".to_string(),
        //rt bus by city
        "/v2/Bus/RealTimeByFrequency/City/city".to_string(),
        "/v2/Bus/RealTimeNearStop/City/city".to_string(),
        "/v2/Bus/EstimatedTimeOfArrival/City/city".to_string(),
        "/v2/Bus/Alert/City/city".to_string(),
    ];
    let ic_bus = vec![
        //static intercity bus
        "/v2/Bus/Route/InterCity".to_string(),
        "/v2/Bus/Stop/InterCity".to_string(),
        "/v2/Bus/Operator/InterCity".to_string(),
        "/v2/Bus/Schedule/InterCity".to_string(), //includes calender,trips,stop times,frequency
        "/v2/Bus/FirstLastTripInfo/InterCity".to_string(),
        "/v2/Bus/Shape/InterCity".to_string(),
        "/v2/Bus/RouteFare/InterCity".to_string(),
        //rt intercity bus
        "/v2/Bus/RealTimeByFrequency/InterCity".to_string(),
        "/v2/Bus/RealTimeNearStop/InterCity".to_string(),
        "/v2/Bus/EstimatedTimeOfArrival/InterCity".to_string(),
        "/v2/Bus/Alert/InterCity".to_string(),
    ];
    let metro = vec![
        //static metro
        "/v2/Rail/Metro/Station/metrosystem".to_string(),
        "/v2/Rail/Metro/Route/metrosystem".to_string(),
        "/v2/Rail/Metro/FirstLastTimetable/metrosystem".to_string(),
        "/v2/Rail/Metro/Frequency/metrosystem".to_string(),
        "/v2/Rail/Metro/Shape/metrosystem".to_string(),
        "/v2/Rail/Metro/ODFare/metrosystem".to_string(),
        //rt metro
        "/v2/Rail/Metro/LiveBoard/metrosystem".to_string(),
        "/v2/Rail/Metro/StationTimeTable/metrosystem".to_string(),
        "/v2/Rail/Metro/Alert/metrosystem".to_string(),
    ];
    let rail = vec![
        //static rail
        "/v2/Rail/Operator".to_string(),              //also for metro
        "/v2/Rail/THSR/Station".to_string(), //theres only one line so they dont have routes
        "/v2/Rail/THSR/GeneralTimetable".to_string(), //calender, trips, stop times
        "/v2/Rail/THSR/Shape".to_string(),
        "/v2/Rail/THSR/ODFare".to_string(),
        "/v3/Rail/TRA/Operator".to_string(),
        "/v3/Rail/TRA/Station".to_string(),
        "/v3/Rail/TRA/GeneralTrainTimetable".to_string(), //calender, trips, stop times
        "/v3/Rail/TRA/Shape".to_string(),
        "/v3/Rail/TRA/ODFare".to_string(),
        "/v3/Rail/AFR/Operator".to_string(),
        "/v3/Rail/AFR/Station".to_string(),
        "/v3/Rail/AFR/Route".to_string(),
        "/v3/Rail/AFR/GeneralTrainTimetable".to_string(),
        "/v3/Rail/AFR/Shape".to_string(),
        "/v3/Rail/AFR/ODFare".to_string(),
        //rt rail
        "/v3/Rail/TRA/TrainLiveBoard".to_string(),
        "/v3/Rail/TRA/StationLiveBoard".to_string(),
        "/v3/Rail/TRA/Alert".to_string(),
        "/v2/Rail/THSR/AlertInfo".to_string(),
    ];
    let city: Vec<String> = vec![
        "Taipei".to_string(),
        "NewTaipei".to_string(),
        "Taoyuan".to_string(),
        "Taichung".to_string(),
        "Tainan".to_string(), //also in v3 dataset, might add it as v3? idk if its repeat
        "Kaohsiung".to_string(),
        "Keelung".to_string(),
        "Hsinchu".to_string(),
        "HsinchuCounty".to_string(),
        "MiaoliCounty".to_string(),
        "ChanghuaCounty".to_string(),
        "NantouCounty".to_string(),
        "YunlinCounty".to_string(),
        "ChiayiCounty".to_string(),
        "Chiayi".to_string(),
        "PingtungCounty".to_string(),
        "YilanCounty".to_string(),
        "HualienCounty".to_string(),
        "TaitungCounty".to_string(),
        "KinmenCounty".to_string(),
        "PenghuCounty".to_string(),
        "LienchiangCounty".to_string(),
    ];
    let metrosystem: Vec<String> = vec![
        "TRTC".to_string(), //has live
        "KRTC".to_string(), //has live
        "KLRT".to_string(), //has live
        "TYMC".to_string(),
        "TRTCMG".to_string(), //gondola :D
        "TMRT".to_string(),
        "NTMC".to_string(),
        "NTALRT".to_string(),
    ];

    let raw_path = match env::consts::ARCH {
        "x86_64" => format!(
            "C:\\Users\\{}\\Downloads\\tdx-secret.json",
            env::var("USERNAME")?
        ),

        &_ => todo!(),
    };
    let file_path = Path::new(&raw_path);
    let file = File::open(file_path).expect("file not found");
    let secret: HashMap<String, String> =
        serde_json::from_reader(file).expect("error while reading");

    let auth_header = json!({
        "grant_type": "client_credentials",
        "client_id": secret.get("client_id").unwrap(),
        "client_secret": secret.get("client_secret").unwrap()
    });

    let client = Client::new();
    let auth_response = client
        .post(AUTH_URL)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .form(&auth_header)
        .send()
        .await?
        .text()
        .await?;

    let data_header = auth_response.split_once("\":\"").unwrap().1;
    let token = format!("Bearer {}", data_header.split_once("\",").unwrap().0);

    for loc in city.iter() {
        fetch(&bus[0].replace("city", loc), &token, &client)
            .await?
            .json::<BusRoutes>()
            .await?;
        fetch(&bus[1].replace("city", loc), &token, &client)
            .await?
            .json::<BusStops>()
            .await?;
        fetch(&bus[2].replace("city", loc), &token, &client)
            .await?
            .json::<Operators>()
            .await?;
        fetch(&bus[3].replace("city", loc), &token, &client)
            .await?
            .json::<BusSchedules>()
            .await?;
        fetch(&bus[4].replace("city", loc), &token, &client)
            .await?
            .json::<FirstLastTripInfo>()
            .await?;
        fetch(&bus[5].replace("city", loc), &token, &client)
            .await?
            .json::<BusShapes>()
            .await?;
        fetch(&bus[6].replace("city", loc), &token, &client)
            .await?
            .json::<BusRouteFare>()
            .await?;
        fetch(&bus[7].replace("city", loc), &token, &client)
            .await?
            .json::<BusRtFrequency>()
            .await?;
        fetch(&bus[8].replace("city", loc), &token, &client)
            .await?
            .json::<BusRtStops>()
            .await?;
        fetch(&bus[9].replace("city", loc), &token, &client)
            .await?
            .json::<BusEta>()
            .await?;
        fetch(&bus[10].replace("city", loc), &token, &client)
            .await?
            .json::<BusAlerts>()
            .await?;
    }

    fetch(&ic_bus[0], &token, &client)
        .await?
        .json::<BusRoutes>()
        .await?;
    fetch(&ic_bus[1], &token, &client)
        .await?
        .json::<BusStops>()
        .await?;
    fetch(&ic_bus[2], &token, &client)
        .await?
        .json::<Operators>()
        .await?;
    fetch(&ic_bus[3], &token, &client)
        .await?
        .json::<BusSchedules>()
        .await?;
    fetch(&ic_bus[4], &token, &client)
        .await?
        .json::<FirstLastTripInfo>()
        .await?;
    fetch(&ic_bus[5], &token, &client)
        .await?
        .json::<BusShapes>()
        .await?;
    fetch(&ic_bus[6], &token, &client)
        .await?
        .json::<BusRouteFare>()
        .await?;
    fetch(&ic_bus[7], &token, &client)
        .await?
        .json::<BusRtFrequency>()
        .await?;
    fetch(&ic_bus[8], &token, &client)
        .await?
        .json::<BusRtStops>()
        .await?;
    fetch(&ic_bus[9], &token, &client)
        .await?
        .json::<BusEta>()
        .await?;
    fetch(&ic_bus[10], &token, &client)
        .await?
        .json::<BusAlerts>()
        .await?;

    for loc in metrosystem.iter() {
        fetch(&metro[0].replace("metrosystem", loc), &token, &client)
            .await?
            .json::<RailStations>()
            .await?;
        fetch(&metro[1].replace("metrosystem", loc), &token, &client)
            .await?
            .json::<RailRoutes>()
            .await?;
        fetch(&metro[2].replace("metrosystem", loc), &token, &client)
            .await?
            .json::<FirstLastTimetables>()
            .await?;
        fetch(&metro[3].replace("metrosystem", loc), &token, &client)
            .await?
            .json::<RailFrequencies>()
            .await?;
        fetch(&metro[4].replace("metrosystem", loc), &token, &client)
            .await?
            .json::<RailShapes>()
            .await?;
        fetch(&metro[5].replace("metrosystem", loc), &token, &client)
            .await?
            .json::<MetroFares>()
            .await?;
        fetch(&metro[6].replace("metrosystem", loc), &token, &client)
            .await?
            .json::<MetroLiveBoard>()
            .await?;
        fetch(&metro[7].replace("metrosystem", loc), &token, &client)
            .await?
            .json::<MetroStationTimeTable>()
            .await?;
        fetch(&metro[8].replace("metrosystem", loc), &token, &client)
            .await?
            .json::<RailAlerts>()
            .await?;
    }

    fetch(&rail[0], &token, &client)
        .await?
        .json::<Operators>()
        .await?;
    fetch(&rail[1], &token, &client)
        .await?
        .json::<RailStations>()
        .await?;
    fetch(&rail[2], &token, &client)
        .await?
        .json::<ThsrGeneralTimetables>()
        .await?;
    fetch(&rail[3], &token, &client)
        .await?
        .json::<RailShapes>()
        .await?;
    fetch(&rail[4], &token, &client)
        .await?
        .json::<ThsrFares>()
        .await?;
    fetch(&rail[5], &token, &client)
        .await?
        .json::<V3RailOperators>()
        .await?;
    fetch(&rail[6], &token, &client)
        .await?
        .json::<V3RailStations>()
        .await?;
    fetch(&rail[7], &token, &client)
        .await?
        .json::<V3GeneralTrainTimetables>()
        .await?;
    fetch(&rail[8], &token, &client)
        .await?
        .json::<V3RailShape>()
        .await?;
    fetch(&rail[9], &token, &client)
        .await?
        .json::<V3OdFares>()
        .await?;
    fetch(&rail[10], &token, &client)
        .await?
        .json::<V3RailOperators>()
        .await?;
    fetch(&rail[11], &token, &client)
        .await?
        .json::<V3RailStations>()
        .await?;
    fetch(&rail[12], &token, &client)
        .await?
        .json::<V3RailRoutes>()
        .await?;
    fetch(&rail[13], &token, &client)
        .await?
        .json::<V3GeneralTrainTimetables>()
        .await?;
    fetch(&rail[14], &token, &client)
        .await?
        .json::<V3RailShapes>()
        .await?;
    fetch(&rail[15], &token, &client)
        .await?
        .json::<V3OdFares>()
        .await?;
    fetch(&rail[16], &token, &client)
        .await?
        .json::<TrainLiveBoard>()
        .await?;
    fetch(&rail[17], &token, &client)
        .await?
        .json::<StationLiveBoard>()
        .await?;
    fetch(&rail[18], &token, &client)
        .await?
        .json::<RailAlerts>()
        .await?;
    fetch(&rail[19], &token, &client)
        .await?
        .json::<ThsrAlertInfo>()
        .await?;

    Ok(())
}

async fn fetch(
    endpoint: &str,
    token: &String,
    client: &Client,
) -> Result<Response, Box<dyn Error + Send + Sync>> {
    let query_url = format!("{}{}", URL_HEAD, endpoint);
    println!("{}\t", query_url);
    thread::sleep(Duration::from_secs(1));

    Ok(client
        .get(&query_url)
        .header(AUTHORIZATION, token)
        .send()
        .await?)
}
