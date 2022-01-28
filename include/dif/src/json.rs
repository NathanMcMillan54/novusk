use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::{convert::From, fmt::Display};
use core::fmt::Formatter;
use rjson::{parse, Array, Null, Object,  Value};

#[derive(Clone, Debug)]
pub(crate) enum JsonValue {
    Null,
    Number(f64),
    Bool(bool),
    String(String),
    Array(Vec<JsonValue>),
    Object(BTreeMap<String, JsonValue>)
}

impl Display for JsonValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{:?}", *self))
    }
}

pub(crate) struct JsonArray(Vec<JsonValue>);
pub(crate) struct JsonObject(BTreeMap<String, JsonValue>);

impl Array<JsonValue, JsonObject, JsonValue> for JsonArray {
    fn new() -> Self {
        JsonArray(Vec::new())
    }
    fn push(&mut self, v: JsonValue) {
        self.0.push(v)
    }
}

impl Object<JsonValue, JsonArray, JsonValue> for JsonObject {
    fn new<'b>() -> Self {
        JsonObject(BTreeMap::new())
    }
    fn insert(&mut self, k: String, v: JsonValue) {
        self.0.insert(k, v);
    }
}

impl Null<JsonValue, JsonArray, JsonObject> for JsonValue {
    fn new() -> Self {
        JsonValue::Null
    }
}
impl Value<JsonArray, JsonObject, JsonValue> for JsonValue {}

impl From<f64> for JsonValue {
    fn from(v: f64) -> Self {
        JsonValue::Number(v)
    }
}
impl From<bool> for JsonValue {
    fn from(v: bool) -> Self {
        JsonValue::Bool(v)
    }
}
impl From<String> for JsonValue {
    fn from(v: String) -> Self{
        JsonValue::String(v)
    }
}
impl From<JsonArray> for JsonValue {
    fn from(v: JsonArray) -> Self {
        JsonValue::Array(v.0)
    }
}
impl From<JsonObject> for JsonValue {
    fn from(v: JsonObject) -> Self {
        JsonValue::Object(v.0)
    }
}
