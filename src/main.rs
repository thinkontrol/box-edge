extern crate chrono;
extern crate clap;
extern crate env_logger;
extern crate futures;
extern crate log;
extern crate regex;
extern crate snap7_sys;

use chrono::Local;
use clap::{App, Arg};
use env_logger::Builder;
use futures::future::{ok, Either};
use futures::{Future, Stream};
use log::{error, info, warn, LevelFilter};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::time::Duration;
use std::{env, process};

use snap7_sys::*;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};

#[derive(Debug)]
struct Client {
    handle: S7Object,
    req_len: usize,
    neg_len: usize,
}

impl Client {
    pub fn new() -> Self {
        Self {
            handle: unsafe { Cli_Create() },
            req_len: 0,
            neg_len: 0,
        }
    }

    pub fn connect(&mut self) {
        let mut req: c_int = 0;
        let mut neg: c_int = 0;

        unsafe {
            Cli_ConnectTo(
                self.handle,
                CString::new("10.0.0.230").unwrap().as_ptr(),
                0,
                1,
            );

            Cli_GetPduLength(self.handle, &mut req, &mut neg);

            self.req_len = req as usize;
            self.neg_len = neg as usize;

            info!("Get PDU: {}, {}", self.req_len, self.neg_len)
        }
    }

    pub fn read(&self, num: u32, start: u32, size: u32) -> Result<Vec<u8>, String> {
        let mut buf = Vec::<u8>::new();

        buf.resize(size as usize, 0);

        let res;
        unsafe {
            res = Cli_DBRead(
                self.handle,
                num as c_int,
                start as c_int,
                size as c_int,
                buf.as_mut_ptr() as *mut c_void,
            ) as i32;
        }

        if res == 0 {
            Ok(buf)
        } else {
            Err(String::from(error_text(res)))
        }
    }

    pub fn close(&mut self) {
        unsafe {
            Cli_Disconnect(self.handle);
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.close();

        unsafe {
            Cli_Destroy(&mut self.handle);
        }
    }
}

pub fn error_text(code: i32) -> String {
    let mut err = Vec::<u8>::new();

    err.resize(1024, 0);

    unsafe {
        Cli_ErrorText(
            code as c_int,
            err.as_mut_ptr() as *mut c_char,
            err.len() as c_int,
        );
    }

    if let Some(i) = err.iter().position(|&r| r == 0) {
        err.truncate(i);
    }

    let err = unsafe { std::str::from_utf8_unchecked(&err) };

    err.to_owned()
}

struct CtlRecord {
    plc_counter: u64,
    ctl_counter: u64,
}

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

    client.connect();

    warn!("{:#?}", client);

    loop {
        info!("{:#?}", client.read(2, 0, 20));
        // if let Ok(result) = client.read(1, 0, 20) {
        //     let num = u32::from_
        // }
        let buf = [0, 64];
        let num = u16::from_be_bytes(buf);
        info!("{}", num);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
