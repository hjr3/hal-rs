use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::BTreeMap;
use serialize::json::{ToJson, Json};
use serialize::{json};

use link::Link;
use state::{HalState, ToHalState};

// todo: maybe just convert these to BTreeMap? would save a lot of code
#[derive(Clone, PartialEq, Debug)]
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
                if key as &str == "_links" {
                    let links = value.as_object().unwrap();

                    for (link_key, link_object) in links.iter() {
                        resource = resource.add_link(
                            link_key.as_ref(),
                            Link::from_json(&link_object.to_json())
                        );
                    }
                } else {
                    resource = resource.add_state(key.as_ref(), value.clone());
                }
            }
        }

        resource
    }

    pub fn add_state<V>(self, key: &str, value: V) -> Resource where V: ToHalState {
        let mut resource = self.clone();
        resource.state.insert(key.to_string(), value.to_hal_state());
        resource
    }

    pub fn add_link(self, rel: &str, link: Link) -> Resource {
        let mut resource = self.clone();
        match resource.links.entry(rel.to_string()) {
            Vacant(entry) => {
                let l = vec![link.clone()];
                entry.insert(l);
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
        match new_r.resources.entry(rel.to_string()) {
            Vacant(entry) => {
                let r = vec![resource.clone()];
                entry.insert(r);
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
        let mut link_rels = BTreeMap::new();
        let mut embeds = BTreeMap::new();

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
            for (rel, resources) in self.resources.iter() {
                if resources.len() > 1 {
                    embeds.insert(rel.clone(), resources.to_json());
                } else {
                    embeds.insert(rel.clone(), resources[0].to_json());
                }
            }

            hal.insert("_embedded".to_string(), embeds.to_json());
        }

        json::Json::Object(hal)
    }
}
