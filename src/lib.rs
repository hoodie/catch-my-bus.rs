extern crate url;
extern crate hyper;
extern crate rustc_serialize;
extern crate time;
extern crate datetime;

pub mod dvb;
pub mod config;

#[test]
fn getting_station(){
    assert!(dvb::get_station_json("Slub").is_ok());
}
