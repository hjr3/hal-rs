//! A pure rust library for generating Hal responses.
//!
//! Example:
//!
//! ```
//! extern crate hal;
//! extern crate rustc_serialize as serialize;
//!
//! use hal::ToHal;
//! use hal::link::Link;
//! use hal::resource::Resource;
//! use hal::state::ToHalState;
//! use serialize::json::ToJson;
//!
//! struct Order {
//!     total: f64,
//!     currency: String,
//!     status: String
//! }
//!
//! impl ToHal for Order {
//!     fn to_hal(self) -> Resource {
//!         let mut resource = Resource::with_self("https://www.example.com/orders/1");
//!         resource.add_state("total", self.total)
//!             .add_state("currency", self.currency)
//!             .add_state("status", self.status);
//!         resource
//!     }
//! }
//!
//! fn main() {
//!     let mut hal = Resource::with_self("/orders");
//!     hal.add_curie("ea", "http://example.com/docs/rels/{rel}")
//!         .add_link("next", &Link::new("/orders?page=2"))
//!         .add_link("ea:find", &Link::new("/orders{?id}").templated(true))
//!         .add_link("ea:admin", &Link::new("/admins/2").title("Fred"))
//!         .add_link("ea:admin", &Link::new("/admins/5").title("Kate"))
//!         .add_state("currentlyProcessing", 14 as i64)
//!         .add_state("shippedToday", 14 as i64)
//!         .add_resource("ea:order",
//!             Resource::with_self("/orders/123")
//!                 .add_link("ea:basket", &Link::new("/baskets/98712"))
//!                 .add_link("ea:customer", &Link::new("/customers/7809"))
//!                 .add_state("total", (30.00 as f64))
//!                 .add_state("currency", "USD")
//!                 .add_state("status", "shipped")
//!         )
//!         .add_resource("ea:order",
//!             Resource::with_self("/orders/124")
//!                 .add_link("ea:basket", &Link::new("/baskets/97213"))
//!                 .add_link("ea:customer", &Link::new("/customers/12369"))
//!                 .add_state("total", (20.00 as f64))
//!                 .add_state("currency", "USD")
//!                 .add_state("status", "processing")
//!         );
//!
//!     println!("Creating Hal using a DSL: {}", hal.to_json().to_string());
//!
//!     let order = Order { total: 20.00 as f64,
//!                         currency: "USD".to_string(),
//!                         status: "processing".to_string() };
//!
//!     println!("Creating Hal using to_hal(): {}", order.to_hal().to_json().pretty().to_string());
//! }
//! ```

extern crate rustc_serialize as serialize;

pub mod state;
pub mod link;
pub mod resource;

use resource::Resource;

pub trait ToHal {
    fn to_hal(self) -> Resource;
}
