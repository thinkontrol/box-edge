pub mod s7;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ETagtype {
    BOOL,
    INT,
    DINT,
    REAL,
    // STRING(u16),
}
impl ETagtype {
    pub fn is_bool(&self) -> bool {
        match self {
            ETagtype::BOOL => true,
            _ => false
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(untagged)]
pub enum ETagValue {
    Bool(bool),
    Int(i64),
    Real(f64),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ETag {
    pub name: String,
    pub address: String,
    pub datatype: ETagtype,
}

pub trait ETagRW {
    fn read_tag(&self, tag: &ETag) -> Result<ETagValue, String>;
    fn read_list(&self, tags: &Vec::<ETag>) -> Result<Vec::<Result<ETagValue, String>>, String>;
    fn write_tag(&self, tag: &ETag, write: ETagValue) -> Result<bool, String>;
    fn write_list(&self, tags: &Vec::<(ETag, ETagValue)>) -> Result<Vec::<Result<bool, String>>, String>;
}
