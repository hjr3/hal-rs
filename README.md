# hal-rs

A pure rust library for generating Hal responses.

[![Build Status](https://travis-ci.org/hjr3/hal-rs.svg)](https://travis-ci.org/hjr3/hal-rs)

## Builds

Running the tests:

```
$ cargo test
```

## Examples

See [hal-rs-demo](https://github.com/hjr3/hal-rs-demo) for actual working example.

A sample program that shows how to create a Hal response manually or by implementing `ToHal` on your struct.

```rust
extern crate hal;
extern crate serialize;

use hal::{Link, Resource, ToHal, ToHalData};
use serialize::json::ToJson;

struct Order {
    total: f64,
    currency: String,
    status: String
}

impl ToHal for Order {
    fn to_hal(&self) -> Resource {
        Resource::with_self("https://www.example.com/orders/1")
            .add_state("total", self.total.to_hal_state())
            .add_state("currency", self.currency.to_hal_state())
            .add_state("status", self.status.to_hal_state())
    }
}

fn main() {
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
                .add_state("total", (30.00 as f64).to_hal_state())
                .add_state("currency", "USD".to_hal_state())
                .add_state("status", "shipped".to_hal_state())
        )
        .add_resource("ea:order",
            Resource::with_self("/orders/124")
                .add_link("ea:basket", Link::new("/baskets/97213"))
                .add_link("ea:customer", Link::new("/customers/12369"))
                .add_state("total", (20.00 as f64).to_hal_state())
                .add_state("currency", "USD".to_hal_state())
                .add_state("status", "processing".to_hal_state())
        );

    println!("Creating Hal using a DSL: {}", hal.to_json().to_pretty_str());

    let order = Order { total: 20.00 as f64, 
                        currency: String::from_str("USD"), 
                        status: String::from_str("processing") };

    println!("Creating Hal using to_hal(): {}", order.to_hal().to_json().to_pretty_str());
}
```
