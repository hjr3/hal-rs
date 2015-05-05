use std::collections::HashMap;
use std::collections::BTreeMap;
use serialize::json::{ToJson, Json};

/// Represents a Hal data value
#[derive(Clone, PartialEq, Debug)]
pub enum HalState {
    I64(i64),
    F64(f64),
    U64(u64),
    HalString(String),
    Boolean(bool),
    Null,
    List(HalList),
    Object(HalObject),
}

/// A vector of HalState enums to be serialized into json
pub type HalList = Vec<HalState>;

/// A map of strings and HalState enums to be serialized into json
pub type HalObject = BTreeMap<String, HalState>;

/// A trait for converting values to Hal data
pub trait ToHalState {
    /// Converts the value of `self` to an instance of HalState
    fn to_hal_state(&self) -> HalState;
}

impl ToHalState for isize {
    fn to_hal_state(&self) -> HalState { HalState::I64(*self as i64) }
}

impl ToHalState for i64 {
    fn to_hal_state(&self) -> HalState { HalState::I64(*self) }
}

impl ToHalState for u64 {
    fn to_hal_state(&self) -> HalState { HalState::U64(*self) }
}

impl ToHalState for f64 {
    fn to_hal_state(&self) -> HalState { HalState::F64(*self) }
}

impl ToHalState for () {
    fn to_hal_state(&self) -> HalState { HalState::Null }
}

impl ToHalState for bool {
    fn to_hal_state(&self) -> HalState { HalState::Boolean(*self) }
}

impl ToHalState for String {
    fn to_hal_state(&self) -> HalState { HalState::HalString((*self).clone()) }
}

impl ToHalState for &'static str {
    fn to_hal_state(&self) -> HalState { HalState::HalString((*self).to_string()) }
}

impl<T:ToHalState> ToHalState for Vec<T> {
    fn to_hal_state(&self) -> HalState { HalState::List(self.iter().map(|elt| elt.to_hal_state()).collect()) }
}

impl<T:ToHalState> ToHalState for BTreeMap<String, T> {
    fn to_hal_state(&self) -> HalState { 
        let mut t = BTreeMap::new();
        for (key, value) in self.iter() {
            t.insert((*key).clone(), value.to_hal_state());
        }
        HalState::Object(t)
    }
}

impl<T:ToHalState> ToHalState for HashMap<String, T> {
    fn to_hal_state(&self) -> HalState { 
        let mut t = BTreeMap::new();
        for (key, value) in self.iter() {
            t.insert((*key).clone(), value.to_hal_state());
        }
        HalState::Object(t)
    }
}

impl<T:ToHalState> ToHalState for Option<T> {
    fn to_hal_state(&self) -> HalState {
        match *self {
            None => HalState::Null,
            Some(ref value) => value.to_hal_state()
        }
    }
}

impl ToHalState for Json {
    fn to_hal_state(&self) -> HalState {
        match *self {
            Json::I64(v) => v.to_hal_state(),
            Json::U64(v) => v.to_hal_state(),
            Json::F64(v) => v.to_hal_state(),
            Json::String(ref v) => v.to_hal_state(),
            Json::Boolean(v) => v.to_hal_state(),
            Json::Array(ref v) => v.to_hal_state(),
            Json::Object(ref v) => v.to_hal_state(),
            Json::Null => ().to_hal_state(),
        }
    }
}

impl ToJson for HalState {
    fn to_json(&self) -> Json { 
        match *self {
            HalState::I64(v) => v.to_json(),
            HalState::F64(v) => v.to_json(),
            HalState::U64(v) => v.to_json(),
            HalState::HalString(ref v) => v.to_json(),
            HalState::Boolean(v) => v.to_json(),
            HalState::Null => ().to_json(),
            HalState::List(ref v) => v.to_json(),
            HalState::Object(ref v) => v.to_json(),
        }
    }
}

