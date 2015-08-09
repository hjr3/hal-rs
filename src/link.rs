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
    pub fn new<S: Into<String>>(href: S) -> Link {
        Link { href: href.into(),
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
        let ref url = json["href"];

        let mut link = Link::new(url.as_string().unwrap());

        // todo: we use these same hard coded strings to conver to json
        // todo: make this a macro
        if json.search("templated").is_some() {
            let value = json.search("templated").unwrap();
            link.templated(value.as_boolean().unwrap());
        } 

        if json.search("type").is_some() {
            let value = json.search("type").unwrap();
            link.media_type(value.as_string().unwrap());
        } 

        if json.search("deprecation").is_some() {
            let value = json.search("deprecation").unwrap();
            link.deprecation(value.as_string().unwrap());
        } 

        if json.search("name").is_some() {
            let value = json.search("name").unwrap();
            link.name(value.as_string().unwrap());
        } 

        if json.search("title").is_some() {
            let value = json.search("title").unwrap();
            link.title(value.as_string().unwrap());
        } 

        if json.search("profile").is_some() {
            let value = json.search("profile").unwrap();
            link.profile(value.as_string().unwrap());
        } 

        if json.search("hreflang").is_some() {
            let value = json.search("hreflang").unwrap();
            link.hreflang(value.as_string().unwrap());
        } 

        link
    }

    pub fn templated(&mut self, is_template: bool) -> &mut Link {
        self.templated = Some(is_template);
        self
    }

    pub fn media_type<S>(&mut self, media_type: S) -> &mut Link
        where S: Into<String> {
        self.media_type = Some(media_type.into());
        self
    }

    pub fn deprecation<S>(&mut self, deprecation: S) -> &mut Link
        where S: Into<String> {
        self.deprecation = Some(deprecation.into());
        self
    }

    pub fn name<S>(&mut self, name: S) -> &mut Link
        where S: Into<String> {
        self.name = Some(name.into());
        self
    }

    pub fn title<S>(&mut self, title: S) -> &mut Link
        where S: Into<String> {
        self.title = Some(title.into());
        self
    }

    pub fn profile<S>(&mut self, profile: S) -> &mut Link
        where S: Into<String> {
        self.profile = Some(profile.into());
        self
    }

    pub fn hreflang<S>(&mut self, hreflang: S) -> &mut Link
        where S: Into<String> {
        self.hreflang = Some(hreflang.into());
        self
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
