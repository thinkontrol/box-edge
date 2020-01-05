use super::{ETag, ETagRW, ETagValue, ETagtype};
use bit_vec::BitVec;
use log::{error, info, warn, LevelFilter};
use regex::Regex;
use snap7_sys::*;
use std::convert::TryInto;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};

#[derive(Debug)]
pub struct Client {
    handle: S7Object,
    req_len: usize,
    neg_len: usize,
    reg: regex::Regex,
}

#[derive(Debug, Copy, Clone)]
pub enum S7Area {
    PE = 0x81,
    PA = 0x82,
    MK = 0x83,
    DB = 0x84,
}

#[derive(Debug)]
pub enum S7WL {
    S7WLBit = 0x01,
    S7WLByte = 0x02,
    S7WLWord = 0x04,
    S7WLDWord = 0x06,
    S7WLReal = 0x08,
}

#[derive(Debug, Copy, Clone)]
pub struct S7Address {
    area: S7Area,
    dbnb: u8,
    bit: u8,
    start: u8,
    size: u8,
}

impl Client {
    pub fn new() -> Self {
        Self {
            handle: unsafe { Cli_Create() },
            req_len: 0,
            neg_len: 0,
            reg: Regex::new(r"^(M|(?:DB(\d+)))(W|D|X)(\d+)(?:\.([0-7]))?$").unwrap(),
        }
    }

    pub fn connect(&mut self, host: &str, rack: i32, slot: i32) {
        let mut req: c_int = 0;
        let mut neg: c_int = 0;

        unsafe {
            Cli_ConnectTo(
                self.handle,
                CString::new(host).unwrap().as_ptr(),
                rack,
                slot,
            );

            Cli_GetPduLength(self.handle, &mut req, &mut neg);

            self.req_len = req as usize;
            self.neg_len = neg as usize;

            info!("Get PDU: {}, {}", self.req_len, self.neg_len);
        }
    }

    pub fn close(&mut self) {
        unsafe {
            Cli_Disconnect(self.handle);
        }
    }

    fn conv_value(&self, buf: Vec<u8>, datatype: &ETagtype, bit: u8) -> Result<ETagValue, String> {
        match datatype {
            ETagtype::INT => Ok(ETagValue::Int(
                i16::from_be_bytes(buf[0..2].try_into().unwrap()) as i64,
            )),

            ETagtype::DINT => Ok(ETagValue::Int(
                i32::from_be_bytes(buf[0..4].try_into().unwrap()) as i64,
            )),

            ETagtype::REAL => Ok(ETagValue::Real(f32::from_bits(u32::from_be_bytes(
                buf[0..4].try_into().unwrap(),
            )) as f64)),
            ETagtype::BOOL => {
                let bv = BitVec::from_bytes(&buf);
                Ok(ETagValue::Bool(bv.get((7 - bit) as usize).unwrap()))
            }
        }
    }

    fn conv_buf(
        &self,
        datatype: &ETagtype,
        write: ETagValue,
        addr: &S7Address,
    ) -> Result<Vec<u8>, String> {
        match datatype {
            ETagtype::INT => {
                if let ETagValue::Int(v) = write {
                    let bytes = (v as i16).to_be_bytes();
                    Ok(vec![bytes[0], bytes[1]])
                } else {
                    Err(String::from("Invalid datatype for write value"))
                }
            }
            ETagtype::DINT => {
                if let ETagValue::Int(v) = write {
                    let bytes = (v as i32).to_be_bytes();
                    Ok(vec![bytes[0], bytes[1], bytes[2], bytes[3]])
                } else {
                    Err(String::from("Invalid datatype for write value"))
                }
            }
            ETagtype::REAL => {
                if let ETagValue::Real(v) = write {
                    let bytes = ((v as f32).to_bits() as u32).to_be_bytes();
                    Ok(vec![bytes[0], bytes[1], bytes[2], bytes[3]])
                } else {
                    Err(String::from("Invalid datatype for write value"))
                }
            }
            ETagtype::BOOL => {
                if let ETagValue::Bool(v) = write {
                    let mut buf = Vec::<u8>::new();
                    buf.resize(addr.size as usize, 0);
                    let res;
                    unsafe {
                        res = Cli_ReadArea(
                            self.handle,
                            addr.area as c_int,
                            addr.dbnb as c_int,
                            addr.start as c_int,
                            addr.size as c_int,
                            S7WL::S7WLByte as c_int,
                            buf.as_mut_ptr() as *mut c_void,
                        ) as i32;
                    }

                    if res == 0 {
                        let mut bv = BitVec::from_bytes(&buf);
                        bv.set((7 - addr.bit) as usize, v);
                        Ok(bv.to_bytes())
                    } else {
                        Err(String::from(error_text(res)))
                    }
                } else {
                    Err(String::from("Invalid datatype for write value"))
                }
            }
        }
    }

