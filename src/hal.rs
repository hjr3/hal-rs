/*!
A pure rust library for generating Hal responses.
 */

#![crate_name = "hal"]
#![crate_type = "lib"]

#[warn(non_camel_case_types)]

extern crate serialize;
extern crate collections;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::BTreeMap;
use serialize::json::{ToJson, Json};
use serialize::{json};

#[cfg(test)]
mod tests;

/// Represents Hal data value
#[deriving(Clone, PartialEq, Show)]
pub enum HalState {
    I64(i64),
    F64(f64),
    U64(u64),
    HalString(String),
    Boolean(bool),
    Null,
    HalList(List),
    Object(HalObject),
}

pub type List = Vec<HalState>;
pub type HalObject = BTreeMap<String, HalState>;

/// A trait for converting values to Hal data
pub trait ToHalState {
    /// Converts the value of `self` to an instance of HalState
    fn to_hal_state(&self) -> HalState;
}

impl ToHalState for int {
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
    fn to_hal_state(&self) -> HalState { HalState::HalList(self.iter().map(|elt| elt.to_hal_state()).collect()) }
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
            HalState::HalList(ref v) => v.to_json(),
            HalState::Object(ref v) => v.to_json(),
        }
    }
}

#[deriving(Clone, PartialEq, Show)]
pub struct Link {
    href: String,
    templated: Option<bool>,
    media_type: Option<String>,
    deprecation: Option<String>,
    name: Option<String>,
    profile: Option<String>,
    title: Option<String>,
    hreflang: Option<String>,
}

impl Link {
    pub fn new(href: &str) -> Link {
        Link { href: String::from_str(href),
        templated: None,
        media_type: None,
        deprecation: None,
        name: None,
        profile: None,
        title: None,
        hreflang: None
        }
    }

    pub fn from_json(json: &Json) -> Link {
        let ref url = json["href".as_slice()];

        let mut link = Link::new(url.as_string().unwrap());

        // todo: we use these same hard coded strings to conver to json
        // todo: make this a macro
        if json.search("templated").is_some() {
            let value = json.search("templated").unwrap();
            link = link.templated(value.as_boolean().unwrap());
        } 

        if json.search("type").is_some() {
            let value = json.search("type").unwrap();
            link = link.media_type(value.as_string().unwrap());
        } 

        if json.search("deprecation").is_some() {
            let value = json.search("deprecation").unwrap();
            link = link.deprecation(value.as_string().unwrap());
        } 

        if json.search("name").is_some() {
            let value = json.search("name").unwrap();
            link = link.name(value.as_string().unwrap());
        } 

        if json.search("title").is_some() {
            let value = json.search("title").unwrap();
            link = link.title(value.as_string().unwrap());
        } 

        if json.search("profile").is_some() {
            let value = json.search("profile").unwrap();
            link = link.profile(value.as_string().unwrap());
        } 

        if json.search("hreflang").is_some() {
            let value = json.search("hreflang").unwrap();
            link = link.hreflang(value.as_string().unwrap());
        } 

        link
    }

    pub fn templated(self, is_template: bool) -> Link {
        let mut link = self.clone();
        link.templated = Some(is_template);
        link
    }

    pub fn media_type(self, media_type: &str) -> Link {
        let mut link = self.clone();
        link.media_type = Some(String::from_str(media_type));
        link
    }

    pub fn deprecation(self, deprecation: &str) -> Link {
        let mut link = self.clone();
        link.deprecation = Some(String::from_str(deprecation));
        link
    }

    pub fn name(self, name: &str) -> Link {
        let mut link = self.clone();
        link.name = Some(String::from_str(name));
        link
    }

    pub fn title(self, title: &str) -> Link {
        let mut link = self.clone();
        link.title = Some(String::from_str(title));
        link
    }

    pub fn profile(self, profile: &str) -> Link {
        let mut link = self.clone();
        link.profile = Some(String::from_str(profile));
        link
    }

    pub fn hreflang(self, hreflang: &str) -> Link {
        let mut link = self.clone();
        link.hreflang = Some(String::from_str(hreflang));
        link
    }
}

