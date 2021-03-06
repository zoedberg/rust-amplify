// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//     Martin Habovstiak <martin.habovstiak@gmail.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

//! Amplifying Rust language capabilities: multiple generic trait
//! implementations, type wrappers, derive macros.
//!
//! Minimum supported rust compiler version (MSRV): 1.46 (stable channel)

#![recursion_limit = "256"]
#![deny(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    unused_mut,
    unused_imports,
    dead_code,
    missing_docs,
    warnings
)]

#[cfg(feature = "std")]
#[macro_use]
extern crate amplify_derive;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde_crate as serde;
#[cfg(feature = "stringly_conversions")]
#[cfg_attr(feature = "std", macro_use)]
pub extern crate stringly_conversions;
#[cfg(feature = "stringly_conversions")]
pub use stringly_conversions::*;

#[macro_use]
mod macros;
#[macro_use]
mod wrapper;

mod as_any;
mod bipolar;
mod dumb_default;
#[cfg(feature = "std")]
pub mod internet;
pub mod strategy;
#[cfg(feature = "serde")]
mod to_serde_string;

pub use crate::as_any::AsAny;
pub use crate::bipolar::Bipolar;
pub use crate::strategy::Holder;
pub use crate::wrapper::Wrapper;
pub use crate::dumb_default::DumbDefault;
#[cfg(feature = "serde")]
pub use crate::to_serde_string::{ToYamlString, ToJsonString, ToTomlString};
