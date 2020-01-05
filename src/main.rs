mod plc_driver;

extern crate bit_vec;
extern crate chrono;
extern crate clap;
extern crate env_logger;
extern crate futures;
extern crate log;
extern crate regex;
extern crate snap7_sys;

use chrono::Local;
use env_logger::Builder;
use futures::future::{ok, Either};
use futures::{Future, Stream};
use log::{error, info, warn, LevelFilter};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::io::Write;
use std::time::Duration;
use std::{env, process};

use bit_vec::BitVec;

use plc_driver::s7::Client;
use plc_driver::{ETag, ETagRW, ETagValue, ETagtype};

fn main() {
    // Initialize the logger from the environment
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] <{}:{}> - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                buf.default_styled_level(record.level()),
                record.module_path().unwrap_or("<unnamed>"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
    info!("I am here");

    let mut client = Client::new();

    client.connect("10.0.0.230", 0, 1);

    warn!("{:#?}", client);

    let mut tag_for_read = ETag {
        name: String::from("test"),
        address: String::from("DB2W2"),
        datatype: ETagtype::INT,
    };
    client.write_tag(&mut tag_for_read, ETagValue::Int(8712));
    info!("{:#?}", client.read_tag(&tag_for_read).unwrap());

    let mut tag_for_read = ETag {
        name: String::from("test"),
        address: String::from("DB2D4"),
        datatype: ETagtype::REAL,
    };
    client.write_tag(&mut tag_for_read, ETagValue::Real(565.25));
    info!("{:#?}", client.read_tag(&tag_for_read).unwrap());

    let mut tag_for_read = ETag {
        name: String::from("test"),
        address: String::from("DB2X9.0"),
        datatype: ETagtype::BOOL,
    };
    info!("{:#?}", client.read_tag(&tag_for_read).unwrap());

    let mut tag_for_read = ETag {
        name: String::from("test"),
        address: String::from("DB2X9.1"),
        datatype: ETagtype::BOOL,
    };
    client.write_tag(&mut tag_for_read, ETagValue::Bool(true));
    info!("{:#?}", client.read_tag(&tag_for_read).unwrap());

    let mut tag_for_read = ETag {
        name: String::from("test"),
        address: String::from("DB2X9.2"),
        datatype: ETagtype::BOOL,
    };
    info!("{:#?}", client.read_tag(&tag_for_read).unwrap());

    let mut tag_for_read = ETag {
        name: String::from("test"),
        address: String::from("DB2X9.3"),
        datatype: ETagtype::BOOL,
    };
    info!("{:#?}", client.read_tag(&tag_for_read).unwrap());

    let mut tag_for_read = ETag {
        name: String::from("test"),
        address: String::from("DB2X9.4"),
        datatype: ETagtype::BOOL,
    };
    info!("{:#?}", client.read_tag(&tag_for_read).unwrap());

    let mut tag_for_read = ETag {
        name: String::from("test"),
        address: String::from("DB2X9.5"),
        datatype: ETagtype::BOOL,
    };
    client.write_tag(&mut tag_for_read, ETagValue::Bool(false));
    info!("{:#?}", client.read_tag(&tag_for_read).unwrap());

    let mut tag_for_read = ETag {
        name: String::from("test"),
        address: String::from("DB2X9.6"),
        datatype: ETagtype::BOOL,
    };
    info!("{:#?}", client.read_tag(&tag_for_read).unwrap());

    let mut tag_for_read = ETag {
        name: String::from("test"),
        address: String::from("DB2X9.7"),
        datatype: ETagtype::BOOL,
    };
    info!("{:#?}", client.read_tag(&tag_for_read).unwrap());

    let mut tag_for_read = ETag {
        name: String::from("test"),
        address: String::from("DB2D10"),
        datatype: ETagtype::DINT,
    };
    client.write_tag(&mut tag_for_read, ETagValue::Int(5842651));
    info!("{:#?}", client.read_tag(&tag_for_read).unwrap());

    // loop {
    //     info!("{:#?}", client.read(2, 0, 20));
    //     // if let Ok(result) = client.read(1, 0, 20) {
    //     //     let num = u32::from_
    //     // }
    //     let buf = [0, 20];
    //     let num = u16::from_be_bytes(buf);
    //     info!("{}", num);
    //     std::thread::sleep(std::time::Duration::from_secs(1));
    // }
}
