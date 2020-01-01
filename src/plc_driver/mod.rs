pub mod s7;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum Datatype {
    BOOL,
    INT,
    DINT,
    REAL,
    STRING,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum ETagValue {
    Bool(bool),
    Int(i64),
    Real(f64),
    Str(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ETag {
    name: String,
    address: String,
    datatype: Datatype,
    read: Result<ETagValue, String>,
    write: Option<ETagValue>,
}

trait ETagRW {
    fn read_tag(&self, tag: &mut ETag) -> Result<ETagValue, String>;
    fn read_list(&self, tags: &mut &[ETag]);
    fn write_tag(&self, tag: &mut ETag) -> Result<bool, String>;
}
