#![allow(unused_imports)]
extern crate url;
extern crate hyper;
extern crate rustc_serialize;

use std::fs::File;
use std::io::Read;
use std::path::{Path};
use std::collections::HashMap;
use url::{Url};
use hyper::Client;
use rustc_serialize::json::Json;

const VVO_URL:&'static str = "http://widgets.vvo-online.de/abfahrtsmonitor/Abfahrten.do?ort=Dresden&vz=0&hst=";

#[allow(dead_code)]
fn get_content(url: Url) -> String
{
    let mut client = Client::new();
    let mut res = client.get(url).send().unwrap();
    assert_eq!(res.status, hyper::Ok);
    // Read the Response.
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    body
}

fn get_content_offline() -> String
{
    let mut file = File::open(&Path::new("./kaitzerstrasse.js")).ok().expect("io error");
    //let mut file = File::open(&Path::new("./kaitzerstrasse.js")).ok().expect("io error");
    let mut string = String::new();
    file.read_to_string(&mut string).ok().expect("read error");
    string
}

pub fn get_station_json(station: &str) -> Json
{
    #![allow(unused_variables)]
    let url: Url = Url::parse(
        &(VVO_URL.to_string() + station)
        ).unwrap();
    //println!("getting \"{}\" ({})", &station, &url);
    let json_str = get_content(url);
    //let json_str = get_content_offline();
    let data = Json::from_str(&json_str).unwrap();
    return data
}

pub fn group_by_line(data: &Json) -> HashMap<String, Vec<String>>
{
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    if let Some(list) = data.as_array() {
        for arrival in list {
            let line = arrival[0].as_string().unwrap();
            let direction = arrival[1].as_string().unwrap();
            let route = format!("{} {}", line.to_string(),  direction);
            let time = format!("{}min ", arrival[2].as_string().unwrap());

            let mut list = map.entry(route).or_insert(vec![]);
            list.push(time);
        }
    }
    return map;
}

