#![feature(associated_type_defaults)]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

#![cfg_attr(not(feature = "std"), no_std)]

mod types;

pub use crate::types::*;
pub use crate::ser::*;

#[cfg(test)]
mod tests {
}
