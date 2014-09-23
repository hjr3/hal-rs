/*!
A pure rust library for generating Hal responses.
 */

#![crate_name = "hal"]
#![crate_type = "lib"]

#[warn(non_camel_case_types)]

extern crate serialize;
extern crate collections;

use std::collections::HashMap;
use std::collections::TreeMap;
use serialize::json::{ToJson, Json};
use serialize::{json, Encodable};

#[cfg(test)]
mod tests;

/// Represents Hal data value
#[deriving(Clone, Encodable)]
pub enum HalState {
    Number(f64),
    HalString(String),
    Boolean(bool),
    Null,
    HalList(List),
}

pub type List = Vec<HalState>;

/// A trait for converting values to Hal data
pub trait ToHalState {
    /// Converts the value of `self` to an instance of HalState
    fn to_hal_state(&self) -> HalState;
}

impl ToHalState for int {
    fn to_hal_state(&self) -> HalState { Number(*self as f64) }
}

impl ToHalState for f64 {
    fn to_hal_state(&self) -> HalState { Number(*self) }
}

impl ToHalState for () {
    fn to_hal_state(&self) -> HalState { Null }
}

impl ToHalState for bool {
    fn to_hal_state(&self) -> HalState { Boolean(*self) }
}

impl ToHalState for String {
    fn to_hal_state(&self) -> HalState { HalString((*self).clone()) }
}

impl ToHalState for &'static str {
    fn to_hal_state(&self) -> HalState { HalString((*self).to_string()) }
}

impl<T:ToHalState> ToHalState for Vec<T> {
    fn to_hal_state(&self) -> HalState { HalList(self.iter().map(|elt| elt.to_hal_state()).collect()) }
}

impl ToJson for HalState {
    fn to_json(&self) -> Json { 
        match *self {
            Number(v) => v.to_json(),
            HalString(ref v) => v.to_json(),
            Boolean(v) => v.to_json(),
            Null => ().to_json(),
            HalList(ref v) => v.to_json(),
        }
    }
}

#[deriving(Clone, Encodable)]
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
        let mut link = TreeMap::new();
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

        json::Object(link)
    }
}

#[deriving(Clone, Encodable)]
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

    pub fn add_state(self, key: &str, value: HalState) -> Resource {
        let mut resource = self.clone();
        resource.state.insert(String::from_str(key), value);
        resource
    }

    pub fn add_link(self, rel: &str, link: Link) -> Resource {
        let mut resource = self.clone();
        let l = vec![link.clone()];
        resource.links.insert_or_update_with(String::from_str(rel), l, |_, links| {
            links.push(link.clone())
        });
        resource
    }

    pub fn add_curie(self, name: &str, href: &str) -> Resource {
        let link = Link::new(href).templated(true).name(name);
        self.add_link("curies", link)
    }

    pub fn add_resource(self, rel: &str, resource: Resource) -> Resource {
        let mut new_r = self.clone();
        let r = vec![resource.clone()];
        new_r.resources.insert_or_update_with(String::from_str(rel), r, |_, resources| {
            resources.push(resource.clone())
        });
        new_r
    }
}

impl ToJson for Resource {
    fn to_json(&self) -> json::Json {
        let mut hal = TreeMap::new();
        let mut link_rels = box TreeMap::new();
        let mut embeds = box TreeMap::new();

        if self.links.len() > 0 {
            for (rel, links) in self.links.iter() {
                if links.len() > 1 || (rel.as_slice() == "curies") {
                    link_rels.insert(rel.as_slice().into_string(), (*links).to_json());
                } else {
                    link_rels.insert(rel.as_slice().into_string(), links[0].to_json());
                }

            }

            hal.insert("_links".to_string(), link_rels.to_json());
        }


        for (k, v) in self.state.iter() {
            hal.insert(k.clone().into_string(), v.to_json());
        }

        if self.resources.len() > 0 {
            for (rel, resources) in self.resources.iter() {
                if resources.len() > 1 {
                    embeds.insert(rel.as_slice().into_string(), resources.to_json());
                } else {
                    embeds.insert(rel.as_slice().into_string(), resources[0].to_json());
                }
            }

            hal.insert("_embedded".to_string(), embeds.to_json());
        }

        json::Object(hal)
    }
}

pub trait ToHal {
    fn to_hal(&self) -> Resource;
}
