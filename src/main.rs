extern crate notify_rust;
extern crate url;
extern crate hyper;
extern crate rustc_serialize;

mod config;
mod dvb;
use notify_rust::Notification;

fn run() {
    let config::Config{sleep_time:_, stations:stations} = config::read_config();
    loop{
        for station in &stations{
            match dvb::get_station_json(station){
                Ok(json) =>{
                    let map = dvb::group_by_line(&json);
                    for (k, v) in map{
                        let body = &format!("{}:\n  {}", &k,v.connect(", "));
                        Notification::new()
                            .appname(&format!("catch_my_bus {}", &k))
                            .icon("/home/hendrik/code/rust/catch_my_bus/Bushaltestelle.png")
                            .summary(&station)
                            .body(body).show();
                    }
                },
                Err(error) => {
                    Notification::new()
                        .appname(&format!("catch_my_bus"))
                        .icon("/home/hendrik/code/rust/catch_my_bus/Bushaltestelle.png")
                        .summary("Error")
                        .body(&format!("{:?}", error))
                        .show_debug();
                    println!("{:?}",error);
                    break;
                }
            }
        }
        std::thread::sleep_ms(60_000);
    }
}


use std::thread;
fn main()
{
    thread::spawn(move||{run()});
    println!("Press enter to exit.");
    let mut devnull= String::new();
    std::io::stdin().read_line(&mut devnull);
    println!("Thank you for choosing catch_my_bus-rs.");
}
