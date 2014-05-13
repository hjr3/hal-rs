/*!
A pure rust library for generating Hal responses.
 */

#![crate_id = "github.com/hjr3/rust-hal#hal:0.0"]
#![crate_type = "lib"]

#[warn(non_camel_case_types)]

extern crate serialize;
extern crate collections;

use collections::HashMap;
use collections::TreeMap;
use serialize::json::{ToJson, Json};
use serialize::{json, Encodable};

#[cfg(test)]
mod tests;

/// Represents Hal data value
#[deriving(Clone, Encodable, Eq)]
pub enum Data {
    Number(f64),
    String(StrBuf),
    Boolean(bool),
    Null,
}

/// A trait for converting values to Hal data
pub trait ToHalData {
    /// Converts the value of `self` to an instance of Data
    fn to_hal_data(&self) -> Data;
}

impl ToHalData for int {
    fn to_hal_data(&self) -> Data { Number(*self as f64) }
}

impl ToHalData for f64 {
    fn to_hal_data(&self) -> Data { Number(*self) }
}

impl ToHalData for () {
    fn to_hal_data(&self) -> Data { Null }
}

impl ToHalData for bool {
    fn to_hal_data(&self) -> Data { Boolean(*self) }
}

impl ToHalData for StrBuf {
    fn to_hal_data(&self) -> Data { String((*self).clone()) }
}

impl ToHalData for ~str {
    fn to_hal_data(&self) -> Data { String(StrBuf::from_owned_str((*self).clone())) }
}

impl ToJson for Data {
    fn to_json(&self) -> Json { 
        match *self {
            Number(v) => v.to_json(),
            String(ref v) => v.to_json(),
            Boolean(v) => v.to_json(),
            Null => ().to_json()
        }
    }
}

#[deriving(Clone, Encodable)]
pub struct Link {
    href: StrBuf,
    templated: Option<bool>,
    media_type: Option<StrBuf>,
    deprecation: Option<StrBuf>,
    name: Option<StrBuf>,
    profile: Option<StrBuf>,
    title: Option<StrBuf>,
    hreflang: Option<StrBuf>,
}

impl Link {
    pub fn new(href: &str) -> Link {
        Link { href: StrBuf::from_str(href),
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
        link.media_type = Some(StrBuf::from_str(media_type));
        link
    }

    pub fn deprecation(self, deprecation: &str) -> Link {
        let mut link = self.clone();
        link.deprecation = Some(StrBuf::from_str(deprecation));
        link
    }

    pub fn name(self, name: &str) -> Link {
        let mut link = self.clone();
        link.name = Some(StrBuf::from_str(name));
        link
    }

    pub fn title(self, title: &str) -> Link {
        let mut link = self.clone();
        link.title = Some(StrBuf::from_str(title));
        link
    }

    pub fn profile(self, profile: &str) -> Link {
        let mut link = self.clone();
        link.profile = Some(StrBuf::from_str(profile));
        link
    }

    pub fn hreflang(self, hreflang: &str) -> Link {
        let mut link = self.clone();
        link.hreflang = Some(StrBuf::from_str(hreflang));
        link
    }
}

impl ToJson for Link {
    fn to_json(&self) -> json::Json {
        let mut link = box TreeMap::new();
        link.insert("href".to_owned(), self.href.to_json());

        if self.templated.is_some() {
            link.insert("templated".to_owned(), self.templated.to_json());
        }

        if self.media_type.is_some() {
            link.insert("type".to_owned(), self.media_type.to_json());
        }

        if self.deprecation.is_some() {
            link.insert("deprecation".to_owned(), self.deprecation.to_json());
        }

        if self.name.is_some() {
            link.insert("name".to_owned(), self.name.to_json());
        }

        if self.title.is_some() {
            link.insert("title".to_owned(), self.title.to_json());
        }

        if self.profile.is_some() {
            link.insert("profile".to_owned(), self.profile.to_json());
        }

        if self.hreflang.is_some() {
            link.insert("hreflang".to_owned(), self.hreflang.to_json());
        }

        json::Object(link)
    }
}

#[deriving(Clone, Encodable)]
pub struct Resource {
    state: HashMap<StrBuf, Data>,
    links: HashMap<StrBuf, Vec<Link>>,
    resources: HashMap<StrBuf, Vec<Resource>>
}

impl Resource {
    pub fn new() -> Resource {
        Resource { state: HashMap::new(), links: HashMap::new(), resources: HashMap::new() }
    }

    pub fn with_self(uri: &str) -> Resource {
        Resource::new().add_link("self", Link::new(uri))
    }

    pub fn add_state(self, key: &str, value: Data) -> Resource {
        let mut resource = self.clone();
        resource.state.insert(StrBuf::from_str(key), value);
        resource
    }

    pub fn add_link(self, rel: &str, link: Link) -> Resource {
        let mut resource = self.clone();
        let l = vec![link.clone()];
        resource.links.insert_or_update_with(StrBuf::from_str(rel), l, |_, links| {
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
        new_r.resources.insert_or_update_with(StrBuf::from_str(rel), r, |_, resources| {
            resources.push(resource.clone())
        });
        new_r
    }
}

impl ToJson for Resource {
    fn to_json(&self) -> json::Json {
        let mut hal = box TreeMap::new();
        let mut link_rels = box TreeMap::new();
        let mut embeds = box TreeMap::new();

        if self.links.len() > 0 {
            for (rel, links) in self.links.iter() {
                if links.len() > 1 || (rel.as_slice() == "curies") {
                    link_rels.insert(rel.as_slice().into_owned(), (*links).to_json());
                } else {
                    link_rels.insert(rel.as_slice().into_owned(), links.get(0).to_json());
                }

            }

            hal.insert("_links".to_owned(), link_rels.to_json());
        }


        for (k, v) in self.state.iter() {
            hal.insert(k.clone().into_owned(), v.to_json());
        }

        if self.resources.len() > 0 {
            for (rel, resources) in self.resources.iter() {
                if resources.len() > 1 {
                    embeds.insert(rel.as_slice().into_owned(), resources.to_json());
                } else {
                    embeds.insert(rel.as_slice().into_owned(), resources.get(0).to_json());
                }
            }

            hal.insert("_embedded".to_owned(), embeds.to_json());
        }

        json::Object(hal)
    }
}

pub trait ToHal {
    fn to_hal(&self) -> Resource;
}
