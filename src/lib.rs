/*!
A pure rust library for generating Hal responses.
 */

extern crate rustc_serialize as serialize;

pub mod state;
pub mod link;
pub mod resource;

use resource::Resource;

pub trait ToHal {
    fn to_hal(self) -> Resource;
}
