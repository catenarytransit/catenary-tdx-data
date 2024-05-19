use catenary_tdx_data::*;
use core::result::Result;
//use iso8601::datetime;
use reqwest::{header::AUTHORIZATION, header::CONTENT_TYPE, *};
use serde_json::*;
use std::{collections::HashMap, env, error::Error, fs::File, path::Path};
//use std::io::Write;
use std::{thread, time::Duration};
use catenary_tdx_data::auth::{URL_HEAD,AUTH_URL};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    //let mut output = File::create("ilha_formosa.json")?;

    let bus = [
        //static bus by city
        "/v2/Bus/Route/City/city",
        "/v2/Bus/Stop/City/city",
        "/v2/Bus/Operator/City/city",
        "/v2/Bus/Schedule/City/city", //includes calender,trips,stop times,frequency
        "/v2/Bus/FirstLastTripInfo/City/city",
        "/v2/Bus/Shape/City/city",
        "/v2/Bus/RouteFare/City/city",
        //rt bus by city
        "/v2/Bus/RealTimeByFrequency/City/city",
        "/v2/Bus/RealTimeNearStop/City/city",
        "/v2/Bus/EstimatedTimeOfArrival/City/city",
        "/v2/Bus/Alert/City/city",
    ];
    let ic_bus = [
        //static intercity bus
        "/v2/Bus/Route/InterCity",
        "/v2/Bus/Stop/InterCity",
        "/v2/Bus/Operator/InterCity",
        "/v2/Bus/Schedule/InterCity", //includes calender,trips,stop times,frequency
        "/v2/Bus/FirstLastTripInfo/InterCity",
        "/v2/Bus/Shape/InterCity",
        "/v2/Bus/RouteFare/InterCity",
        //rt intercity bus
        "/v2/Bus/RealTimeByFrequency/InterCity",
        "/v2/Bus/RealTimeNearStop/InterCity",
        "/v2/Bus/EstimatedTimeOfArrival/InterCity",
        "/v2/Bus/Alert/InterCity",
    ];
    let metro = [
        //static metro
        "/v2/Rail/Metro/Station/metrosystem",
        "/v2/Rail/Metro/Route/metrosystem",
        "/v2/Rail/Metro/FirstLastTimetable/metrosystem",
        "/v2/Rail/Metro/Frequency/metrosystem",
        "/v2/Rail/Metro/Shape/metrosystem",
        "/v2/Rail/Metro/ODFare/metrosystem",
        //rt metro
        "/v2/Rail/Metro/LiveBoard/metrosystem",
        "/v2/Rail/Metro/StationTimeTable/metrosystem",
        "/v2/Rail/Metro/Alert/metrosystem",
    ];
    let rail = [
        //static rail
        "/v2/Rail/Operator",              //also for metro
        "/v2/Rail/THSR/Station", //theres only one line so they dont have routes
        "/v2/Rail/THSR/GeneralTimetable", //calender, trips, stop times
        "/v2/Rail/THSR/Shape",
        "/v2/Rail/THSR/ODFare",
        "/v3/Rail/TRA/Operator",
        "/v3/Rail/TRA/Station",
        "/v3/Rail/TRA/GeneralTrainTimetable", //calender, trips, stop times
        "/v3/Rail/TRA/Shape",
        "/v3/Rail/TRA/ODFare",
        "/v3/Rail/AFR/Operator",
        "/v3/Rail/AFR/Station",
        "/v3/Rail/AFR/Route",
        "/v3/Rail/AFR/GeneralTrainTimetable",
        "/v3/Rail/AFR/Shape",
        "/v3/Rail/AFR/ODFare",
        //rt rail
        "/v3/Rail/TRA/TrainLiveBoard",
        "/v3/Rail/TRA/StationLiveBoard",
        "/v3/Rail/TRA/Alert",
        "/v2/Rail/THSR/AlertInfo",
    ];
    let city = [
        "Taipei",
        "NewTaipei",
        "Taoyuan",
        "Taichung",
        "Tainan", //also in v3 dataset, might add it as v3? idk if its repeat
        "Kaohsiung",
        "Keelung",
        "Hsinchu",
        "HsinchuCounty",
        "MiaoliCounty",
        "ChanghuaCounty",
        "NantouCounty",
        "YunlinCounty",
        "ChiayiCounty",
        "Chiayi",
        "PingtungCounty",
        "YilanCounty",
        "HualienCounty",
        "TaitungCounty",
        "KinmenCounty",
        "PenghuCounty",
        "LienchiangCounty",
    ];
    let metrosystem = [
        "TRTC", //has live
        "KRTC", //has live
        "KLRT", //has live
        "TYMC",
        "TRTCMG", //gondola :D
        "TMRT",
        "NTMC",
        "NTALRT",
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


        let client = Client::new();

    let token = catenary_tdx_data::auth::get_token_header(secret.get("client_id").unwrap(), secret.get("client_secret").unwrap()).await?;

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
    thread::sleep(Duration::from_secs(5));

    Ok(client
        .get(&query_url)
        .header(AUTHORIZATION, token)
        .send()
        .await?)
}
