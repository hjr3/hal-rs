use super::{Link, Resource, ToHal, ToHalState};
use serialize::json::ToJson;

struct Order {
    total: f64,
    currency: StrBuf,
    status: StrBuf
}

impl ToHal for Order {
    fn to_hal(&self) -> Resource {
        Resource::with_self("https://www.example.com/orders/1")
            .add_state("total", self.total.to_hal_state())
            .add_state("currency", self.currency.to_hal_state())
            .add_state("status", self.status.to_hal_state())
    }
}

#[test]
fn link_new() {
    let link = Link::new("https://www.example.com");

    let output = r#"{"href":"https://www.example.com"}"#;
    assert_eq!(link.to_json().to_str(), output.to_owned());
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
    assert_eq!(link.to_json().to_str(), output.to_owned());
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
    assert_eq!(hal.to_json().to_str(), output.to_owned());
}

#[test]
fn hal_with_self() {
    let hal = Resource::with_self("https://www.example.com");

    let output = r#"{"_links":{"self":{"href":"https://www.example.com"}}}"#;
    assert_eq!(hal.to_json().to_str(), output.to_owned());
}

#[test]
fn hal_with_self_and_link() {
    let output = r#"{"_links":{"orders":{"href":"https://www.example.com/orders"},"self":{"href":"https://www.example.com"}}}"#;
    let hal = Resource::with_self("https://www.example.com")
        .add_link("orders", Link::new("https://www.example.com/orders"));
    assert_eq!(hal.to_json().to_str(), output.to_owned());
}

#[test]
fn hal_with_self_and_two_links() {
    let hal = Resource::with_self("https://www.example.com")
        .add_link("orders", Link::new("https://www.example.com/orders/1"))
        .add_link("orders", Link::new("https://www.example.com/orders/2"));

    let output = r#"{"_links":{"orders":[{"href":"https://www.example.com/orders/1"},{"href":"https://www.example.com/orders/2"}],"self":{"href":"https://www.example.com"}}}"#;
    assert_eq!(hal.to_json().to_str(), output.to_owned());
}

#[test]
fn hal_and_add_curie() {
    let hal = Resource::with_self("https://www.example.com")
        .add_curie("ea", "http://example.com/docs/rels/{rel}");


    let output = r#"{"_links":{"curies":[{"href":"http://example.com/docs/rels/{rel}","name":"ea","templated":true}],"self":{"href":"https://www.example.com"}}}"#;
    assert_eq!(hal.to_json().to_str(), output.to_owned());
}

#[test]
fn hal_add_state() {
    let hal = Resource::new()
        .add_state("currentlyProcessing", (14 as int).to_hal_state())
        .add_state("currency", "USD".to_owned().to_hal_state())
        .add_state("active", true.to_hal_state())
        .add_state("errors", ().to_hal_state());

    let output = r#"{"active":true,"currency":"USD","currentlyProcessing":14,"errors":null}"#;
    assert_eq!(hal.to_json().to_str(), output.to_owned());
}

#[test]
fn hal_spec() {
    let hal = Resource::with_self("/orders")
        .add_curie("ea", "http://example.com/docs/rels/{rel}")
        .add_link("next", Link::new("/orders?page=2"))
        .add_link("ea:find", Link::new("/orders{?id}").templated(true))
        .add_link("ea:admin", Link::new("/admins/2").title("Fred"))
        .add_link("ea:admin", Link::new("/admins/5").title("Kate"))
        .add_state("currentlyProcessing", (14 as int).to_hal_state())
        .add_state("shippedToday", (14 as int).to_hal_state())
        .add_resource("ea:order",
            Resource::with_self("/orders/123")
                .add_link("ea:basket", Link::new("/baskets/98712"))
                .add_link("ea:customer", Link::new("/customers/7809"))
                .add_state("total", (30.00 as f64).to_hal_state()) // fix precision
                .add_state("currency", "USD".to_owned().to_hal_state())
                .add_state("status", "shipped".to_owned().to_hal_state())
        )
        .add_resource("ea:order",
            Resource::with_self("/orders/124")
                .add_link("ea:basket", Link::new("/baskets/97213"))
                .add_link("ea:customer", Link::new("/customers/12369"))
                .add_state("total", (20.00 as f64).to_hal_state()) // fix precision
                .add_state("currency", "USD".to_owned().to_hal_state())
                .add_state("status", "processing".to_owned().to_hal_state())
        );

    let output = r#"{"_embedded":{"ea:order":[{"_links":{"ea:basket":{"href":"/baskets/98712"},"ea:customer":{"href":"/customers/7809"},"self":{"href":"/orders/123"}},"currency":"USD","status":"shipped","total":30},{"_links":{"ea:basket":{"href":"/baskets/97213"},"ea:customer":{"href":"/customers/12369"},"self":{"href":"/orders/124"}},"currency":"USD","status":"processing","total":20}]},"_links":{"curies":[{"href":"http://example.com/docs/rels/{rel}","name":"ea","templated":true}],"ea:admin":[{"href":"/admins/2","title":"Fred"},{"href":"/admins/5","title":"Kate"}],"ea:find":{"href":"/orders{?id}","templated":true},"next":{"href":"/orders?page=2"},"self":{"href":"/orders"}},"currentlyProcessing":14,"shippedToday":14}"#;
    assert_eq!(hal.to_json().to_str(), output.to_owned());
}

#[test]
fn order_to_hal() {
    let order = Order { total: 20.00 as f64, currency: StrBuf::from_str("USD"), status: StrBuf::from_str("processing") };

    let output = r#"{"_links":{"self":{"href":"https://www.example.com/orders/1"}},"currency":"USD","status":"processing","total":20}"#;
    assert_eq!(order.to_hal().to_json().to_str(), output.to_owned());
}

#[test]
fn list_to_hal_state() {
    let friends = vec!("Mary", "Timmy", "Sally", "Wally");

    let hal = Resource::with_self("/user/1")
        .add_state("friends", friends.to_hal_state());

    let output = r#"{"_links":{"self":{"href":"/user/1"}},"friends":["Mary","Timmy","Sally","Wally"]}"#;
    assert_eq!(hal.to_json().to_str(), output.to_owned());
}
