extern crate hal;
extern crate rustc_serialize as serialize;

use hal::ToHal;
use hal::resource::Resource;
use hal::link::Link;
use hal::state::ToHalState;
use hal::state::HalState::{I64, Null};
use serialize::json::Json;
use serialize::json::ToJson;
use std::collections::{HashMap, BTreeMap};

struct Order {
    total: f64,
    currency: String,
    status: String
}

impl ToHal for Order {
    fn to_hal(self) -> Resource {
        Resource::with_self("https://www.example.com/orders/1")
            .add_state("total", self.total)
            .add_state("currency", self.currency)
            .add_state("status", self.status)
    }
}

#[test]
fn link_new() {
    let link = Link::new("https://www.example.com");

    let output = r#"{"href":"https://www.example.com"}"#;
    assert_eq!(link.to_json().to_string(), output.to_string());
}

#[test]
fn link_from_json() {
    let json_str = r#"{"deprecation":"https://www.example.com/newer","href":"https://www.example.com","hreflang":"en","name":"example","profile":"http://tools.ietf.org/html/draft-wilde-profile-link-04","templated":true,"title":"An example link","type":"text/html"}"#;

    let json = Json::from_str(json_str).unwrap();

    let link = Link::new("https://www.example.com")
        .templated(true)
        .deprecation("https://www.example.com/newer")
        .media_type("text/html")
        .name("example")
        .title("An example link")
        .profile("http://tools.ietf.org/html/draft-wilde-profile-link-04")
        .hreflang("en");

    assert_eq!(link, Link::from_json(&json));
}

#[test]
fn link_attributes() {
    let link = Link::new("https://www.example.com")
        .templated(true)
        .deprecation("https://www.example.com/newer")
        .media_type("text/html")
        .name("example")
        .title("An example link")
        .profile("http://tools.ietf.org/html/draft-wilde-profile-link-04")
        .hreflang("en");

    let output = r#"{"deprecation":"https://www.example.com/newer","href":"https://www.example.com","hreflang":"en","name":"example","profile":"http://tools.ietf.org/html/draft-wilde-profile-link-04","templated":true,"title":"An example link","type":"text/html"}"#;
    assert_eq!(link.to_json().to_string(), output.to_string());
}

#[test]
fn hal_add_resource() {
    let hal = Resource::new();
    hal.add_resource("orders", Resource::new());
}

#[test]
fn hal_new() {
    let hal = Resource::new();

    let output = r#"{}"#;
    assert_eq!(hal.to_json().to_string(), output.to_string());
}

#[test]
fn hal_with_self() {
    let hal = Resource::with_self("https://www.example.com");

    let output = r#"{"_links":{"self":{"href":"https://www.example.com"}}}"#;
    assert_eq!(hal.to_json().to_string(), output.to_string());
}

#[test]
fn hal_from_json() {
    let hal = Resource::with_self("https://www.example.com")
        .add_state("currentlyProcessing", 14i64)
        .add_state("currency", "USD")
        .add_state("active", true)
        .add_state("errors", ());

    assert_eq!(hal, Resource::from_json(hal.to_json()));
}

#[test]
fn hal_with_self_and_link() {
    let output = r#"{"_links":{"orders":{"href":"https://www.example.com/orders"},"self":{"href":"https://www.example.com"}}}"#;
    let hal = Resource::with_self("https://www.example.com")
        .add_link("orders", Link::new("https://www.example.com/orders"));
    assert_eq!(hal.to_json().to_string(), output.to_string());
}

#[test]
fn hal_with_self_and_two_links() {
    let hal = Resource::with_self("https://www.example.com")
        .add_link("orders", Link::new("https://www.example.com/orders/1"))
        .add_link("orders", Link::new("https://www.example.com/orders/2"));

    let output = r#"{"_links":{"orders":[{"href":"https://www.example.com/orders/1"},{"href":"https://www.example.com/orders/2"}],"self":{"href":"https://www.example.com"}}}"#;
    assert_eq!(hal.to_json().to_string(), output.to_string());
}

#[test]
fn hal_and_add_curie() {
    let hal = Resource::with_self("https://www.example.com")
        .add_curie("ea", "http://example.com/docs/rels/{rel}");


    let output = r#"{"_links":{"curies":[{"href":"http://example.com/docs/rels/{rel}","name":"ea","templated":true}],"self":{"href":"https://www.example.com"}}}"#;
    assert_eq!(hal.to_json().to_string(), output.to_string());
}

