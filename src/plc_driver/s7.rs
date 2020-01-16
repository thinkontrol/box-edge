use super::{ETag, ETagRW, ETagValue, ETagtype};
use bit_vec::BitVec;
use itertools::Itertools;
use log::{error, info, warn, LevelFilter};
use regex::Regex;
use snap7_sys::*;
use std::cmp::Ordering;
use std::convert::TryInto;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};
use url::Url;

#[derive(Debug)]
pub struct Client {
    handle: S7Object,
    req_len: usize,
    neg_len: usize,
    reg: regex::Regex,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct S7Address {
    area: S7Area,
    dbnb: u8,
    bit: u8,
    start: u8,
    size: u8,
    datatype: ETagtype,
}

// impl Ord for S7Address {
//     fn cmp(&self, other: &Self) -> Ordering {
//         if (self.area as i32) - (other.area as i32) > 0 {
//             Ordering::Greater
//         } else if (self.area as i32) - (other.area as i32) < 0 {
//             Ordering::Less
//         } else {
//             Ordering::Equal
//         }
//     }
// }

// impl PartialOrd for S7Address {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl PartialEq for S7Address {
//     fn eq(&self, other: &Self) -> bool {
//         self.area == other.area &&
//         self.dbnb == other.dbnb &&
//         self.bit == other.bit
//     }
// }

impl Client {
    pub fn new() -> Self {
        Self {
            handle: unsafe { Cli_Create() },
            req_len: 0,
            neg_len: 0,
            reg: Regex::new(r"^(M|I|Q|(?:DB(\d+)))(W|D|X)(\d+)(?:\.([0-7]))?$").unwrap(),
        }
    }

    // pub fn setTimeOut(&mut self, timeout: i32) {
    //     unsafe {
    //         Cli_SetParam(
    //             self.handle,

    //         );
    //     }
    // }

    pub fn connect(&mut self, host: &str, rack: i32, slot: i32) {
        let mut req: c_int = 0;
        let mut neg: c_int = 0;
        let mut buf = Vec::<u8>::new();
        buf.resize(4, 0);
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

            Cli_GetParam(self.handle, 5 as c_int, buf.as_mut_ptr() as *mut c_void);
            info!(
                "Ping timeout: {:#?}",
                i32::from_le_bytes(buf[0..4].try_into().unwrap())
            );
        }
    }

    pub fn close(&mut self) {
        unsafe {
            Cli_Disconnect(self.handle);
        }
    }

    pub fn connected(&mut self) -> bool {
        let mut r: c_int = 0;
        let res;
        unsafe {
            res = Cli_GetConnected(self.handle, &mut r) as i32;
        }
        info!("Connect Status: {}, {}", res, r);
        res == 0 && r == 1
    }