    pub fn conv_address(&self, address: &str, datatype: &ETagtype) -> Result<S7Address, String> {
        if let Some(r) = &self.reg.captures(address) {
            let area: S7Area = match r.get(1).unwrap().as_str() {
                "M" => S7Area::MK,
                "I" => S7Area::PE,
                "Q" => S7Area::PA,
                _ => S7Area::DB,
            };
            let dbnb: u8 = match area {
                S7Area::DB => r.get(2).unwrap().as_str().parse().unwrap(),
                _ => 0,
            };
            let dd = r.get(3).unwrap().as_str();
            let size: u8 = match dd {
                "W" => 2,
                "D" => 4,
                _ => 1,
            };
            let start: u8 = r.get(4).unwrap().as_str().parse().unwrap();
            let bit: u8 = if r.get(5).is_none() {
                0
            } else {
                r.get(5).unwrap().as_str().parse().unwrap()
            };
            let addr = S7Address {
                area,
                dbnb,
                size,
                start,
                bit,
            };
            match datatype {
                ETagtype::BOOL if dd == "X" => Ok(addr),
                ETagtype::INT if dd == "W" => Ok(addr),
                _ if dd == "D" => Ok(addr),
                _ => Err(String::from("Invalid S7 addree")),
            }
        } else {
            Err(String::from("Invalid S7 addree"))
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

impl ETagRW for Client {
    fn read_tag(&self, tag: &ETag) -> Result<ETagValue, String> {
        match self.conv_address(tag.address.as_str(), &tag.datatype) {
            Ok(addr) => {
                let mut buf = Vec::<u8>::new();
                buf.resize(addr.size as usize, 0);
                let res;
                unsafe {
                    res = Cli_ReadArea(
                        self.handle,
                        addr.area as c_int,
                        addr.dbnb as c_int,
                        addr.start as c_int,
                        addr.size as c_int,
                        S7WL::S7WLByte as c_int,
                        buf.as_mut_ptr() as *mut c_void,
                    ) as i32;
                }

                if res == 0 {
                    self.conv_value(buf, &tag.datatype, addr.bit)
                } else {
                    Err(String::from(error_text(res)))
                }
            }
            Err(err) => Err(err),
        }
    }
    fn read_list(&self, tags: &mut &[ETag]) -> Result<bool, String> {
        Ok(true)
    }
    fn write_tag(&self, tag: &ETag, write: ETagValue) -> Result<bool, String> {
        match self.conv_address(tag.address.as_str(), &tag.datatype) {
            Ok(addr) => match self.conv_buf(&tag.datatype, write, &addr) {
                Ok(buf) => {
                    let res;
                    unsafe {
                        res = Cli_WriteArea(
                            self.handle,
                            addr.area as c_int,
                            addr.dbnb as c_int,
                            addr.start as c_int,
                            addr.size as c_int,
                            S7WL::S7WLByte as c_int,
                            buf.as_ptr() as *mut c_void,
                        ) as i32;
                    }

                    if res == 0 {
                        Ok(true)
                    } else {
                        Err(String::from(error_text(res)))
                    }
                }
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }
    fn write_list(&self, tags: &[ETag]) -> Result<bool, String> {
        Ok(true)
    }
}

// struct CtlRecord {
//     plc_counter: u64,
//     ctl_counter: u64,
// }

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
