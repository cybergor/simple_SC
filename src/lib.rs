#![cfg_attr(not(feature = "std"), no_std)]

mod contract;
mod events;
mod access;

pub use contract::MyToken;
