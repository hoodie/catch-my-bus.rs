extern crate notify_rust;
extern crate url;
extern crate hyper;
extern crate rustc_serialize;

mod config;
mod dvb;
use notify_rust::Notification;

fn main() {
    let stations = config::read_config();
    for station in &stations{
        let json = dvb::get_station_json(station);
        let map = dvb::group_by_line(&json);
        for (k, v) in map{

            let body = &format!("{}:\n {}", &k,v.connect(", "));
            Notification::new()
                .appname(&format!("catch_my_bus {}", &k))
                .icon("/home/hendrik/code/rust/catch_my_bus/Bushaltestelle.png")
                .summary(&station)
                .body(body).show_debug();
        }

    }
}

