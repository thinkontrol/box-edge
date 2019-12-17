extern crate chrono;
extern crate clap;
extern crate env_logger;
extern crate futures;
extern crate libc;
extern crate log;
extern crate regex;

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

use libc::{c_int, c_void, Elf32_Word};

struct S7Object {}

#[link(name = "snap7")]
extern "C" {
    fn Cli_Create() -> *mut S7Object;
    fn Cli_Destroy(client: &S7Object);

    fn Cli_ConnectTo(clinet: *mut S7Object, address: *const u8, rack: c_int, slot: c_int) -> c_int;
    fn Cli_SetConnectionParams(
        client: *mut S7Object,
        address: *const u8,
        localTSAP: Elf32_Word,
        remoteTSAP: Elf32_Word,
    ) -> c_int;
    fn Cli_SetConnectionType(clinet: *mut S7Object, connectionType: Elf32_Word) -> c_int;
    fn Cli_Connect(clinet: *mut S7Object) -> c_int;
    fn Cli_Disconnect(clinet: *mut S7Object) -> c_int;
    fn Cli_GetParam(clinet: *mut S7Object, paramNumber: c_int, pValue: *mut c_void) -> c_int;
    fn Cli_SetParam(clinet: *mut S7Object, paramNumber: c_int, pValue: *mut c_void) -> c_int;
    // fn Cli_SetAsCallback(clinet: *mut S7Object, pfn_CliCompletion pCompletion, void *usrPtr) -> c_int;
    // Data I/O main functions
    fn Cli_ReadArea(
        clinet: *mut S7Object,
        area: c_int,
        dbNumber: c_int,
        start: c_int,
        amount: c_int,
        wordLen: c_int,
        pUsrData: *mut c_void,
    ) -> c_int;
    fn Cli_WriteArea(
        clinet: *mut S7Object,
        area: c_int,
        dbNumber: c_int,
        start: c_int,
        amount: c_int,
        wordLen: c_int,
        pUsrData: *mut c_void,
    ) -> c_int;
    // fn Cli_ReadMultiVars(clinet: *mut S7Object, item: *mut TS7DataItem, itemsCount: c_int) -> c_int;
    // fn Cli_WriteMultiVars(clinet: *mut S7Object, item: *mut TS7DataItem, itemsCount: c_int) -> c_int;
    // Data I/O Lean functions
    fn Cli_DBRead(
        clinet: *mut S7Object,
        dbNumber: c_int,
        start: c_int,
        size: c_int,
        pUsrData: *mut c_void,
    ) -> c_int;
    fn Cli_DBWrite(
        clinet: *mut S7Object,
        dbNumber: c_int,
        start: c_int,
        size: c_int,
        pUsrData: *mut c_void,
    ) -> c_int;
    fn Cli_MBRead(clinet: *mut S7Object, start: c_int, size: c_int, pUsrData: *mut c_void)
        -> c_int;
    fn Cli_MBWrite(
        clinet: *mut S7Object,
        start: c_int,
        size: c_int,
        pUsrData: *mut c_void,
    ) -> c_int;
    fn Cli_EBRead(clinet: *mut S7Object, start: c_int, size: c_int, pUsrData: *mut c_void)
        -> c_int;
    fn Cli_EBWrite(
        clinet: *mut S7Object,
        start: c_int,
        size: c_int,
        pUsrData: *mut c_void,
    ) -> c_int;
    fn Cli_ABRead(clinet: *mut S7Object, start: c_int, size: c_int, pUsrData: *mut c_void)
        -> c_int;
    fn Cli_ABWrite(
        clinet: *mut S7Object,
        start: c_int,
        size: c_int,
        pUsrData: *mut c_void,
    ) -> c_int;
    fn Cli_TMRead(
        clinet: *mut S7Object,
        start: c_int,
        amount: c_int,
        pUsrData: *mut c_void,
    ) -> c_int;
    fn Cli_TMWrite(
        clinet: *mut S7Object,
        start: c_int,
        amount: c_int,
        pUsrData: *mut c_void,
    ) -> c_int;
    fn Cli_CTRead(
        clinet: *mut S7Object,
        start: c_int,
        amount: c_int,
        pUsrData: *mut c_void,
    ) -> c_int;
    fn Cli_CTWrite(
        clinet: *mut S7Object,
        start: c_int,
        amount: c_int,
        pUsrData: *mut c_void,
    ) -> c_int;
    // Directory functions
    // fn Cli_ListBlocks(clinet: *mut S7Object, pUsrData: *mut TS7BlocksList) -> c_int;
    // fn Cli_GetAgBlockInfo(clinet: *mut S7Object, blockType: c_int, blockNum: c_int, TS7BlockInfo *pUsrData) -> c_int;
    // fn Cli_GetPgBlockInfo(clinet: *mut S7Object, pBlock: *mut c_void, TS7BlockInfo *pUsrData, size: c_int) -> c_int;
    // fn Cli_ListBlocksOfType(clinet: *mut S7Object, blockType: c_int, TS7BlocksOfType *pUsrData, itemsCount: *mut c_int) -> c_int;
    // Blocks functions
    // fn Cli_Upload(clinet: *mut S7Object, blockType: c_int, blockNum: c_int, pUsrData: *mut c_void, size: *mut c_int) -> c_int;
    // fn Cli_FullUpload(clinet: *mut S7Object, blockType: c_int, blockNum: c_int, pUsrData: *mut c_void, size: *mut c_int) -> c_int;
    // fn Cli_Download(clinet: *mut S7Object, blockNum: c_int, pUsrData: *mut c_void, size: c_int) -> c_int;
    // fn Cli_Delete(clinet: *mut S7Object, blockType: c_int, blockNum: c_int) -> c_int;
    fn Cli_DBGet(
        clinet: *mut S7Object,
        dbNumber: c_int,
        pUsrData: *mut c_void,
        size: *mut c_int,
    ) -> c_int;
    fn Cli_DBFill(clinet: *mut S7Object, dbNumber: c_int, fillChar: c_int) -> c_int;
    // Date/Time functions
    // fn Cli_GetPlcDateTime(clinet: *mut S7Object, tm *DateTime) -> c_int;
    // fn Cli_SetPlcDateTime(clinet: *mut S7Object, tm *DateTime) -> c_int;
    // fn Cli_SetPlcSystemDateTime(clinet: *mut S7Object) -> c_int;
    // System Info functions
    // fn Cli_GetOrderCode(clinet: *mut S7Object, TS7OrderCode *pUsrData) -> c_int;
    // fn Cli_GetCpuInfo(clinet: *mut S7Object, TS7CpuInfo *pUsrData) -> c_int;
    // fn Cli_GetCpInfo(clinet: *mut S7Object, TS7CpInfo *pUsrData) -> c_int;
    // fn Cli_ReadSZL(clinet: *mut S7Object, id: c_int, index: c_int, TS7SZL *pUsrData, size: *mut c_int) -> c_int;
    // fn Cli_ReadSZLList(clinet: *mut S7Object, TS7SZLList *pUsrData, itemsCount: *mut c_int) -> c_int;
    // Control functions
    // fn Cli_PlcHotStart(clinet: *mut S7Object) -> c_int;
    // fn Cli_PlcColdStart(clinet: *mut S7Object) -> c_int;
    // fn Cli_PlcStop(clinet: *mut S7Object) -> c_int;
    // fn Cli_CopyRamToRom(clinet: *mut S7Object, timeout: c_int) -> c_int;
    // fn Cli_Compress(clinet: *mut S7Object, timeout: c_int) -> c_int;
    // fn Cli_GetPlcStatus(clinet: *mut S7Object, status: *mut c_int) -> c_int;
    // Security functions
    // fn Cli_GetProtection(clinet: *mut S7Object, TS7Protection *pUsrData) -> c_int;
    // fn Cli_SetSessionPassword(clinet: *mut S7Object, char *Password) -> c_int;
    // fn Cli_ClearSessionPassword(clinet: *mut S7Object) -> c_int;
    // Low level
    // fn Cli_IsoExchangeBuffer(clinet: *mut S7Object, pUsrData: *mut c_void, size: *mut c_int) -> c_int;
    // Misc
    // fn Cli_GetExecTime(clinet: *mut S7Object, time: *mut c_int) -> c_int;
    // fn Cli_GetLastError(clinet: *mut S7Object, lastError: *mut c_int) -> c_int;
    // fn Cli_GetPduLength(clinet: *mut S7Object, requested: *mut c_int, negotiated: *mut c_int) -> c_int;
    // fn Cli_ErrorText(error: c_int, char *Text, textLen: c_int) -> c_int;
    // 1.1.0
    fn Cli_GetConnected(clinet: *mut S7Object, connected: *mut c_int) -> c_int;
//------------------------------------------------------------------------------
//  Async functions
//------------------------------------------------------------------------------
// fn Cli_AsReadArea(clinet: *mut S7Object, area: c_int, dbNumber: c_int, start: c_int, amount: c_int, wordLen: c_int, pUsrData: *mut c_void) -> c_int;
// fn Cli_AsWriteArea(clinet: *mut S7Object, area: c_int, dbNumber: c_int, start: c_int, amount: c_int, wordLen: c_int, pUsrData: *mut c_void) -> c_int;
// fn Cli_AsDBRead(clinet: *mut S7Object, dbNumber: c_int, start: c_int, size: c_int, pUsrData: *mut c_void) -> c_int;
// fn Cli_AsDBWrite(clinet: *mut S7Object, dbNumber: c_int, start: c_int, size: c_int, pUsrData: *mut c_void) -> c_int;
// fn Cli_AsMBRead(clinet: *mut S7Object, start: c_int, size: c_int, pUsrData: *mut c_void) -> c_int;
// fn Cli_AsMBWrite(clinet: *mut S7Object, start: c_int, size: c_int, pUsrData: *mut c_void) -> c_int;
// fn Cli_AsEBRead(clinet: *mut S7Object, start: c_int, size: c_int, pUsrData: *mut c_void) -> c_int;
// fn Cli_AsEBWrite(clinet: *mut S7Object, start: c_int, size: c_int, pUsrData: *mut c_void) -> c_int;
// fn Cli_AsABRead(clinet: *mut S7Object, start: c_int, size: c_int, pUsrData: *mut c_void) -> c_int;
// fn Cli_AsABWrite(clinet: *mut S7Object, start: c_int, size: c_int, pUsrData: *mut c_void) -> c_int;
// fn Cli_AsTMRead(clinet: *mut S7Object, start: c_int, amount: c_int, pUsrData: *mut c_void) -> c_int;
// fn Cli_AsTMWrite(clinet: *mut S7Object, start: c_int, amount: c_int, pUsrData: *mut c_void) -> c_int;
// fn Cli_AsCTRead(clinet: *mut S7Object, start: c_int, amount: c_int, pUsrData: *mut c_void) -> c_int;
// fn Cli_AsCTWrite(clinet: *mut S7Object, start: c_int, amount: c_int, pUsrData: *mut c_void) -> c_int;
// fn Cli_AsListBlocksOfType(clinet: *mut S7Object, blockType: c_int, TS7BlocksOfType *pUsrData, itemsCount: *mut c_int) -> c_int;
// fn Cli_AsReadSZL(clinet: *mut S7Object, id: c_int, index: c_int, TS7SZL *pUsrData, size: *mut c_int) -> c_int;
// fn Cli_AsReadSZLList(clinet: *mut S7Object, TS7SZLList *pUsrData, itemsCount: *mut c_int) -> c_int;
// fn Cli_AsUpload(clinet: *mut S7Object, blockType: c_int, blockNum: c_int, pUsrData: *mut c_void, size: *mut c_int) -> c_int;
// fn Cli_AsFullUpload(clinet: *mut S7Object, blockType: c_int, blockNum: c_int, pUsrData: *mut c_void, size: *mut c_int) -> c_int;
// fn Cli_AsDownload(clinet: *mut S7Object, blockNum: c_int, pUsrData: *mut c_void, size: c_int) -> c_int;
// fn Cli_AsCopyRamToRom(clinet: *mut S7Object, timeout: c_int) -> c_int;
// fn Cli_AsCompress(clinet: *mut S7Object, timeout: c_int) -> c_int;
// fn Cli_AsDBGet(clinet: *mut S7Object, dbNumber: c_int, pUsrData: *mut c_void, size: *mut c_int) -> c_int;
// fn Cli_AsDBFill(clinet: *mut S7Object, dbNumber: c_int, fillChar: c_int) -> c_int;
// fn Cli_CheckAsCompletion(clinet: *mut S7Object, opResult: *mut c_int) -> c_int;
// fn Cli_WaitAsCompletion(clinet: *mut S7Object, timeout: c_int) -> c_int;
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
}
