extern crate dvbrs;
use dvbrs::*;

fn main(){
    //println!("monitor:\n{}\n", dvb::get_monitor("slub","Dresden").unwrap());

    //println!("find_station:\n{}\n", dvb::find_station("slub").unwrap());

    println!("get_route:\n{}\n", dvb::get_route().unwrap());
}
