use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};
use std::collections::hash_map::Iter;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct ExifData {
    pub data: HashMap<String, Value>, //HashMap<&'a str, Value>,
}

impl ExifData {
    pub fn iter(&self) -> Iter<'_, String, Value> {
        self.data.iter()
    }
}
