use std::collections::btree_map::Entry::{Occupied, Vacant};
use std::collections::BTreeMap;
use serialize::json::{ToJson, Json};
use serialize::{json};

use link::Link;
use state::{HalState, ToHalState};

#[derive(Clone, PartialEq, Debug)]
pub struct Resource {
    state: BTreeMap<String, HalState>,
    links: BTreeMap<String, Vec<Link>>,
    resources: BTreeMap<String, Vec<Resource>>
}

impl Resource {
    pub fn new() -> Resource {
        Resource { state: BTreeMap::new(), links: BTreeMap::new(), resources: BTreeMap::new() }
    }

    pub fn with_self<S>(uri: S) -> Resource
        where S: Into<String> {

        let link = Link::new(uri);
        let mut resource = Resource::new();
        resource.add_link("self", &link);
        resource
    }

    /// This feature is still experimental.
    pub fn from_json(json: Json) -> Resource {
        let mut resource = Resource::new();

        if json.is_object() {
            let json = json.as_object().unwrap();
            for (key, value) in json.iter() {
                if key as &str == "_links" {
                    let links = value.as_object().unwrap();

                    for (link_key, link_object) in links.iter() {
                        let link = Link::from_json(&link_object.to_json());
                        resource.add_link(
                            link_key.as_ref(),
                            &link
                        );
                    }
                } else {
                    resource.add_state(key.as_ref(), value.clone());
                }
            }
        }

        resource
    }

    pub fn add_state<S, V>(&mut self, key: S, value: V) -> &mut Resource 
        where V: ToHalState, S: Into<String> {
        self.state.insert(key.into(), value.to_hal_state());
        self
    }

    pub fn add_link<S>(&mut self, rel: S, link: &Link) -> &mut Resource
        where S: Into<String> {
        match self.links.entry(rel.into()) {
            Vacant(entry) => {
                let l = vec![link.clone()];
                entry.insert(l);
            },
            Occupied(entry) => {
                let links = entry.into_mut();
                links.push(link.clone());
            }
        };

        self
    }

    pub fn add_curie<S>(&mut self, name: S, href: S) -> &mut Resource
        where S: Into<String> {
        let mut link = Link::new(href);
        link.templated(true).name(name);
        self.add_link("curies", &link)
    }

    pub fn add_resource<S>(&mut self, rel: S, resource: &Resource) -> &mut Resource
        where S: Into<String> {
        match self.resources.entry(rel.into()) {
            Vacant(entry) => {
                let r = vec![resource.clone()];
                entry.insert(r);
            },
            Occupied(entry) => {
                let resources = entry.into_mut();
                resources.push(resource.clone());
            }
        }

        self
    }
}

impl ToJson for Resource {
    fn to_json(&self) -> json::Json {
        let mut hal = BTreeMap::new();
        let mut link_rels = BTreeMap::new();

        if self.links.len() > 0 {
            for (rel, links) in self.links.iter() {
                if links.len() > 1 || (rel as &str == "curies") {
                    link_rels.insert(rel.clone(), (*links).to_json());
                } else {
                    link_rels.insert(rel.clone(), links[0].to_json());
                }

            }

            hal.insert("_links".to_string(), link_rels.to_json());
        }


        for (k, v) in self.state.iter() {
            hal.insert(k.clone().to_string(), v.to_json());
        }

        if self.resources.len() > 0 {
            hal.insert("_embedded".to_string(), self.resources.to_json());
        }

        json::Json::Object(hal)
    }
}
