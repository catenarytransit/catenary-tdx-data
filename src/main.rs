use catenary_tdx_data::*;
use core::result::Result;
//use iso8601::datetime;
use reqwest::{header::AUTHORIZATION, header::CONTENT_TYPE, *};
use serde_json::*;
use std::{collections::HashMap, env, error::Error, fs::File, io::Write, path::Path};

static AUTH_URL: &str =
    "https://tdx.transportdata.tw/auth/realms/TDXConnect/protocol/openid-connect/token";
static URL_HEAD: &str = "https://tdx.transportdata.tw/api/basic";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut output = File::create("ilha_formosa.json")?;

    let endpoint_links: Vec<String> = vec![
        //rt bus
        "/v2/Bus/RealTimeByFrequency/City/city".to_string(),
        "/v2/Bus/RealTimeNearStop/City/city".to_string(),
        "/v2/Bus/EstimatedTimeOfArrival/City/city".to_string(),
        "/v2/Bus/Alert/City/city".to_string(),
        "/v2/Bus/RealTimeByFrequency/InterCity".to_string(),
        "/v2/Bus/RealTimeNearStop/InterCity".to_string(),
        "/v2/Bus/EstimatedTimeOfArrival/InterCity".to_string(),
        "/v2/Bus/Alert/InterCity".to_string(),
        //static bus
        "/v2/Bus/Operator/City/city".to_string(),
        "/v2/Bus/Stop/City/city".to_string(),
        "/v2/Bus/Route/City/city".to_string(),
        "/v2/Bus/Schedule/City/city".to_string(), //includes calender,trips,stop times,frequency
        "/v2/Bus/FirstLastTripInfo/City/city".to_string(),
        "/v2/Bus/Shape/City/city".to_string(),
        "/v2/Bus/RouteFare/City/city".to_string(),
        "/v2/Bus/Operator/InterCity".to_string(),
        "/v2/Bus/Stop/InterCity".to_string(),
        "/v2/Bus/Route/InterCity".to_string(),
        "/v2/Bus/Schedule/InterCity".to_string(), //includes calender,trips,stop times,frequency
        "/v2/Bus/FirstLastTripInfo/InterCity".to_string(),
        "/v2/Bus/Shape/InterCity".to_string(),
        "/v2/Bus/RouteFare/InterCity".to_string(),
        //rt rail
        "/v3/Rail/TRA/TrainLiveBoard".to_string(),
        "/v3/Rail/TRA/StationLiveBoard".to_string(),
        "/v3/Rail/TRA/Alert".to_string(),
        "/v2/Rail/Metro/LiveBoard/railsystem".to_string(),
        "/v2/Rail/Metro/StationTimeTable/railsystem".to_string(),
        "/v2/Rail/Metro/Alert/railsystem".to_string(),
        "/v2/Rail/THSR/AlertInfo".to_string(),
        //static rail
        "/v2/Rail/Operator".to_string(),
        "/v2/Rail/Metro/Station/railsystem".to_string(),
        "/v2/Rail/Metro/Route/railsystem".to_string(),
        "/v2/Rail/Metro/FirstLastTimetable/railsystem".to_string(),
        "/v2/Rail/Metro/Frequency/railsystem".to_string(),
        "/v2/Rail/Metro/Shape/railsystem".to_string(),
        "/v2/Rail/Metro/ODFare/railsystem".to_string(),
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

    let railsystem = vec![
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
    let access_token = format!("Bearer {}", data_header.split_once("\",").unwrap().0);

    let list_item_counter: usize = 0;
    for mut endpoint in endpoint_links {
        if list_item_counter < city.len() {
            endpoint = endpoint.replace("city", &city[list_item_counter]);
        }
        if list_item_counter < railsystem.len() {
            endpoint = endpoint.replace("railsystem", &railsystem[list_item_counter]);
        }
        let query_url = format!("{}{}", URL_HEAD, endpoint);

        let data = client
            .get(&query_url)
            .header(AUTHORIZATION, &access_token)
            .send()
            .await?
            .text()
            .await?;
        print!("{:?}", data); //checks if the data is even there

        if let Some(data) = client
            .get(&query_url)
            .header(AUTHORIZATION, &access_token)
            .send()
            .await?
            .json::<TrainLiveBoard>()
            .await
            .ok()
        {
            print!("{:?}", data);
        }

        write!(output, "{}", data).expect("file dne");
    }
    Ok(())
}
