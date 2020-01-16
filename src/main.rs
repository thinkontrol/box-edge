mod plc_driver;

extern crate bit_vec;
extern crate chrono;
extern crate clap;
extern crate env_logger;
extern crate futures;
extern crate itertools;
extern crate log;
extern crate regex;
extern crate snap7_sys;
extern crate url;

use bit_vec::BitVec;
use chrono::Local;
use env_logger::Builder;
use futures::future::{ok, Either};
use futures::{Future, Stream};
use log::{error, info, warn, LevelFilter};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::convert::TryInto;
use std::io::Write;
use std::time::Duration;
use std::{env, process};

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
    info!("Connected: {}", client.connected());

    client.connect("10.0.0.230", 0, 1);

    info!("Connected: {}", client.connected());

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

    let tags = vec![
        ETag {
            name: String::from("test"),
            address: String::from("DB2W0"),
            datatype: ETagtype::INT,
        },
        ETag {
            name: String::from("test"),
            address: String::from("DB2W2"),
            datatype: ETagtype::INT,
        },
        ETag {
            name: String::from("test"),
            address: String::from("DB2D4"),
            datatype: ETagtype::REAL,
        },
        ETag {
            name: String::from("test"),
            address: String::from("DB2X9.0"),
            datatype: ETagtype::BOOL,
        },
        ETag {
            name: String::from("test"),
            address: String::from("DB2X9.1"),
            datatype: ETagtype::BOOL,
        },
        ETag {
            name: String::from("test"),
            address: String::from("DB2X9.2"),
            datatype: ETagtype::BOOL,
        },
        ETag {
            name: String::from("test"),
            address: String::from("DB2X9.3"),
            datatype: ETagtype::BOOL,
        },
        ETag {
            name: String::from("test"),
            address: String::from("DB2X9.4"),
            datatype: ETagtype::BOOL,
        },
        ETag {
            name: String::from("test"),
            address: String::from("DB2X9.5"),
            datatype: ETagtype::BOOL,
        },
        ETag {
            name: String::from("test"),
            address: String::from("DB2X9.6"),
            datatype: ETagtype::BOOL,
        },
        ETag {
            name: String::from("test"),
            address: String::from("DB2X9.7"),
            datatype: ETagtype::BOOL,
        },
        ETag {
            name: String::from("test"),
            address: String::from("MW104"),
            datatype: ETagtype::INT,
        },
        ETag {
            name: String::from("test"),
            address: String::from("MW102"),
            datatype: ETagtype::INT,
        },
        ETag {
            name: String::from("test"),
            address: String::from("MD110"),
            datatype: ETagtype::REAL,
        },
        ETag {
            name: String::from("test"),
            address: String::from("MX100.7"),
            datatype: ETagtype::BOOL,
        },
        ETag {
            name: String::from("test"),
            address: String::from("MX100.6"),
            datatype: ETagtype::BOOL,
        },
        ETag {
            name: String::from("test"),
            address: String::from("MX100.5"),
            datatype: ETagtype::BOOL,
        },
        ETag {
            name: String::from("test"),
            address: String::from("MX100.4"),
            datatype: ETagtype::BOOL,
        },
        ETag {
            name: String::from("test"),
            address: String::from("MX100.3"),
            datatype: ETagtype::BOOL,
        },
        ETag {
            name: String::from("test"),
            address: String::from("MX100.2"),
            datatype: ETagtype::BOOL,
        },
        ETag {
            name: String::from("test"),
            address: String::from("MX100.1"),
            datatype: ETagtype::BOOL,
        },
        ETag {
            name: String::from("test"),
            address: String::from("MX100.0"),
            datatype: ETagtype::BOOL,
        },
    ];

    let results = client.read_list(&tags).unwrap();
    for r in results {
        info!("{:#?}", r.unwrap())
    }

    // let tags = vec![
    //     (
    //         ETag {
    //             name: String::from("test"),
    //             address: String::from("DB2X9.7"),
    //             datatype: ETagtype::BOOL,
    //         },
    //         ETagValue::Bool(true),
    //     ),
    //     (
    //         ETag {
    //             name: String::from("test"),
    //             address: String::from("DB2W0"),
    //             datatype: ETagtype::INT,
    //         },
    //         ETagValue::Int(546),
    //     ),
    //     (
    //         ETag {
    //             name: String::from("test"),
    //             address: String::from("DB2X9.0"),
    //             datatype: ETagtype::BOOL,
    //         },
    //         ETagValue::Bool(false),
    //     ),
    //     (
    //         ETag {
    //             name: String::from("test"),
    //             address: String::from("DB2X9.4"),
    //             datatype: ETagtype::BOOL,
    //         },
    //         ETagValue::Bool(false),
    //     ),
    //     (
    //         ETag {
    //             name: String::from("test"),
    //             address: String::from("DB2X9.5"),
    //             datatype: ETagtype::BOOL,
    //         },
    //         ETagValue::Bool(false),
    //     ),
    //     (
    //         ETag {
    //             name: String::from("test"),
    //             address: String::from("DB2W2"),
    //             datatype: ETagtype::INT,
    //         },
    //         ETagValue::Int(854),
    //     ),
    //     (
    //         ETag {
    //             name: String::from("test"),
    //             address: String::from("DB2D4"),
    //             datatype: ETagtype::REAL,
    //         },
    //         ETagValue::Real(856.32),
    //     ),
    //     (
    //         ETag {
    //             name: String::from("test"),
    //             address: String::from("DB2X9.1"),
    //             datatype: ETagtype::BOOL,
    //         },
    //         ETagValue::Bool(true),
    //     ),
    //     (
    //         ETag {
    //             name: String::from("test"),
    //             address: String::from("DB2X9.2"),
    //             datatype: ETagtype::BOOL,
    //         },
    //         ETagValue::Bool(true),
    //     ),
    //     (
    //         ETag {
    //             name: String::from("test"),
    //             address: String::from("MW100"),
    //             datatype: ETagtype::INT,
    //         },
    //         ETagValue::Int(3405),
    //     ),
    //     (
    //         ETag {
    //             name: String::from("test"),
    //             address: String::from("MD102"),
    //             datatype: ETagtype::DINT,
    //         },
    //         ETagValue::Int(96646598),
    //     ),
    //     (
    //         ETag {
    //             name: String::from("test"),
    //             address: String::from("MD106"),
    //             datatype: ETagtype::REAL,
    //         },
    //         ETagValue::Real(0.002),
    //     ),
    //     (
    //         ETag {
    //             name: String::from("test"),
    //             address: String::from("MX10.5"),
    //             datatype: ETagtype::BOOL,
    //         },
    //         ETagValue::Bool(true),
    //     ),
    //     (
    //         ETag {
    //             name: String::from("test"),
    //             address: String::from("MX10.2"),
    //             datatype: ETagtype::BOOL,
    //         },
    //         ETagValue::Bool(true),
    //     ),
    //     (
    //         ETag {
    //             name: String::from("test"),
    //             address: String::from("IX0.0"),
    //             datatype: ETagtype::BOOL,
    //         },
    //         ETagValue::Bool(false),
    //     ),
    //     (
    //         ETag {
    //             name: String::from("test"),
    //             address: String::from("QX0.0"),
    //             datatype: ETagtype::BOOL,
    //         },
    //         ETagValue::Bool(false),
    //     ),
    // ];

    // client.write_list(&tags);

    // let tags = vec![
    //     ETag {
    //         name: String::from("test"),
    //         address: String::from("DB2W0"),
    //         datatype: ETagtype::INT,
    //     },
    //     ETag {
    //         name: String::from("test"),
    //         address: String::from("DB2W2"),
    //         datatype: ETagtype::INT,
    //     },
    //     ETag {
    //         name: String::from("test"),
    //         address: String::from("DB2D4"),
    //         datatype: ETagtype::REAL,
    //     },
    //     ETag {
    //         name: String::from("test"),
    //         address: String::from("DB2X9.0"),
    //         datatype: ETagtype::BOOL,
    //     },
    //     ETag {
    //         name: String::from("test"),
    //         address: String::from("DB2X9.1"),
    //         datatype: ETagtype::BOOL,
    //     },
    //     ETag {
    //         name: String::from("test"),
    //         address: String::from("DB2X9.2"),
    //         datatype: ETagtype::BOOL,
    //     },
    //     ETag {
    //         name: String::from("test"),
    //         address: String::from("DB2X9.3"),
    //         datatype: ETagtype::BOOL,
    //     },
    //     ETag {
    //         name: String::from("test"),
    //         address: String::from("DB2X9.4"),
    //         datatype: ETagtype::BOOL,
    //     },
    //     ETag {
    //         name: String::from("test"),
    //         address: String::from("DB2X9.5"),
    //         datatype: ETagtype::BOOL,
    //     },
    //     ETag {
    //         name: String::from("test"),
    //         address: String::from("DB2X9.6"),
    //         datatype: ETagtype::BOOL,
    //     },
    //     ETag {
    //         name: String::from("test"),
    //         address: String::from("DB2X9.7"),
    //         datatype: ETagtype::BOOL,
    //     },
    //     ETag {
    //         name: String::from("test"),
    //         address: String::from("MW100"),
    //         datatype: ETagtype::INT,
    //     },
    //     ETag {
    //         name: String::from("test"),
    //         address: String::from("MD102"),
    //         datatype: ETagtype::DINT,
    //     },
    //     ETag {
    //         name: String::from("test"),
    //         address: String::from("MD106"),
    //         datatype: ETagtype::REAL,
    //     },
    //     ETag {
    //         name: String::from("test"),
    //         address: String::from("MX10.5"),
    //         datatype: ETagtype::BOOL,
    //     },
    //     ETag {
    //         name: String::from("test"),
    //         address: String::from("MX10.2"),
    //         datatype: ETagtype::BOOL,
    //     },
    //     ETag {
    //         name: String::from("test"),
    //         address: String::from("IX0.0"),
    //         datatype: ETagtype::BOOL,
    //     },
    //     ETag {
    //         name: String::from("test"),
    //         address: String::from("QX0.0"),
    //         datatype: ETagtype::BOOL,
    //     },
    // ];

    // match client.read_list(&tags) {
    //     Ok(results) => {
    //         for r in results {
    //             info!("{:#?}", r.unwrap())
    //         }
    //     }
    //     Err(err) => info!("{:#?}", err),
    // };

    info!("Connected: {}", client.connected());
    client.close();
    info!("Connected: {}", client.connected());

    let tag = ETag {
        name: String::from("test"),
        address: String::from("DB2X9.1"),
        datatype: ETagtype::BOOL,
    };

    let value = ETagValue::Real(3.5);

    let pair = (
        ETag {
            name: String::from("test"),
            address: String::from("DB2X9.1"),
            datatype: ETagtype::BOOL,
        },
        ETagValue::Real(3.5),
    );

    match serde_json::to_string(&tag) {
        Ok(v) => info!("{}", v),
        Err(e) => error!("{}", e),
    };

    match serde_json::to_string(&value) {
        Ok(v) => info!("{}", v),
        Err(e) => error!("{}", e),
    };

    match serde_json::to_string(&pair) {
        Ok(v) => info!("{}", v),
        Err(e) => error!("{}", e),
    };

    let tag_str = r#"{"name":"test","address":"DB2X9.1","datatype":"BOOL"}"#;
    let value_str = r#"{"Real":3.5}"#;
    let pair_str = r#"[{"name":"test","address":"DB2X9.1","datatype":"BOOL"},{"Bool":true}]"#;

    let tag: Result<ETag> = serde_json::from_str(tag_str);
    match tag {
        Ok(v) => info!("{:#?}", v),
        Err(e) => error!("{}", e),
    };

    let value: Result<ETagValue> = serde_json::from_str(value_str);
    match value {
        Ok(v) => info!("{:#?}", v),
        Err(e) => error!("{}", e),
    };

    let pair: Result<(ETag, ETagValue)> = serde_json::from_str(pair_str);
    match pair {
        Ok(v) => info!("{:#?}", v),
        Err(e) => error!("{}", e),
    };

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
