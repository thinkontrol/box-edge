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

    let buf = client.read(2, 0, 16).unwrap();

    let (int_bytes, rest) = buf.split_at(std::mem::size_of::<i16>());

    let a = i16::from_be_bytes(buf[0..2].try_into().unwrap());
    info!("{:#?}, {:#?}", a, int_bytes);

    let (int_bytes, rest) = rest.split_at(std::mem::size_of::<i16>());
    let a = i16::from_be_bytes(buf[2..4].try_into().unwrap());
    info!("{:#?}, {:#?}", a, int_bytes);

    let (int_bytes, rest) = rest.split_at(std::mem::size_of::<f32>());
    let a = f32::from_bits(u32::from_be_bytes(buf[4..8].try_into().unwrap()));
    info!("{:#?}, {:#?}", a, buf[8]);

    info!("{:#?}", rest);

    let bv = BitVec::from_bytes(&buf[8..9]);
    info!("{:#?}", bv.get(0));
    info!("{:#?}", bv.get(1));
    info!("{:#?}", bv.get(2));
    info!("{:#?}", bv.get(3));
    info!("{:#?}", bv.get(4));
    info!("{:#?}", bv.get(5));
    info!("{:#?}", bv.get(6));
    info!("{:#?}", bv.get(7));

    let bv = BitVec::from_bytes(&buf[9..10]);
    info!("{:#?}", bv.get(0));
    info!("{:#?}", bv.get(1));
    info!("{:#?}", bv.get(2));
    info!("{:#?}", bv.get(3));
    info!("{:#?}", bv.get(4));
    info!("{:#?}", bv.get(5));
    info!("{:#?}", bv.get(6));
    info!("{:#?}", bv.get(7));
    info!("{:#?}", bv.get(8));

    let mut tag_for_read = ETag {
        name: String::from("test"),
        address: String::from("DB2W2"),
        datatype: ETagtype::INT,
        read: Err(String::from("")),
        write: None,
    };
    match client.read_tag(&mut tag_for_read) {
        Ok(_) => info!("{:#?}", &tag_for_read.read.unwrap()),
        Err(msg) => error!("{:#?}", msg),
    }

    let mut tag_for_read = ETag {
        name: String::from("test"),
        address: String::from("DB2D4"),
        datatype: ETagtype::REAL,
        read: Err(String::from("")),
        write: None,
    };
    match client.read_tag(&mut tag_for_read) {
        Ok(_) => info!("{:#?}", &tag_for_read.read.unwrap()),
        Err(msg) => error!("{:#?}", msg),
    }

    let mut tag_for_read = ETag {
        name: String::from("test"),
        address: String::from("DB2X9.0"),
        datatype: ETagtype::BOOL,
        read: Err(String::from("")),
        write: None,
    };
    match client.read_tag(&mut tag_for_read) {
        Ok(_) => info!(
            "{:#?}: {:#?}",
            &tag_for_read.address,
            &tag_for_read.read.unwrap()
        ),
        Err(msg) => error!("{:#?}", msg),
    }

    let mut tag_for_read = ETag {
        name: String::from("test"),
        address: String::from("DB2X9.1"),
        datatype: ETagtype::BOOL,
        read: Err(String::from("")),
        write: None,
    };
    match client.read_tag(&mut tag_for_read) {
        Ok(_) => info!(
            "{:#?}: {:#?}",
            &tag_for_read.address,
            &tag_for_read.read.unwrap()
        ),
        Err(msg) => error!("{:#?}", msg),
    }

    let mut tag_for_read = ETag {
        name: String::from("test"),
        address: String::from("DB2X9.2"),
        datatype: ETagtype::BOOL,
        read: Err(String::from("")),
        write: None,
    };
    match client.read_tag(&mut tag_for_read) {
        Ok(_) => info!(
            "{:#?}: {:#?}",
            &tag_for_read.address,
            &tag_for_read.read.unwrap()
        ),
        Err(msg) => error!("{:#?}", msg),
    }

    let mut tag_for_read = ETag {
        name: String::from("test"),
        address: String::from("DB2X9.3"),
        datatype: ETagtype::BOOL,
        read: Err(String::from("")),
        write: None,
    };
    match client.read_tag(&mut tag_for_read) {
        Ok(_) => info!(
            "{:#?}: {:#?}",
            &tag_for_read.address,
            &tag_for_read.read.unwrap()
        ),
        Err(msg) => error!("{:#?}", msg),
    }

    let mut tag_for_read = ETag {
        name: String::from("test"),
        address: String::from("DB2X9.4"),
        datatype: ETagtype::BOOL,
        read: Err(String::from("")),
        write: None,
    };
    match client.read_tag(&mut tag_for_read) {
        Ok(_) => info!(
            "{:#?}: {:#?}",
            &tag_for_read.address,
            &tag_for_read.read.unwrap()
        ),
        Err(msg) => error!("{:#?}", msg),
    }

    let mut tag_for_read = ETag {
        name: String::from("test"),
        address: String::from("DB2X9.5"),
        datatype: ETagtype::BOOL,
        read: Err(String::from("")),
        write: None,
    };
    match client.read_tag(&mut tag_for_read) {
        Ok(_) => info!(
            "{:#?}: {:#?}",
            &tag_for_read.address,
            &tag_for_read.read.unwrap()
        ),
        Err(msg) => error!("{:#?}", msg),
    }

    let mut tag_for_read = ETag {
        name: String::from("test"),
        address: String::from("DB2X9.6"),
        datatype: ETagtype::BOOL,
        read: Err(String::from("")),
        write: None,
    };
    match client.read_tag(&mut tag_for_read) {
        Ok(_) => info!(
            "{:#?}: {:#?}",
            &tag_for_read.address,
            &tag_for_read.read.unwrap()
        ),
        Err(msg) => error!("{:#?}", msg),
    }

    let mut tag_for_read = ETag {
        name: String::from("test"),
        address: String::from("DB2X9.7"),
        datatype: ETagtype::BOOL,
        read: Err(String::from("")),
        write: None,
    };
    match client.read_tag(&mut tag_for_read) {
        Ok(_) => info!(
            "{:#?}: {:#?}",
            &tag_for_read.address,
            &tag_for_read.read.unwrap()
        ),
        Err(msg) => error!("{:#?}", msg),
    }

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