    fn conv_value(&self, buf: &Vec<u8>, datatype: &ETagtype, bit: u8) -> Result<ETagValue, String> {
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
        write: ETagValue,
        addr: &S7Address,
        prefetch_bool_byte: bool,
    ) -> Result<Vec<u8>, String> {
        match addr.datatype {
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
                    if prefetch_bool_byte {
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
                        Ok(buf)
                    }
                } else {
                    Err(String::from("Invalid datatype for write value"))
                }
            }
        }
    }

    pub fn conv_address(&self, address: &str, datatype: ETagtype) -> Result<S7Address, String> {
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
                datatype,
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

    fn get_s7data_item(&self, addr: &S7Address, buf: &mut Vec<u8>) -> TS7DataItem {
        TS7DataItem {
            Area: addr.area as c_int,
            WordLen: S7WL::S7WLByte as c_int,
            Result: 0 as c_int,
            DBNumber: addr.dbnb as c_int,
            Start: addr.start as c_int,
            Amount: addr.size as c_int,
            pdata: buf.as_mut_ptr() as *mut c_void,
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
        match self.conv_address(tag.address.as_str(), tag.datatype) {
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
                    self.conv_value(&buf, &tag.datatype, addr.bit)
                } else {
                    Err(String::from(error_text(res)))
                }
            }
            Err(err) => Err(err),
        }
    }
    fn read_list(&self, tags: &Vec<ETag>) -> Result<Vec<Result<ETagValue, String>>, String> {
        let addrs: Vec<_> = tags
            .iter()
            .map(|tag| self.conv_address(tag.address.as_str(), tag.datatype))
            .collect();
        if addrs.iter().any(|addr| addr.is_err()) {
            Err(String::from("Address error"))
        } else {
            let items: Vec<_> = addrs
                .iter()
                .map(|addr| {
                    let addr_ = addr.as_ref().unwrap();
                    let mut buf = Vec::<u8>::new();
                    buf.resize(addr_.size as usize, 0);
                    (self.get_s7data_item(addr_, &mut buf), buf, addr_)
                })
                .collect();
            let mut ts7_items: Vec<TS7DataItem> = items.iter().map(|t| t.0).collect();
            // let res;
            // unsafe {
            //     res = Cli_ReadMultiVars(self.handle, &mut ts7_items[0], ts7_items.len() as c_int)
            //         as i32;
            // }
            // if res == 0 {
            //     let results: Vec<_> = items
            //         .iter()
            //         .map(|t| {
            //             let p = t.0;
            //             if p.Result == 0 {
            //                 self.conv_value(&t.1, &t.2.datatype, t.2.bit)
            //             } else {
            //                 Err(String::from(error_text(res)))
            //             }
            //         })
            //         .collect();
            //     Ok(results)
            // } else {
            //     Err(String::from(error_text(res)))
            // }
            let cli_results: Vec<_> = ts7_items
                .chunks_mut(20)
                .map(|chunk| {
                    let res;
                    unsafe {
                        res = Cli_ReadMultiVars(self.handle, &mut chunk[0], chunk.len() as c_int)
                            as i32;
                    }
                    if res == 0 {
                        Ok(())
                    } else {
                        Err(res)
                    }
                })
                .collect();
            match cli_results.into_iter().find(|cli_r| cli_r.is_err()) {
                Some(Err(res)) => Err(String::from(error_text(res))),
                _ => {
                    let results: Vec<_> = items
                        .iter()
                        .map(|t| {
                            let p = t.0;
                            if p.Result == 0 {
                                self.conv_value(&t.1, &t.2.datatype, t.2.bit)
                            } else {
                                Err(String::from(error_text(p.Result)))
                            }
                        })
                        .collect();
                    Ok(results)
                }
            }
        }
    }
    fn write_tag(&self, tag: &ETag, write: ETagValue) -> Result<bool, String> {
        match self.conv_address(tag.address.as_str(), tag.datatype) {
            Ok(addr) => match self.conv_buf(write, &addr, true) {
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
    fn write_list(
        &self,
        tags: &Vec<(ETag, ETagValue)>,
    ) -> Result<Vec<Result<bool, String>>, String> {
        let addrs: Vec<_> = tags
            .iter()
            .map(|t| self.conv_address(t.0.address.as_str(), t.0.datatype))
            .collect();
        if addrs.iter().any(|addr| addr.is_err()) {
            Err(String::from("Address error"))
        } else {
            let addrs: Vec<_> = tags
                .iter()
                .map(|t| {
                    self.conv_address(t.0.address.as_str(), t.0.datatype)
                        .unwrap()
                })
                .collect();
            let mut items: Vec<_> = addrs
                .iter()
                .enumerate()
                .map(|(i, addr)| {
                    let mut buf_ = Vec::<u8>::new();
                    buf_.resize(addr.size as usize, 0);
                    let mut buf = self.conv_buf(tags[i].1, addr, false).unwrap_or(buf_);
                    (self.get_s7data_item(addr, &mut buf), buf)
                })
                .collect();
            let addrs: Vec<_> = tags
                .iter()
                .map(|t| {
                    self.conv_address(t.0.address.as_str(), t.0.datatype)
                        .unwrap()
                })
                .collect();
            for (area_key, area_group) in &addrs
                .iter()
                .filter(|addr| addr.datatype.is_bool())
                .sorted_by(|a, b| Ord::cmp(a, b))
                .group_by(|t| t.area)
            {
                for (dbnb_key, dbnb_group) in &area_group.into_iter().group_by(|t| t.dbnb) {
                    for (start_key, start_group) in &dbnb_group.into_iter().group_by(|t| t.start) {
                        let mut buf = Vec::<u8>::new();
                        buf.resize(1, 0);
                        let res;
                        unsafe {
                            res = Cli_ReadArea(
                                self.handle,
                                area_key as c_int,
                                dbnb_key as c_int,
                                start_key as c_int,
                                1 as c_int,
                                S7WL::S7WLByte as c_int,
                                buf.as_mut_ptr() as *mut c_void,
                            ) as i32;
                        }
                        if res == 0 {
                            let mut bv = BitVec::from_bytes(&buf);
                            let start_items: Vec<_> = start_group.into_iter().collect();
                            for v in &start_items {
                                let index = addrs.iter().position(|r| r == *v).unwrap();
                                if let ETagValue::Bool(b) = tags[index].1 {
                                    bv.set((7 - v.bit) as usize, b);
                                }
                            }
                            for v in &start_items {
                                let index = addrs.iter().position(|r| r == *v).unwrap();
                                items[index].1[0] = bv.to_bytes()[0];
                            }
                        } else {
                            return Err(String::from(error_text(res)));
                        }
                    }
                }
            }
            let mut ts7_items: Vec<TS7DataItem> = items.iter().map(|t| t.0).collect();
            // let res;
            // unsafe {
            //     res = Cli_WriteMultiVars(self.handle, &mut ts7_items[0], ts7_items.len() as c_int)
            //         as i32;
            // }
            // if res == 0 {
            //     let results: Vec<_> = items
            //         .iter()
            //         .map(|t| {
            //             let p = t.0;
            //             if p.Result == 0 {
            //                 Ok(true)
            //             } else {
            //                 Err(String::from(error_text(res)))
            //             }
            //         })
            //         .collect();
            //     Ok(results)
            // } else {
            //     Err(String::from(error_text(res)))
            // }
            let cli_results: Vec<_> = ts7_items
                .chunks_mut(20)
                .map(|chunk| {
                    let res;
                    unsafe {
                        res = Cli_WriteMultiVars(self.handle, &mut chunk[0], chunk.len() as c_int)
                            as i32;
                    }
                    if res == 0 {
                        Ok(())
                    } else {
                        Err(res)
                    }
                })
                .collect();
            match cli_results.into_iter().find(|cli_r| cli_r.is_err()) {
                Some(Err(res)) => Err(String::from(error_text(res))),
                _ => {
                    let results: Vec<_> = items
                        .iter()
                        .map(|t| {
                            let p = t.0;
                            if p.Result == 0 {
                                Ok(true)
                            } else {
                                Err(String::from(error_text(p.Result)))
                            }
                        })
                        .collect();
                    Ok(results)
                }
            }
        }
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
