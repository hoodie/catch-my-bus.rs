extern crate dvbrs;
use dvbrs::*;
extern crate datetime;
use datetime::local::{DatePiece,LocalDateTime};

fn main(){
    //println!("monitor:\n{}\n", dvb::get_monitor("slub","Dresden").unwrap());

    //println!("find_station:\n{}\n", dvb::find_station("slub").unwrap());

    //println!("get_route:\n{}\n", dvb::get_route().unwrap());
    
     
    println!("{}", LocalDateTime::now().day());
    println!("{}", LocalDateTime::now().month() as i32 + 1);
    println!("{}", LocalDateTime::now().year());
}
