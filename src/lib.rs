#![feature(associated_type_defaults)]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

mod convert;
mod types;
mod utils;

#[cfg(feature = "ahash")]
type RandomState = ahash::RandomState;
#[cfg(not(feature = "ahash"))]
type RandomState = std::collections::hash_map::RandomState;
#[cfg(feature = "indexmap")]
pub type Map<K, V> = indexmap::IndexMap<K, V, RandomState>;
#[cfg(not(feature = "indexmap"))]
pub type Map<K, V> = std::collections::HashMap<K, V, RandomState>;

pub use crate::convert::*;
pub use crate::types::*;
pub use crate::utils::*;

#[cfg(test)]
mod tests {
}
