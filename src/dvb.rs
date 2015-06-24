use std::fs::File;
use std::io::Read;
use std::path::{Path};
use url::{Url};
use std::collections::HashMap;
use rustc_serialize::json::Json;
use hyper::{Client, Ok};
use hyper::status::StatusCode;

const VVO_URL:&'static str = "http://widgets.vvo-online.de/abfahrtsmonitor/Abfahrten.do?ort=Dresden&vz=0&hst=";

#[allow(dead_code)]
#[allow(unused)]
fn get_content(url: Url) -> Result<String, StatusCode>
{
    let mut client = Client::new();
    let mut res = client.get(url).send().unwrap();
    match res.status
    {
        StatusCode::Ok => {
            let mut body = String::new();
            res.read_to_string(&mut body).unwrap();
            return Result::Ok(body)
        },
        _ => (Err(res.status)),
    }
    // Read the Response.
}

#[allow(unused)]
fn get_content_offline() -> String
{
    let mut file = File::open(&Path::new("./kaitzerstrasse.js")).ok().expect("io error");
    //let mut file = File::open(&Path::new("./kaitzerstrasse.js")).ok().expect("io error");
    let mut string = String::new();
    file.read_to_string(&mut string).ok().expect("read error");
    string
}

pub fn get_station_json(station: &str) -> Result<Json,StatusCode>
{
    #![allow(unused_variables)]
    let url: Url = Url::parse(
        &(VVO_URL.to_string() + station)
        ).unwrap();
    //println!("getting \"{}\" ({})", &station, &url);
    return get_content(url).map(|str| Json::from_str(&str).unwrap());

}

#[allow(unused)]
pub fn group_by_line(data: &Json) -> HashMap<String, Vec<String>>
{
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    if let Some(list) = data.as_array() {
        for arrival in list {
            let line = arrival[0].as_string().unwrap();
            let direction = arrival[1].as_string().unwrap();
            let route = format!("{} {}", line.to_string(),  direction);
            let minutes_string = arrival[2].as_string().unwrap();
            //let minutes = time::Duration::minutes(minutes_string.parse::<i64>().unwrap_or(-1));
            let wait = format!("{}min ", minutes_string);

            let mut list = map.entry(route).or_insert(vec![]);
            list.push(wait);
        }
    }
    return map;
}
