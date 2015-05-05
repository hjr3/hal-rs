use std::collections::BTreeMap;
use serialize::json::{ToJson, Json};

/// A Hal Link object
#[derive(Clone, Eq, PartialEq, Debug)]
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
        Link { href: href.to_string(),
        templated: None,
        media_type: None,
        deprecation: None,
        name: None,
        profile: None,
        title: None,
        hreflang: None
        }
    }

    /// Convert a json object into a Link
    pub fn from_json(json: &Json) -> Link {
        let ref url = json["href".as_ref()];

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
        link.media_type = Some(media_type.to_string());
        link
    }

    pub fn deprecation(self, deprecation: &str) -> Link {
        let mut link = self.clone();
        link.deprecation = Some(deprecation.to_string());
        link
    }

    pub fn name(self, name: &str) -> Link {
        let mut link = self.clone();
        link.name = Some(name.to_string());
        link
    }

    pub fn title(self, title: &str) -> Link {
        let mut link = self.clone();
        link.title = Some(title.to_string());
        link
    }

    pub fn profile(self, profile: &str) -> Link {
        let mut link = self.clone();
        link.profile = Some(profile.to_string());
        link
    }

    pub fn hreflang(self, hreflang: &str) -> Link {
        let mut link = self.clone();
        link.hreflang = Some(hreflang.to_string());
        link
    }
}

impl ToJson for Link {
    fn to_json(&self) -> Json {
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

        Json::Object(link)
    }
}