#[test]
fn hal_add_state() {
    let hal = Resource::new()
        .add_state("currentlyProcessing", 14i64)
        .add_state("currency", "USD")
        .add_state("active", true)
        .add_state("errors", ());

    let output = r#"{"active":true,"currency":"USD","currentlyProcessing":14,"errors":null}"#;
    assert_eq!(hal.to_json().to_string(), output.to_string());
}

#[test]
fn hal_spec() {
    let hal = Resource::with_self("/orders")
        .add_curie("ea", "http://example.com/docs/rels/{rel}")
        .add_link("next", Link::new("/orders?page=2"))
        .add_link("ea:find", Link::new("/orders{?id}").templated(true))
        .add_link("ea:admin", Link::new("/admins/2").title("Fred"))
        .add_link("ea:admin", Link::new("/admins/5").title("Kate"))
        .add_state("currentlyProcessing", 14i64)
        .add_state("shippedToday", 14i64)
        .add_resource("ea:order",
            Resource::with_self("/orders/123")
                .add_link("ea:basket", Link::new("/baskets/98712"))
                .add_link("ea:customer", Link::new("/customers/7809"))
                .add_state("total", (30.00 as f64)) // fix precision
                .add_state("currency", "USD")
                .add_state("status", "shipped")
        )
        .add_resource("ea:order",
            Resource::with_self("/orders/124")
                .add_link("ea:basket", Link::new("/baskets/97213"))
                .add_link("ea:customer", Link::new("/customers/12369"))
                .add_state("total", (20.00 as f64)) // fix precision
                .add_state("currency", "USD")
                .add_state("status", "processing")
        );

    let output = r#"{"_embedded":{"ea:order":[{"_links":{"ea:basket":{"href":"/baskets/98712"},"ea:customer":{"href":"/customers/7809"},"self":{"href":"/orders/123"}},"currency":"USD","status":"shipped","total":30.0},{"_links":{"ea:basket":{"href":"/baskets/97213"},"ea:customer":{"href":"/customers/12369"},"self":{"href":"/orders/124"}},"currency":"USD","status":"processing","total":20.0}]},"_links":{"curies":[{"href":"http://example.com/docs/rels/{rel}","name":"ea","templated":true}],"ea:admin":[{"href":"/admins/2","title":"Fred"},{"href":"/admins/5","title":"Kate"}],"ea:find":{"href":"/orders{?id}","templated":true},"next":{"href":"/orders?page=2"},"self":{"href":"/orders"}},"currentlyProcessing":14,"shippedToday":14}"#;
    assert_eq!(hal.to_json().to_string(), output.to_string());
}

#[test]
fn order_to_hal() {
    let order = Order { total: 20.00 as f64, currency: "USD".to_string(), status: "processing".to_string() };

    let output = r#"{"_links":{"self":{"href":"https://www.example.com/orders/1"}},"currency":"USD","status":"processing","total":20.0}"#;
    assert_eq!(order.to_hal().to_json().to_string(), output.to_string());
}

#[test]
fn list_to_hal_state() {
    let friends = vec!("Mary", "Timmy", "Sally", "Wally");

    let hal = Resource::with_self("/user/1")
        .add_state("friends", friends);

    let output = r#"{"_links":{"self":{"href":"/user/1"}},"friends":["Mary","Timmy","Sally","Wally"]}"#;
    assert_eq!(hal.to_json().to_string(), output.to_string());
}

#[test]
fn object_to_hal_state() {
    let mut fullname = BTreeMap::new();
    fullname.insert("given".to_string(), "John");
    fullname.insert("family".to_string(), "Doe");

    let hal = Resource::with_self("/user/1")
        .add_state("fullname", fullname);

    let output = r#"{"_links":{"self":{"href":"/user/1"}},"fullname":{"family":"Doe","given":"John"}}"#;
    assert_eq!(hal.to_json().to_string(), output.to_string());

    let mut fullname = HashMap::new();
    fullname.insert("given".to_string(), "John");
    fullname.insert("family".to_string(), "Doe");

    let hal = Resource::with_self("/user/1")
        .add_state("fullname", fullname);

    assert_eq!(hal.to_json().to_string(), output.to_string());
}

#[test]
fn option_to_hal_state() {
    assert_eq!(Some(15i64).to_hal_state(), I64(15));
    assert_eq!(None::<isize>.to_hal_state(), Null);
}
