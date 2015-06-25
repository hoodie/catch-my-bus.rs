extern crate dvbrs;
use dvbrs::*;

fn main(){
    println!("monitor:\n{}\n", dvb::get_monitor("Slub").unwrap());

    println!("find_station:\n{}\n", dvb::find_station("slub").unwrap());
}
