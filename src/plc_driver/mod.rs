pub mod s7;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ETagtype {
    BOOL,
    INT,
    DINT,
    REAL,
    // STRING(u16),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ETagValue {
    Bool(bool),
    Int(i64),
    Real(f64),
    Str(String),
    None,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ETag {
    name: String,
    address: String,
    datatype: ETagtype,
    read: Result<ETagValue, String>,
    write: Option<ETagValue>,
}

pub trait ETagRW {
    fn read_tag(&self, tag: &mut ETag) -> Result<bool, String>;
    fn read_list(&self, tags: &mut &[ETag]) -> Result<bool, String>;
    fn write_tag(&self, tag: &mut ETag) -> Result<bool, String>;
    fn write_list(&self, tag: &mut &[ETag]) -> Result<bool, String>;
}