impl ToJson for Link {
    fn to_json(&self) -> json::Json {
        let mut link = BTreeMap::new();
        link.insert("href".to_string(), self.href.to_json());

        if self.templated.is_some() {
            link.insert("templated".to_string(), self.templated.to_json());
        }

        if self.media_type.is_some() {
            link.insert("type".to_string(), self.media_type.to_json());
        }

        if self.deprecation.is_some() {
            link.insert("deprecation".to_string(), self.deprecation.to_json());
        }

        if self.name.is_some() {
            link.insert("name".to_string(), self.name.to_json());
        }

        if self.title.is_some() {
            link.insert("title".to_string(), self.title.to_json());
        }

        if self.profile.is_some() {
            link.insert("profile".to_string(), self.profile.to_json());
        }

        if self.hreflang.is_some() {
            link.insert("hreflang".to_string(), self.hreflang.to_json());
        }

        json::Json::Object(link)
    }
}

// todo: maybe just convert these to BTreeMap? would save a lot of code
#[deriving(Clone, PartialEq, Show)]
pub struct Resource {
    state: HashMap<String, HalState>,
    links: HashMap<String, Vec<Link>>,
    resources: HashMap<String, Vec<Resource>>
}

impl Resource {
    pub fn new() -> Resource {
        Resource { state: HashMap::new(), links: HashMap::new(), resources: HashMap::new() }
    }

    pub fn with_self(uri: &str) -> Resource {
        Resource::new().add_link("self", Link::new(uri))
    }

    /// This feature is still experimental.
    pub fn from_json(json: Json) -> Resource {
        let mut resource = Resource::new();

        if json.is_object() {
            let json = json.as_object().unwrap();
            for (key, value) in json.iter() {
                if key.as_slice() == "_links" {
                    let links = value.as_object().unwrap();

                    for (link_key, link_object) in links.iter() {
                        resource = resource.add_link(
                            link_key.as_slice(),
                            Link::from_json(&link_object.to_json())
                        );
                    }
                } else {
                    resource = resource.add_state(key.as_slice(), value.clone());
                }
            }
        }

        resource
    }

    pub fn add_state<V>(self, key: &str, value: V) -> Resource where V: ToHalState {
        let mut resource = self.clone();
        resource.state.insert(String::from_str(key), value.to_hal_state());
        resource
    }

    pub fn add_link(self, rel: &str, link: Link) -> Resource {
        let mut resource = self.clone();
        match resource.links.entry(String::from_str(rel)) {
            Vacant(entry) => {
                let l = vec![link.clone()];
                entry.set(l);
            },
            Occupied(entry) => {
                let links = entry.into_mut();
                links.push(link.clone());
            }
        };

        resource
    }

    pub fn add_curie(self, name: &str, href: &str) -> Resource {
        let link = Link::new(href).templated(true).name(name);
        self.add_link("curies", link)
    }

    pub fn add_resource(self, rel: &str, resource: Resource) -> Resource {
        let mut new_r = self.clone();
        match new_r.resources.entry(String::from_str(rel)) {
            Vacant(entry) => {
                let r = vec![resource.clone()];
                entry.set(r);
            },
            Occupied(entry) => {
                let resources = entry.into_mut();
                resources.push(resource.clone());
            }
        }

        new_r
    }
}

impl ToJson for Resource {
    fn to_json(&self) -> json::Json {
        let mut hal = BTreeMap::new();
        let mut link_rels = box BTreeMap::new();
        let mut embeds = box BTreeMap::new();

        if self.links.len() > 0 {
            for (rel, links) in self.links.iter() {
                if links.len() > 1 || (rel.as_slice() == "curies") {
                    link_rels.insert(rel.as_slice().to_string(), (*links).to_json());
                } else {
                    link_rels.insert(rel.as_slice().to_string(), links[0].to_json());
                }

            }

            hal.insert("_links".to_string(), link_rels.to_json());
        }


        for (k, v) in self.state.iter() {
            hal.insert(k.clone().to_string(), v.to_json());
        }

        if self.resources.len() > 0 {
            for (rel, resources) in self.resources.iter() {
                if resources.len() > 1 {
                    embeds.insert(rel.as_slice().to_string(), resources.to_json());
                } else {
                    embeds.insert(rel.as_slice().to_string(), resources[0].to_json());
                }
            }

            hal.insert("_embedded".to_string(), embeds.to_json());
        }

        json::Json::Object(hal)
    }
}

pub trait ToHal {
    fn to_hal(self) -> Resource;
}
