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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ETag {
    pub name: String,
    pub address: String,
    pub datatype: ETagtype,
}

pub trait ETagRW {
    fn read_tag(&self, tag: &ETag) -> Result<ETagValue, String>;
    fn read_list(&self, tags: &mut &[ETag]) -> Result<bool, String>;
    fn write_tag(&self, tag: &ETag, write: ETagValue) -> Result<bool, String>;
    fn write_list(&self, tags: &[ETag]) -> Result<bool, String>;
}
